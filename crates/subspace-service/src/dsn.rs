pub mod import_blocks;
pub mod node_provider_storage;

use crate::dsn::node_provider_storage::NodeProviderStorage;
use crate::piece_cache::PieceCache;
use crate::SegmentHeaderCache;
use either::Either;
use sc_client_api::AuxStore;
use sc_consensus_subspace_rpc::SegmentHeaderProvider;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Instant;
use subspace_core_primitives::{SegmentHeader, SegmentIndex};
use subspace_networking::libp2p::kad::ProviderRecord;
use subspace_networking::libp2p::{identity, Multiaddr};
use subspace_networking::{
    peer_id, BootstrappedNetworkingParameters, CreationError, MemoryProviderStorage,
    NetworkParametersPersistenceError, NetworkingParametersManager, Node, NodeRunner,
    ParityDbError, ParityDbProviderStorage, PieceAnnouncementRequestHandler,
    PieceAnnouncementResponse, PieceByHashRequestHandler, PieceByHashResponse, ProviderStorage,
    SegmentHeaderBySegmentIndexesRequestHandler, SegmentHeaderRequest, SegmentHeaderResponse,
    KADEMLIA_PROVIDER_TTL_IN_SECS,
};
use thiserror::Error;
use tracing::{debug, error, trace};

/// Provider records cache size
const MAX_PROVIDER_RECORDS_LIMIT: usize = 100000; // ~ 10 MB

const ROOT_BLOCK_NUMBER_LIMIT: u64 = 100;

/// Errors that might happen during DSN configuration.
#[derive(Debug, Error)]
pub enum DsnConfigurationError {
    /// Can't instantiate the DSN.
    #[error("Can't instantiate the DSN: {0}")]
    CreationError(#[from] CreationError),
    /// ParityDb storage error
    #[error("ParityDb storage error: {0}")]
    ParityDbStorageError(#[from] ParityDbError),
    /// Network parameter manager error.
    #[error("Network parameter manager error: {0}")]
    NetworkParameterManagerError(#[from] NetworkParametersPersistenceError),
}

/// DSN configuration parameters.
#[derive(Clone, Debug)]
pub struct DsnConfig {
    /// Where local DSN node will listen for incoming connections.
    pub listen_on: Vec<Multiaddr>,

    /// Bootstrap nodes for DSN.
    pub bootstrap_nodes: Vec<Multiaddr>,

    /// Reserved nodes for DSN.
    pub reserved_peers: Vec<Multiaddr>,

    /// Identity keypair of a node used for authenticated connections.
    pub keypair: identity::Keypair,

    /// Determines whether we allow keeping non-global (private, shared, loopback..) addresses in Kademlia DHT.
    pub allow_non_global_addresses_in_dht: bool,

    /// System base path.
    pub base_path: Option<PathBuf>,

    /// Defines max established incoming swarm connection limit.
    pub max_in_connections: u32,

    /// Defines max established outgoing swarm connection limit.
    pub max_out_connections: u32,

    /// Defines max pending incoming swarm connection limit.
    pub max_pending_in_connections: u32,

    /// Defines max pending outgoing swarm connection limit.
    pub max_pending_out_connections: u32,

    /// Defines target total (in and out) connection number for DSN that should be maintained.
    pub target_connections: u32,
}

type DsnProviderStorage<AS> =
    NodeProviderStorage<PieceCache<AS>, Either<ParityDbProviderStorage, MemoryProviderStorage>>;

pub(crate) fn create_dsn_instance<AS>(
    dsn_protocol_version: String,
    dsn_config: DsnConfig,
    piece_cache: PieceCache<AS>,
    segment_header_cache: SegmentHeaderCache<AS>,
) -> Result<(Node, NodeRunner<DsnProviderStorage<AS>>), DsnConfigurationError>
where
    AS: AuxStore + Sync + Send + 'static,
{
    trace!("Subspace networking starting.");

    let peer_id = peer_id(&dsn_config.keypair);

    let external_provider_storage = if let Some(path) = &dsn_config.base_path {
        let db_path = path.join("storage_providers_db");

        let cache_size: NonZeroUsize = NonZeroUsize::new(MAX_PROVIDER_RECORDS_LIMIT)
            .expect("Manual value should be greater than zero.");

        Either::Left(ParityDbProviderStorage::new(&db_path, cache_size, peer_id)?)
    } else {
        Either::Right(MemoryProviderStorage::new(peer_id))
    };

    let networking_parameters_registry = {
        dsn_config
            .base_path
            .map(|path| {
                let db_path = path.join("known_addresses_db");

                NetworkingParametersManager::new(&db_path, dsn_config.bootstrap_nodes.clone())
                    .map(|manager| manager.boxed())
            })
            .unwrap_or(Ok(BootstrappedNetworkingParameters::new(
                dsn_config.bootstrap_nodes,
            )
            .boxed()))?
    };

    let provider_storage =
        NodeProviderStorage::new(peer_id, piece_cache.clone(), external_provider_storage);
    let keypair = dsn_config.keypair.clone();
    let mut default_networking_config =
        subspace_networking::Config::new(dsn_protocol_version, keypair, provider_storage.clone());

    default_networking_config
        .kademlia
        .set_provider_record_ttl(KADEMLIA_PROVIDER_TTL_IN_SECS);

    let networking_config = subspace_networking::Config {
        keypair: dsn_config.keypair.clone(),
        listen_on: dsn_config.listen_on,
        allow_non_global_addresses_in_dht: dsn_config.allow_non_global_addresses_in_dht,
        networking_parameters_registry,
        request_response_protocols: vec![
            PieceAnnouncementRequestHandler::create({
                move |peer_id, req| {
                    trace!(?req, %peer_id, "Piece announcement request received.");

                    let provider_record = ProviderRecord {
                        provider: peer_id,
                        key: req.piece_index_hash.into(),
                        addresses: req.addresses.clone(),
                        expires: KADEMLIA_PROVIDER_TTL_IN_SECS.map(|ttl| Instant::now() + ttl),
                    };

                    let result = match provider_storage.add_provider(provider_record) {
                        Ok(()) => Some(PieceAnnouncementResponse::Success),
                        Err(error) => {
                            error!(
                                %error,
                                %peer_id,
                                ?req,
                                "Failed to add provider for received key."
                            );

                            None
                        }
                    };

                    async move { result }
                }
            }),
            PieceByHashRequestHandler::create(move |_, req| {
                let result = match piece_cache.get_piece(req.piece_index_hash) {
                    Ok(maybe_piece) => maybe_piece,
                    Err(error) => {
                        error!(piece_index_hash = ?req.piece_index_hash, %error, "Failed to get piece from cache");
                        None
                    }
                };

                async { Some(PieceByHashResponse { piece: result }) }
            }),
            SegmentHeaderBySegmentIndexesRequestHandler::create(move |_, req| {
                let segment_indexes = match req {
                    SegmentHeaderRequest::SegmentIndexes { segment_indexes } => {
                        segment_indexes.clone()
                    }
                    SegmentHeaderRequest::LastSegmentHeaders {
                        segment_header_number,
                    } => {
                        let mut block_limit = *segment_header_number;
                        if *segment_header_number > ROOT_BLOCK_NUMBER_LIMIT {
                            debug!(
                                %segment_header_number,
                                "Segment header number exceeded the limit."
                            );

                            block_limit = ROOT_BLOCK_NUMBER_LIMIT;
                        }

                        let max_segment_index = segment_header_cache.max_segment_index();

                        // several last segment indexes
                        (SegmentIndex::ZERO..=max_segment_index)
                            .rev()
                            .take(block_limit as usize)
                            .collect::<Vec<_>>()
                    }
                };

                let internal_result = segment_indexes
                    .iter()
                    .map(|segment_index| segment_header_cache.get_segment_header(*segment_index))
                    .collect::<Result<Option<Vec<SegmentHeader>>, _>>();

                let result = match internal_result {
                    Ok(Some(segment_headers)) => Some(SegmentHeaderResponse { segment_headers }),
                    Ok(None) => {
                        error!("Segment header collection contained empty segment headers.");

                        None
                    }
                    Err(error) => {
                        error!(%error, "Failed to get segment headers from cache");

                        None
                    }
                };

                async move { result }
            }),
        ],
        max_established_incoming_connections: dsn_config.max_in_connections,
        max_established_outgoing_connections: dsn_config.max_out_connections,
        max_pending_incoming_connections: dsn_config.max_pending_in_connections,
        max_pending_outgoing_connections: dsn_config.max_pending_out_connections,
        target_connections: dsn_config.target_connections,
        reserved_peers: dsn_config.reserved_peers,

        ..default_networking_config
    };

    subspace_networking::create(networking_config).map_err(Into::into)
}
