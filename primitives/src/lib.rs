// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Low-level types used throughout the Substrate code.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod credit;
pub mod deeper_node;
pub mod user_privileges;

use codec::Decode;
use scale_info::prelude::vec::Vec;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, TrailingZeroInput, Verify},
    MultiSignature, OpaqueExtrinsic,
};

pub const DPR: u128 = 1_000_000_000_000_000_000;
/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// A timestamp: milliseconds since the unix epoch.
/// `u64` is enough to represent a duration of half a billion years, when the
/// time scale is milliseconds.
pub type Timestamp = u64;

/// Digest item type.
pub type DigestItem = generic::DigestItem;
/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;
/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Interface for verify device signature
pub trait VerifySignatureInterface<AccountId> {
    /// verify device signature
    fn verify_atomos_signature(nonce: u64, signature: Vec<u8>, sender: AccountId) -> bool;
}

impl<AccountId> VerifySignatureInterface<AccountId> for () {
    fn verify_atomos_signature(_nonce: u64, _signature: Vec<u8>, _sender: AccountId) -> bool {
        true
    }
}

pub trait OperationInterface<AccountId, Balance> {
    fn is_single_max_limit(pay_amount: Balance) -> bool;
}

impl<AccountId, Balance> OperationInterface<AccountId, Balance> for () {
    fn is_single_max_limit(_pay_amount: Balance) -> bool {
        true
    }
}

pub trait AccountCreator<AccountId> {
    fn create_account(string: &'static str) -> AccountId;
}
impl<AccountId: Decode> AccountCreator<AccountId> for () {
    fn create_account(string: &'static str) -> AccountId {
        Decode::decode(&mut TrailingZeroInput::new(string.as_bytes())).expect("input too long.")
    }
}
