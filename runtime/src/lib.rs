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

