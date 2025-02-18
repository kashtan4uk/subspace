use codec::{Decode, Encode};
use sp_api::ProvideRuntimeApi;
use sp_core::traits::FetchRuntimeCode;
use sp_domains::fraud_proof::VerificationError;
use sp_domains::{DomainId, ExecutorApi};
use sp_runtime::traits::Block as BlockT;
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) struct RuntimeCodeFetcher<'a> {
    pub(crate) wasm_bundle: &'a [u8],
}

impl<'a> FetchRuntimeCode for RuntimeCodeFetcher<'a> {
    fn fetch_runtime_code(&self) -> Option<Cow<[u8]>> {
        Some(self.wasm_bundle.into())
    }
}

pub(crate) struct DomainRuntimeCode {
    pub(crate) wasm_bundle: Cow<'static, [u8]>,
}

impl DomainRuntimeCode {
    pub(crate) fn as_runtime_code_fetcher(&self) -> RuntimeCodeFetcher {
        RuntimeCodeFetcher {
            wasm_bundle: &self.wasm_bundle,
        }
    }
}

pub(crate) fn retrieve_domain_runtime_code<PBlock, PClient, Hash>(
    domain_id: DomainId,
    at: PBlock::Hash,
    primary_chain_client: &Arc<PClient>,
) -> Result<DomainRuntimeCode, VerificationError>
where
    PBlock: BlockT,
    Hash: Encode + Decode,
    PClient: ProvideRuntimeApi<PBlock>,
    PClient::Api: ExecutorApi<PBlock, Hash>,
{
    let system_wasm_bundle = primary_chain_client
        .runtime_api()
        .system_domain_wasm_bundle(at)
        .map_err(VerificationError::RuntimeApi)?;

    let wasm_bundle = {
        if domain_id.is_system() {
            system_wasm_bundle
        } else {
            return Err(VerificationError::RuntimeCode(format!(
                "No runtime code for {domain_id:?}"
            )));
        }
    };

    Ok(DomainRuntimeCode { wasm_bundle })
}
