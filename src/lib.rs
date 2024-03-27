#![cfg_attr(not(feature = "std"), no_std)]
// 'construct_runtime!' does a lot of recursion and requires us to increase the limit to 256.

#![recursion_limit="256"]

// Make the WASM binary available
#[cfg(feature="std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_std::prelude::*;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    ApplyExtrinsicResult, generic, create_runtime_str, impl_opaque_keys, MultiSignature,
    transaction_validity::{TransactionValidity, TransactionSource},
};

use sp_runtime::traits::{
    BlakeTwo256, Block as BlockT, AccountIdLookup, Verify, IdentiyAccount, NumberFor,
};

use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use pallet_grandpa::fg_primitives;
use sp_version::RuntimeVersion;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

// A few exports that help ease life for downstream crates
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_balances::Call as BalancesCall;
pub use sp_runtime::{Permill, Perbill};
pub use frame_support::{
    construct_runtime, parameter_types, StorageValue,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        Weight, IdentityFee,
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RockDbWeight, WEIGHT_PER_SECOND},
    },
};

use pallet_transaction_payment::CurrencyAdapter;

// Import the template pallet.
pub use pallet_template;

// An index to a block
pub type BlockNumber = u32;

// Alias to 512-bit hash when used in the context of a transaction signature on the chain
pub type Signature = MultiSignature;

// Some way of identifying an account on the chain. We intentionally make it equivalent
// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentityAccount>::AccountId;


// The type for looking up accounts, we don't expect more than 4 bill of them
pub type AccountIndex = u32;


// Balance of an account
pub type Balance = u128;

// Index of a transaction in the chain
pub type Index = u32;

// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

// Digest item type
pub type DigestItem = generic::DigestItem<Hash>;

// Opaque types, there are used by the cli to instantiate machinery that don't need to know the specifics of
// the runtime. They can then be made to be agnostic over specific formats of the data like extrinsics
// allowing for them to continue syncing the network through upgrades to even the core data structures.

pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    // Opaque block header type
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

    // Opaque block type
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;

    // Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub aura: Aura,
            pub grandpa: Grandpa,
        }
    }
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("node-template"),
    impl_name: create_runtime_str!("node-template"),
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

// This determines the average expected block time that we are targeting. Blocks will be produced at a
// minimum duration defined by "SLOT_DURATION" that is picked up by 'pallet_timestamp' which is in turn
// picked up by 'pallet_aura' to implement 'fn_slot_duration()'
// change this to adjust the blocktime.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 60;

// The version information used to indentify this runtime when compiled natively
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const Version: RuntimeVersion = VERSION;
    pub const BlockHashCount: BlockNumber = 2400;
    // we allow for 2 seconds of compute with a 6 second average block time.
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
    ::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
    ::max_with_normal_ratio(5*1024*1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime
impl frame_system::Config for Runtime {
    // The basic call filter to use in dispatchable.
    type BaseCallFilter = ();

    // Block & extrinsics weights: base values and limits
    type BlockWeights = BlockWeights;

    // The maximum length of a block (in bytes)
    type BlockLength = BlockLength;

    // The identifier used to distinguish between accounts.
    type AccountId = AccountId;

    // The aggregated dispatch type that is available for extrinsics.
    type Call = Call;

    // The look up mechanism to get account Id from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;

    // The index type for storing how many extrinsic an account has signed.
    type Index = Index;

    // The index type for blocks.
    type BlockNumber = BlockNumber;

    // the type for hashing blocks and tries.
    type Hash = Hash;

    // the hashing algorithm used
    type Hashing = BlakeTwo256;

    // the header type
    type Header = generic::Header<BlockNumber, BlakeTwo256>;

    // The ubiquitous event type.
    type Event = Event;

    // the ubiquitous origin type
    type Origin = Origin;

    // Maximum number of block number to block hash mappings to keep (oldest pruned first)
    type BlockHashCount = BlockHashCount;

    // the weight of database operations that the runtime can invoke
    type DbWeigth = RocksDbWeight;

    // Version of the runtime
    type Version = Version;

    // the type is being generated by 'construct_runtime'
    type PalletInfo = PalletInfo;

    // generate a new account
    type OnNewAccount = ();

    type OnKilledAccount = ();

    type AccountData = pallet_balances::AccountData<Balance>;

    // Weight information for the extrinsic of this pallet
    type SystemWeightInfo = ();

    // This is used as an identifier of the chain. 42 is the generic substrate prefix
    type SS58Prefix = SS58Prefix;
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;

    type KeyOwnerProofSystem = ();

    type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId,)
    >>::IdentificationTuple;

    type HandEquivocation = ();

    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    // A timestamp: milliseconds since the unix epoch
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;

    // the type for recording an account's balance
    type Balance = Balance;

    // the ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balance, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}

// COnfigure the pallet-template in pallets/ template;
impl pallet_template::Config for Runtime {
    type Event = Event;
}

// Create the runtime by composing the FRAME pallets that were previously configured
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Aura: pallet_aura::{Module, Config<T>},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Storage},
        Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},

        // Include the custom logic from the pallet_template in the runtime
        TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
    }
);

// The address format for describing accounts
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

// Block header type as expected by this runtime
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

// Block type as expected by this runtime
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

// A block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

// BlockId type as expected by this runtime
pub type BlockId = generic::BlockId<Block>;

// The SignedExtension to the basic transaction logic
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>
);

// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

// Extrinsic type that has already been checked
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;

// Executive: Handles dispatch to the various modules
pub type Executive = frame_executive::Executive<Runtime, Block, frame_system::ChainContext<Runtime>,
    Runtime, AllModules,>;

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVerison {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            RandomnessCollectiveFlip::random_seed()
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction (
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> u64 {
            Aura::slot_duration()
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(encoded: Vec<u8>,) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<<Block as BlockT>::Hash, NumberFor<Block>,
            >, _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,) -> Option<()> { None }

        fn generate_key_ownership_proof(_set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info (
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransctionPayment::query_fee_details(uxt, len)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benmark<Block> for Runtime {
        fn dispatch_benchmark(config: frame_benchmarking::BenmarkConfig)
        -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Module as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);
            add_benchmark!(params, batches, pallet_template, TemplateModule);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into())}
        }
    }
}