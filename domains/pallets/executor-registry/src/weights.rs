
//! Autogenerated weights for pallet_executor_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/subspace-node
// executor
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_executor_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./domains/pallets/executor-registry/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_executor_registry.
pub trait WeightInfo {
	fn register() -> Weight;
	fn increase_stake() -> Weight;
	fn decrease_stake() -> Weight;
	fn withdraw_stake() -> Weight;
	fn pause_execution() -> Weight;
	fn resume_execution() -> Weight;
	fn update_public_key() -> Weight;
	fn update_reward_address() -> Weight;
}

/// Weights for pallet_executor_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry CounterForExecutors (r:1 w:1)
	/// Proof: ExecutorRegistry CounterForExecutors (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry KeyOwner (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry KeyOwner (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveExecutors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveExecutors (max_values: Some(1), max_size: None, mode: Measured)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `590`
		//  Estimated: `25620`
		// Minimum execution time: 54_000_000 picoseconds.
		Weight::from_parts(55_000_000, 25620)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	fn increase_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `18075`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_000_000, 18075)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	fn decrease_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `5798`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 5798)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `607`
		//  Estimated: `15943`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 15943)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveExecutors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveExecutors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	fn pause_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `7707`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 7707)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveExecutors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveExecutors (max_values: Some(1), max_size: None, mode: Measured)
	fn resume_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `7707`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(19_000_000, 7707)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: ExecutorRegistry KeyOwner (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry KeyOwner (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry Executors (r:1 w:0)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry NextKey (r:0 w:1)
	/// Proof Skipped: ExecutorRegistry NextKey (max_values: None, max_size: None, mode: Measured)
	fn update_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `8166`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 8166)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	fn update_reward_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `3849`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3849)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry CounterForExecutors (r:1 w:1)
	/// Proof: ExecutorRegistry CounterForExecutors (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry KeyOwner (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry KeyOwner (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveExecutors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveExecutors (max_values: Some(1), max_size: None, mode: Measured)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `590`
		//  Estimated: `25620`
		// Minimum execution time: 54_000_000 picoseconds.
		Weight::from_parts(55_000_000, 25620)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	fn increase_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `627`
		//  Estimated: `18075`
		// Minimum execution time: 41_000_000 picoseconds.
		Weight::from_parts(42_000_000, 18075)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	fn decrease_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `5798`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 5798)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `607`
		//  Estimated: `15943`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 15943)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveExecutors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveExecutors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	fn pause_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `7707`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 7707)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveStake (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveStake (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry TotalActiveExecutors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry TotalActiveExecutors (max_values: Some(1), max_size: None, mode: Measured)
	fn resume_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `424`
		//  Estimated: `7707`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(19_000_000, 7707)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ExecutorRegistry KeyOwner (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry KeyOwner (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry Executors (r:1 w:0)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExecutorRegistry NextKey (r:0 w:1)
	/// Proof Skipped: ExecutorRegistry NextKey (max_values: None, max_size: None, mode: Measured)
	fn update_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `412`
		//  Estimated: `8166`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 8166)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: ExecutorRegistry Executors (r:1 w:1)
	/// Proof Skipped: ExecutorRegistry Executors (max_values: None, max_size: None, mode: Measured)
	fn update_reward_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `3849`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3849)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
