// Service and ServiceFactory implementation. Specialized wrapper over subtrate service.

use std::sync::Arc;
use std::time::Duration;
use sc_client_api::{ExecutorProvider, RemoteBackend};
use node_template_runtime::{self, opaque::Block, RuntimeApi};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sp_inherents::InherentDataProviders;
use sc_executor::NativeExecutor;
pub use sc_executor::NativeExecutor;
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sc_finality_grandpa::SharedVoterState;
use sc_keystore::LocalKeystore;

native_executor_instance!(
    pub Executor,
    node_template_runtime::api::dispatch,
    node_template_runtime::native_version,
    frame_benchmarking::benchmarking::HostFunctions,
);

type FullClient = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn new_partial(config: &Configuration) -> Result<sc_service::PartialComponents<
    FullClient, FullBackend, FullSelectChain,
    sp_consenus::DefaultImportQueue<Block, FullClient>,
    sc_transaction_pool::FullPool<Block, FullClient>,
    (
        sc_consensus_aura::AuraBlockImport<
            Block,
            FullClient,
            sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
            AuraPair
        >,
        sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
    )
>, ServiceError> {
    if config.keystore_remote.is_some() {
        return Err(ServiceError::Other(format!("Remote Keystores are not supported.")))
    }

    let inherent_data_providers = InherentDataProviders::new();
    let (client, backend, keystore_container, task_manager) = sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
    let client = Arc::new(client);
    let select_chain = sc_consensus::LongestChain::new(backend.clone());
    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
    );
    let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
    )?;
    let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, AuraPair>::new(grandpa_block_import.clone(), client.clone(),);

}