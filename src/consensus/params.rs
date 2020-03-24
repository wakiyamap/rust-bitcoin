// Rust Bitcoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Consensus parameters
//!
//! This module provides predefined set of parameters for different chains.
//!

use network::constants::Network;
use util::uint::Uint256;

/// Lowest possible difficulty for Mainnet.
const MAX_BITS_BITCOIN: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000fffffffffffu64,
]);
/// Lowest possible difficulty for Testnet.
const MAX_BITS_TESTNET: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000fffffffffffu64,
]);
/// Lowest possible difficulty for Regtest.
const MAX_BITS_REGTEST: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x7fffffffffffffffu64,
]);

#[derive(Debug, Clone)]
/// Parameters that influence chain consensus.
pub struct Params {
    /// Network for which parameters are valid.
    pub network: Network,
    /// Time when BIP16 becomes active.
    pub bip16_time: u32,
    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,
    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,
    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,
    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,
    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,
    /// Proof of work limit value. It contains the lowest possible difficulty.
    pub pow_limit: Uint256,
    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,
    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,
    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,
    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
    /// Block height at which Lyra2REv2 and DGWv3 becomes active.
    pub switch_lyra2rev2_dgwblock: u32,
}

impl Params {
    /// Creates parameters set for the given network.
    pub fn new(network: Network) -> Self {
        match network {
            Network::Bitcoin => Params {
                network: Network::Bitcoin,
                bip16_time: 0, // gensis block
                bip34_height: 0,
                bip65_height: 977759, // ecc773c827a8cde039f6dfcdee2de981b747f58aa1bc4dddcb28e3c857dbc860
                bip66_height: 977759, // ecc773c827a8cde039f6dfcdee2de981b747f58aa1bc4dddcb28e3c857dbc860
                rule_change_activation_threshold: 7560, // 75% of 10080
                miner_confirmation_window: 10080, // 3.5 days / nPowTargetSpacing * 4 * 0.75
                pow_limit: MAX_BITS_BITCOIN,
                pow_target_spacing: 90, // 1.5 minutes(1.5 * 60)
                pow_target_timespan: 95040, // 1.1 days(1.1 * 24 * 60 * 60)
                allow_min_difficulty_blocks: false,
                no_pow_retargeting: false,
                switch_lyra2rev2_dgwblock: 450000,
            },
            Network::Testnet => Params {
                network: Network::Testnet,
                bip16_time: 0, // always enforce P2SH BIP16 on regtest
                bip34_height: 0,
                bip65_height: 100000000, // TODO
                bip66_height: 100000000, // TODO
                rule_change_activation_threshold: 75, // 75%
                miner_confirmation_window: 100,
                pow_limit: MAX_BITS_TESTNET,
                pow_target_spacing: 90, // 1.5 minutes(1.5 * 60)
                pow_target_timespan: 95040, // 1.1 days(1.1 * 24 * 60 * 60)
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
                switch_lyra2rev2_dgwblock: 60,
            },
            Network::Regtest => Params {
                network: Network::Regtest,
                bip16_time: 0,                 // gensis block
                bip34_height: 100000000, // not activated on regtest
                bip65_height: 100000000,
                bip66_height: 100000000, // used only in rpc tests
                rule_change_activation_threshold: 108, // 75%
                miner_confirmation_window: 144,
                pow_limit: MAX_BITS_REGTEST,
                pow_target_spacing: 90, // 1.5 minutes(1.5 * 60)
                pow_target_timespan: 95040, // 1.1 days(1.1 * 24 * 60 * 60)
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: true,
                switch_lyra2rev2_dgwblock: 30,
            },
        }
    }

    /// Calculates the number of blocks between difficulty adjustments.
    pub fn difficulty_adjustment_interval(&self) -> u64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
