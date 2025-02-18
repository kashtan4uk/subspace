
//! Autogenerated weights for pallet_messenger
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_messenger
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./domains/pallets/messenger/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_messenger.
pub trait WeightInfo {
	fn initiate_channel() -> Weight;
	fn close_channel() -> Weight;
	fn do_open_channel() -> Weight;
	fn do_close_channel() -> Weight;
	fn relay_message() -> Weight;
	fn relay_message_response() -> Weight;
	fn join_relayer_set() -> Weight;
	fn exit_relayer_set() -> Weight;
}

/// Weights for pallet_messenger using the Substrate node and recommended hardware.
#[derive(Debug)]
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Messenger NextChannelId (r:1 w:1)
	/// Proof Skipped: Messenger NextChannelId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForOutbox (r:1 w:1)
	/// Proof: Messenger CounterForOutbox (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Outbox (r:1 w:1)
	/// Proof Skipped: Messenger Outbox (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger RelayerMessages (r:1 w:1)
	/// Proof Skipped: Messenger RelayerMessages (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Channels (r:0 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	fn initiate_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `15508`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_000_000, 15508)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForOutbox (r:1 w:1)
	/// Proof: Messenger CounterForOutbox (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Outbox (r:1 w:1)
	/// Proof Skipped: Messenger Outbox (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger RelayerMessages (r:1 w:1)
	/// Proof Skipped: Messenger RelayerMessages (max_values: None, max_size: None, mode: Measured)
	fn close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391`
		//  Estimated: `16809`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(34_000_000, 16809)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	fn do_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3779`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 3779)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	fn do_close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3779`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_000, 3779)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Messenger Inbox (r:1 w:1)
	/// Proof Skipped: Messenger Inbox (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger InboxResponses (r:1 w:1)
	/// Proof Skipped: Messenger InboxResponses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForInboxResponses (r:1 w:1)
	/// Proof: Messenger CounterForInboxResponses (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger RelayerMessages (r:1 w:1)
	/// Proof Skipped: Messenger RelayerMessages (max_values: None, max_size: None, mode: Measured)
	fn relay_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `19279`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(31_000_000, 19279)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: Messenger OutboxResponses (r:1 w:1)
	/// Proof Skipped: Messenger OutboxResponses (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Outbox (r:1 w:1)
	/// Proof Skipped: Messenger Outbox (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForOutbox (r:1 w:1)
	/// Proof: Messenger CounterForOutbox (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn relay_message_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `737`
		//  Estimated: `17930`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 17930)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Messenger RelayersInfo (r:1 w:1)
	/// Proof Skipped: Messenger RelayersInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:1)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	fn join_relayer_set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `9053`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(33_000_000, 9053)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Messenger RelayersInfo (r:1 w:1)
	/// Proof Skipped: Messenger RelayersInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:1)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	fn exit_relayer_set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `925`
		//  Estimated: `12803`
		// Minimum execution time: 37_000_000 picoseconds.
		Weight::from_parts(39_000_000, 12803)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Messenger NextChannelId (r:1 w:1)
	/// Proof Skipped: Messenger NextChannelId (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForOutbox (r:1 w:1)
	/// Proof: Messenger CounterForOutbox (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Outbox (r:1 w:1)
	/// Proof Skipped: Messenger Outbox (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger RelayerMessages (r:1 w:1)
	/// Proof Skipped: Messenger RelayerMessages (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Channels (r:0 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	fn initiate_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `15508`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_000_000, 15508)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForOutbox (r:1 w:1)
	/// Proof: Messenger CounterForOutbox (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Outbox (r:1 w:1)
	/// Proof Skipped: Messenger Outbox (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger RelayerMessages (r:1 w:1)
	/// Proof Skipped: Messenger RelayerMessages (max_values: None, max_size: None, mode: Measured)
	fn close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `391`
		//  Estimated: `16809`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(34_000_000, 16809)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	fn do_open_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3779`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(9_000_000, 3779)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	fn do_close_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `3779`
		// Minimum execution time: 9_000_000 picoseconds.
		Weight::from_parts(10_000_000, 3779)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Messenger Inbox (r:1 w:1)
	/// Proof Skipped: Messenger Inbox (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger InboxResponses (r:1 w:1)
	/// Proof Skipped: Messenger InboxResponses (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForInboxResponses (r:1 w:1)
	/// Proof: Messenger CounterForInboxResponses (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger RelayerMessages (r:1 w:1)
	/// Proof Skipped: Messenger RelayerMessages (max_values: None, max_size: None, mode: Measured)
	fn relay_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `490`
		//  Estimated: `19279`
		// Minimum execution time: 31_000_000 picoseconds.
		Weight::from_parts(31_000_000, 19279)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: Messenger OutboxResponses (r:1 w:1)
	/// Proof Skipped: Messenger OutboxResponses (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger Channels (r:1 w:1)
	/// Proof Skipped: Messenger Channels (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger Outbox (r:1 w:1)
	/// Proof Skipped: Messenger Outbox (max_values: None, max_size: None, mode: Measured)
	/// Storage: Messenger CounterForOutbox (r:1 w:1)
	/// Proof: Messenger CounterForOutbox (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:0)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn relay_message_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `737`
		//  Estimated: `17930`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(29_000_000, 17930)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Messenger RelayersInfo (r:1 w:1)
	/// Proof Skipped: Messenger RelayersInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:1)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	fn join_relayer_set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `255`
		//  Estimated: `9053`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(33_000_000, 9053)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Messenger RelayersInfo (r:1 w:1)
	/// Proof Skipped: Messenger RelayersInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Messenger Relayers (r:1 w:1)
	/// Proof Skipped: Messenger Relayers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Messenger NextRelayerIdx (r:1 w:1)
	/// Proof Skipped: Messenger NextRelayerIdx (max_values: Some(1), max_size: None, mode: Measured)
	fn exit_relayer_set() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `925`
		//  Estimated: `12803`
		// Minimum execution time: 37_000_000 picoseconds.
		Weight::from_parts(39_000_000, 12803)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
