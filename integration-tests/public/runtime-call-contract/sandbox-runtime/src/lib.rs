//! # Contract Caller
//!
//! Demonstrates calling into an `ink!` contract from a pallet.

#![cfg_attr(not(feature = "std"), no_std)]

ink_sandbox::create_sandbox!(ContractCallerSandbox, ContractCallerSandboxRuntime, (), (), {
    ContractCaller: pallet_contract_caller,
});

impl pallet_contract_caller::Config for ContractCallerSandboxRuntime {}
