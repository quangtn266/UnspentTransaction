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

    )
>>