// Copyright 2018-2020 Origo Foundation.
// This file is part of Origo Network.

// Origo Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Origo Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Origo Network.  If not, see <http://www.gnu.org/licenses/>.

#![warn(missing_docs)]

//! Miner module
//! Keeps track of transactions and mined block.

extern crate ansi_term;
extern crate common_types as types;
extern crate ethabi;
extern crate ethcore_call_contract as call_contract;
extern crate ethereum_types;
extern crate futures;
extern crate heapsize;
extern crate keccak_hash as hash;
extern crate linked_hash_map;
extern crate parity_runtime;
extern crate parking_lot;
#[cfg(feature = "price-info")]
extern crate price_info;
extern crate rlp;
extern crate transaction_pool as txpool;
extern crate zcash_primitives;
extern crate sapling_crypto;
extern crate pairing;
extern crate zcash_proofs;
extern crate bellman;
extern crate ethcore_blockchain;
extern crate rand;
extern crate ff;

#[macro_use]
extern crate ethabi_contract;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg(test)]
extern crate rustc_hex;
#[cfg(test)]
extern crate ethkey;
#[cfg(test)]
extern crate env_logger;

pub mod external;
#[cfg(feature = "price-info")]
pub mod gas_price_calibrator;
pub mod gas_pricer;
pub mod local_accounts;
pub mod pool;
pub mod service_transaction_checker;
#[cfg(feature = "work-notify")]
pub mod work_notify;
