//! Utilities used for testing with the system domain.
#![warn(missing_docs)]
use crate::{construct_extrinsic_generic, node_config};
use domain_client_executor::ExecutorStreams;
use domain_runtime_primitives::{AccountId, Balance};
use domain_service::{DomainConfiguration, FullPool};
use domain_test_primitives::OnchainStateApi;
use frame_system_rpc_runtime_api::AccountNonceApi;
use sc_client_api::{BlockchainEvents, HeaderBackend};
use sc_network::{NetworkService, NetworkStateInfo};
use sc_network_sync::SyncingService;
use sc_service::config::MultiaddrWithPeerId;
use sc_service::{
    BasePath, Configuration as ServiceConfiguration, Role, RpcHandlers, TFullBackend, TFullClient,
    TaskManager,
};
use sc_utils::mpsc::TracingUnboundedSender;
use sp_api::ProvideRuntimeApi;
use sp_core::H256;
use sp_domains::DomainId;
use sp_keyring::Sr25519Keyring;
use sp_messenger::messages::ChannelId;
use sp_runtime::OpaqueExtrinsic;
use std::future::Future;
use std::sync::Arc;
use subspace_runtime_primitives::opaque::Block as PBlock;
use subspace_test_service::MockPrimaryNode;
use substrate_test_client::{
    BlockchainEventsExt, RpcHandlersExt, RpcTransactionError, RpcTransactionOutput,
};
use system_domain_test_runtime;
use system_domain_test_runtime::opaque::Block;

/// The backend type used by the test service.
pub type Backend = TFullBackend<Block>;

/// The system domain client type being used by the test service.
pub type SClient = TFullClient<
    Block,
    system_domain_test_runtime::RuntimeApi,
    sc_executor::NativeElseWasmExecutor<SystemDomainExecutorDispatch>,
>;

/// System domain code executor for the test service.
pub type SystemCodeExecutor = sc_executor::NativeElseWasmExecutor<SystemDomainExecutorDispatch>;

/// System domain executor for the test service.
pub type SystemExecutor = domain_client_executor::SystemExecutor<
    Block,
    PBlock,
    SClient,
    subspace_test_client::Client,
    FullPool<
        PBlock,
        subspace_test_client::Client,
        system_domain_test_runtime::RuntimeApi,
        SystemDomainExecutorDispatch,
    >,
    Backend,
    SystemCodeExecutor,
>;

type SystemGossipMessageValidator = domain_client_executor::SystemGossipMessageValidator<
    Block,
    PBlock,
    SClient,
    subspace_test_client::Client,
    FullPool<
        PBlock,
        subspace_test_client::Client,
        system_domain_test_runtime::RuntimeApi,
        SystemDomainExecutorDispatch,
    >,
    Backend,
    SystemCodeExecutor,
    domain_client_executor::SystemDomainParentChain<Block, PBlock, subspace_test_client::Client>,
>;

/// The System domain native executor instance.
pub struct SystemDomainExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for SystemDomainExecutorDispatch {
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        system_domain_test_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        system_domain_test_runtime::native_version()
    }
}

/// Start an executor with the given system domain `Configuration` and the mock primary node.
#[sc_tracing::logging::prefix_logs_with(system_domain_config.network.node_name.as_str())]
async fn run_executor_with_mock_primary_node(
    role: Role,
    system_domain_config: ServiceConfiguration,
    mock_primary_node: &mut MockPrimaryNode,
    maybe_relayer_id: Option<AccountId>,
) -> sc_service::error::Result<(
    TaskManager,
    Arc<SClient>,
    Arc<Backend>,
    Arc<SystemCodeExecutor>,
    Arc<NetworkService<Block, H256>>,
    Arc<SyncingService<Block>>,
    RpcHandlers,
    SystemExecutor,
    SystemGossipMessageValidator,
    TracingUnboundedSender<Vec<u8>>,
)> {
    let system_domain_config = DomainConfiguration {
        service_config: system_domain_config,
        maybe_relayer_id,
    };
    let executor_streams = ExecutorStreams {
        // Set `primary_block_import_throttling_buffer_size` to 0 to ensure the primary chain will not be
        // ahead of the execution chain by more than one block, thus slot will not be skipped in test.
        primary_block_import_throttling_buffer_size: 0,
        block_importing_notification_stream: mock_primary_node
            .block_importing_notification_stream(),
        imported_block_notification_stream: mock_primary_node
            .client
            .every_import_notification_stream(),
        new_slot_notification_stream: mock_primary_node.new_slot_notification_stream(),
        _phantom: Default::default(),
    };
    let gossip_msg_sink = mock_primary_node
        .xdm_gossip_worker_builder()
        .gossip_msg_sink();
    let system_domain_node = domain_service::new_full_system::<
        _,
        _,
        _,
        _,
        _,
        _,
        system_domain_test_runtime::RuntimeApi,
        SystemDomainExecutorDispatch,
    >(
        system_domain_config,
        mock_primary_node.client.clone(),
        mock_primary_node.sync_service.clone(),
        &mock_primary_node.select_chain,
        executor_streams,
        gossip_msg_sink,
    )
    .await?;

    let domain_service::NewFullSystem {
        task_manager,
        client,
        backend,
        code_executor,
        network_service,
        sync_service,
        network_starter,
        rpc_handlers,
        executor,
        gossip_message_validator,
        tx_pool_sink,
    } = system_domain_node;

    if role.is_authority() {
        mock_primary_node
            .xdm_gossip_worker_builder()
            .push_domain_tx_pool_sink(DomainId::SYSTEM, tx_pool_sink.clone());
    }

    network_starter.start_network();

    Ok((
        task_manager,
        client,
        backend,
        code_executor,
        network_service,
        sync_service,
        rpc_handlers,
        executor,
        gossip_message_validator,
        tx_pool_sink,
    ))
}

/// A Cumulus test node instance used for testing.
pub struct SystemDomainNode {
    /// The node's key
    pub key: Sr25519Keyring,
    /// TaskManager's instance.
    pub task_manager: TaskManager,
    /// Client's instance.
    pub client: Arc<SClient>,
    /// Client backend.
    pub backend: Arc<Backend>,
    /// Code executor.
    pub code_executor: Arc<SystemCodeExecutor>,
    /// Network service.
    pub network_service: Arc<NetworkService<Block, H256>>,
    /// Sync service.
    pub sync_service: Arc<SyncingService<Block>>,
    /// The `MultiaddrWithPeerId` to this node. This is useful if you want to pass it as "boot node"
    /// to other nodes.
    pub addr: MultiaddrWithPeerId,
    /// RPCHandlers to make RPC queries.
    pub rpc_handlers: RpcHandlers,
    /// System domain executor.
    pub executor: SystemExecutor,
    /// System domain gossip message validator.
    pub gossip_message_validator: SystemGossipMessageValidator,
    /// Sink to the node's tx pool
    pub tx_pool_sink: TracingUnboundedSender<Vec<u8>>,
}

/// A builder to create a [`SystemDomainNode`].
pub struct SystemDomainNodeBuilder {
    tokio_handle: tokio::runtime::Handle,
    key: Sr25519Keyring,
    system_domain_nodes: Vec<MultiaddrWithPeerId>,
    system_domain_nodes_exclusive: bool,
    base_path: BasePath,
    run_relayer: bool,
}

impl SystemDomainNodeBuilder {
    /// Create a new instance of `Self`.
    ///
    /// `tokio_handle` - The tokio handler to use.
    /// `key` - The key that will be used to generate the name.
    /// `base_path` - Where databases will be stored.
    pub fn new(
        tokio_handle: tokio::runtime::Handle,
        key: Sr25519Keyring,
        base_path: BasePath,
    ) -> Self {
        SystemDomainNodeBuilder {
            key,
            tokio_handle,
            system_domain_nodes: Vec::new(),
            system_domain_nodes_exclusive: false,
            base_path,
            run_relayer: false,
        }
    }

    /// Run relayer with the node account id as the relayer id
    pub fn run_relayer(mut self) -> Self {
        self.run_relayer = true;
        self
    }

    /// Instruct the node to exclusively connect to registered parachain nodes.
    ///
    /// System domain nodes can be registered using [`Self::connect_to_system_domain_node`] and
    /// [`Self::connect_to_system_domain_nodes`].
    pub fn exclusively_connect_to_registered_parachain_nodes(mut self) -> Self {
        self.system_domain_nodes_exclusive = true;
        self
    }

    /// Make the node connect to the given system domain node.
    ///
    /// By default the node will not be connected to any node or will be able to discover any other
    /// node.
    pub fn connect_to_system_domain_node(mut self, node: &SystemDomainNode) -> Self {
        self.system_domain_nodes.push(node.addr.clone());
        self
    }

    /// Make the node connect to the given system domain nodes.
    ///
    /// By default the node will not be connected to any node or will be able to discover any other
    /// node.
    pub fn connect_to_system_domain_nodes<'a>(
        mut self,
        nodes: impl Iterator<Item = &'a SystemDomainNode>,
    ) -> Self {
        self.system_domain_nodes
            .extend(nodes.map(|n| n.addr.clone()));
        self
    }

    /// Build the [`SystemDomainNode`] with `MockPrimaryNode` as the embedded primary node.
    pub async fn build_with_mock_primary_node(
        self,
        role: Role,
        mock_primary_node: &mut MockPrimaryNode,
    ) -> SystemDomainNode {
        let system_domain_config = node_config(
            DomainId::SYSTEM,
            self.tokio_handle.clone(),
            self.key,
            self.system_domain_nodes,
            self.system_domain_nodes_exclusive,
            role.clone(),
            BasePath::new(self.base_path.path().join("system")),
        )
        .expect("could not generate system domain node Configuration");

        let multiaddr = system_domain_config.network.listen_addresses[0].clone();

        let maybe_relayer_id = if self.run_relayer {
            Some(self.key.into())
        } else {
            None
        };
        let (
            task_manager,
            client,
            backend,
            code_executor,
            network_service,
            sync_service,
            rpc_handlers,
            executor,
            gossip_message_validator,
            tx_pool_sink,
        ) = run_executor_with_mock_primary_node(
            role,
            system_domain_config,
            mock_primary_node,
            maybe_relayer_id,
        )
        .await
        .expect("could not start system domain node");

        let peer_id = network_service.local_peer_id();
        let addr = MultiaddrWithPeerId { multiaddr, peer_id };

        SystemDomainNode {
            key: self.key,
            task_manager,
            client,
            backend,
            code_executor,
            network_service,
            sync_service,
            addr,
            rpc_handlers,
            executor,
            gossip_message_validator,
            tx_pool_sink,
        }
    }
}

impl SystemDomainNode {
    /// Wait for `count` blocks to be imported in the node and then exit. This function will not
    /// return if no blocks are ever created, thus you should restrict the maximum amount of time of
    /// the test execution.
    pub fn wait_for_blocks(&self, count: usize) -> impl Future<Output = ()> {
        self.client.wait_for_blocks(count)
    }

    /// Get the nonce of the node account
    pub fn account_nonce(&self) -> u32 {
        self.client
            .runtime_api()
            .account_nonce(self.client.info().best_hash, self.key.into())
            .expect("Fail to get account nonce")
    }

    /// Get the free balance of the given account
    pub fn free_balance(&self, account_id: AccountId) -> Balance {
        self.client
            .runtime_api()
            .free_balance(self.client.info().best_hash, account_id)
            .expect("Fail to get account free balance")
    }

    /// Get the last open channel of the given domain
    pub fn get_open_channel_for_domain(&self, dst_domain_id: DomainId) -> Option<ChannelId> {
        self.client
            .runtime_api()
            .get_open_channel_for_domain(self.client.info().best_hash, dst_domain_id)
            .expect("Fail to get open channel")
    }

    /// Construct an extrinsic with the current nonce of the node account and send it to this node.
    pub async fn construct_and_send_extrinsic(
        &mut self,
        function: impl Into<system_domain_test_runtime::RuntimeCall>,
    ) -> Result<RpcTransactionOutput, RpcTransactionError> {
        let extrinsic = construct_extrinsic_generic::<system_domain_test_runtime::Runtime, _>(
            &self.client,
            function,
            self.key,
            false,
            self.account_nonce(),
        );
        self.rpc_handlers.send_transaction(extrinsic.into()).await
    }

    /// Construct an extrinsic.
    pub fn construct_extrinsic(
        &mut self,
        nonce: u32,
        function: impl Into<system_domain_test_runtime::RuntimeCall>,
    ) -> system_domain_test_runtime::UncheckedExtrinsic {
        crate::construct_extrinsic_generic::<system_domain_test_runtime::Runtime, _>(
            &self.client,
            function,
            self.key,
            false,
            nonce,
        )
    }

    /// Construct an extrinsic.
    pub fn construct_extrinsic_with_caller(
        &mut self,
        caller: Sr25519Keyring,
        function: impl Into<system_domain_test_runtime::RuntimeCall>,
    ) -> system_domain_test_runtime::UncheckedExtrinsic {
        let nonce = self
            .client
            .runtime_api()
            .account_nonce(self.client.info().best_hash, caller.into())
            .expect("Fail to get account nonce");
        crate::construct_extrinsic_generic::<system_domain_test_runtime::Runtime, _>(
            &self.client,
            function,
            caller,
            false,
            nonce,
        )
    }

    /// Send an extrinsic to this node.
    pub async fn send_extrinsic(
        &self,
        extrinsic: impl Into<OpaqueExtrinsic>,
    ) -> Result<RpcTransactionOutput, RpcTransactionError> {
        self.rpc_handlers.send_transaction(extrinsic.into()).await
    }
}
