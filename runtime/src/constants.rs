// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
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

//! A set of constant values used in substrate runtime.

/// Money matters.
pub mod currency {
    use node_primitives::Balance;

    pub const MILLICENTS: Balance = 10_000_000_000_000;
    pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
    pub const DOLLARS: Balance = 100 * CENTS;
    pub const DPR: Balance = DOLLARS;

    // 2 DPR per account for 2158 accounts
    const GENESIS_ACCOUNTS_INITIAL_BALANCE: u128 = 2158 * 2 * DPR;
    // 110 DPR (100 for stash, 10 for controller) per chain validator for 7 validators
    const GENESIS_CHAIN_VALIDATORS_BALANCE: u128 = 7 * 110 * DPR;
    // Deeper Network will transfer 250_000_000 DPR from Ethereum to the bridge on genesis
    // and will transfer more and increase remainder_mining_reward in staking pallet when needed
    pub const TOTAL_MINING_REWARD: u128 =
        250_000_000 * DPR - GENESIS_ACCOUNTS_INITIAL_BALANCE - GENESIS_CHAIN_VALIDATORS_BALANCE;

    pub const MICROPAYMENT_TO_CREDIT_FACTOR: u128 = DPR * 10 / 1024 / 1024; // 1 credit per 10 MB

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
    }
}

/// Time.
pub mod time {
    use node_primitives::{BlockNumber, Moment};

    /// Since BABE is probabilistic this is the average expected block time that
    /// we are targeting. Blocks will be produced at a minimum duration defined
    /// by `SLOT_DURATION`, but some slots will not be allocated to any
    /// authority and hence no block will be produced. We expect to have this
    /// block time on average following the defined slot duration and the value
    /// of `c` configured for BABE (where `1 - c` represents the probability of
    /// a slot being empty).
    /// This value is only used indirectly to define the unit constants below
    /// that are expressed in blocks. The rest of the code should use
    /// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
    /// minimum period).
    ///
    /// If using BABE with secondary slots (default) then all of the slots will
    /// always be assigned, in which case `MILLISECS_PER_BLOCK` and
    /// `SLOT_DURATION` should have the same value.
    ///
    /// <https://research.web3.foundation/en/latest/polkadot/block-production/Babe.html#-6.-practical-results>
    pub const MILLISECS_PER_BLOCK: Moment = 5000;
    pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

    // These time units are defined in number of blocks.
    pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;

    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 240 * MINUTES;

    pub const EPOCH_DURATION_IN_SLOTS: u64 = {
        const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

        (EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
    };

    pub const BLOCKS_PER_ERA: BlockNumber = 6 * EPOCH_DURATION_IN_BLOCKS;

    // 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
    pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}
