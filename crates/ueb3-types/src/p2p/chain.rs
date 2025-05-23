use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Address;

// -----------------------------------------------------------------------------
// Chain Configuration
// -----------------------------------------------------------------------------

/// The core config which determines the blockchain settings.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChainConfig {
  /// Identifies the current chain.
  pub chain_id: u64,
  /// Identifies the current chain.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chain_name: Option<String>,
  /// Identifies the current consensus algorithm.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub consensus: Option<String>,
  /// The homestead switch block (`None` = no fork, 0 = already homestead).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub homestead_block: Option<u64>,
  /// The DAO hard-fork switch block (`None` = no fork)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dao_fork_block: Option<u64>,
  /// Whether the node supports or opposes the DAO hard-fork.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dao_fork_support: Option<bool>,
  /// The [EIP-150] hard-fork block (`None` = no fork).
  ///
  /// [EIP-150]: https://eips.ethereum.org/EIPS/eip-150
  #[serde(skip_serializing_if = "Option::is_none")]
  pub eip_150_block: Option<u64>,
  /// The [EIP-155] hard-fork block (`None` = no fork).
  ///
  /// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
  #[serde(skip_serializing_if = "Option::is_none")]
  pub eip_155_block: Option<u64>,
  /// The [EIP-158] hard-fork block (`None` = no fork).
  ///
  /// [EIP-158]: https://eips.ethereum.org/EIPS/eip-158
  #[serde(skip_serializing_if = "Option::is_none")]
  pub eip_158_block: Option<u64>,
  /// The Byzantium switch block (`None` = no fork, 0 = already on byzantium).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub byzantium_block: Option<u64>,
  /// The Constantinople switch block (`None` = no fork, 0 = already activated).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub constantinople_block: Option<u64>,
  /// The Petersburg switch block (`None` = same as Constantinople).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub petersburg_block: Option<u64>,
  /// The Istanbul switch block (`None` = no fork, 0 = already on istanbul).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub istanbul_block: Option<u64>,
  /// The [EIP-2384] (bomb delay) switch block (`None` = no fork, 0 = already activated).
  ///
  /// [EIP-2384]: https://eips.ethereum.org/EIPS/eip-2384
  #[serde(skip_serializing_if = "Option::is_none")]
  pub muir_glacier_block: Option<u64>,
  /// The Berlin switch block (`None` = no fork, 0 = already on berlin).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub berlin_block: Option<u64>,
  /// The London switch block (`None` = no fork, 0 = already on london).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub london_block: Option<u64>,
  /// The [EIP-4345] (bomb delay) switch block (`None` = no fork, 0 = already activated).
  ///
  /// [EIP-4345]: https://eips.ethereum.org/EIPS/eip-4345
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arrow_glacier_block: Option<u64>,
  /// The [EIP-5133] (bomb delay) switch block (`None` = no fork, 0 = already activated).
  ///
  /// [EIP-5133]: https://eips.ethereum.org/EIPS/eip-5133
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gray_glacier_block: Option<u64>,
  /// Virtual fork after [The Merge] to use as a network splitter.
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[serde(skip_serializing_if = "Option::is_none")]
  pub merge_netsplit_block: Option<u64>,
  /// Shanghai switch time (`None` = no fork, 0 = already on shanghai)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shanghai_time: Option<u64>,
  /// Cancun switch time (`None` = no fork, 0 = already on cancun)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cancun_time: Option<u64>,
  /// Prague switch time (`None` = no fork, 0 = already on prague)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prague_time: Option<u64>,
  /// Osaka switch time (`None` = no fork, 0 = already on verkle)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub osaka_time: Option<u64>,
  /// Verkle switch time (`None` = no fork, 0 = already on verkle)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verkle_time: Option<u64>,
  /// The amount of total difficulty reached by the network that triggers the
  /// consensus upgrade.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub terminal_total_difficulty: Option<u128>,
  /// Specifies that the network already passed the terminal total difficulty.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub terminal_total_difficulty_passed: Option<bool>,
  /// The ETH staking deposit contract address.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deposit_contract_address: Option<Address>,
  /// Ethash consensus engine configuration.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ethash: Option<EthashConfig>,
  /// Clique consensus engine configuration.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub clique: Option<CliqueConfig>,
  // ---------------------------------------------------------------------------
  // EIP-7840
  //
  // https://eips.ethereum.org/EIPS/eip-7840
  // ---------------------------------------------------------------------------
  /// Blob schedule configuration.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blob_schedule: Option<BlobScheduleConfig>,
}

impl Debug for ChainConfig {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("ChainConfig")
      .field("chain_id", &self.chain_id)
      .field("chain_name", &self.chain_name.format())
      .field("consensus", &self.consensus.format())
      .field("homestead_block", &self.homestead_block.format())
      .field("dao_fork_block", &self.dao_fork_block.format())
      .field("dao_fork_support", &self.dao_fork_support.format())
      .field("eip_150_block", &self.eip_150_block.format())
      .field("eip_155_block", &self.eip_155_block.format())
      .field("eip_158_block", &self.eip_158_block.format())
      .field("byzantium_block", &self.byzantium_block.format())
      .field("constantinople_block", &self.constantinople_block.format())
      .field("petersburg_block", &self.petersburg_block.format())
      .field("istanbul_block", &self.istanbul_block.format())
      .field("muir_glacier_block", &self.muir_glacier_block.format())
      .field("berlin_block", &self.berlin_block.format())
      .field("london_block", &self.london_block.format())
      .field("arrow_glacier_block", &self.arrow_glacier_block.format())
      .field("gray_glacier_block", &self.gray_glacier_block.format())
      .field("merge_netsplit_block", &self.merge_netsplit_block.format())
      .field("shanghai_time", &self.shanghai_time.format())
      .field("cancun_time", &self.cancun_time.format())
      .field("prague_time", &self.prague_time.format())
      .field("osaka_time", &self.osaka_time.format())
      .field("verkle_time", &self.verkle_time.format())
      .field("terminal_total_difficulty", &self.terminal_total_difficulty.format())
      .field("terminal_total_difficulty_passed", &self.terminal_total_difficulty_passed.format())
      .field("deposit_contract_address", &self.deposit_contract_address.format())
      .field("ethash", &self.ethash.format())
      .field("clique", &self.clique.format())
      .field("blob_schedule", &self.blob_schedule.format())
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Ethash Configuration
// -----------------------------------------------------------------------------

/// Consensus engine configuration for proof-of-work based sealing.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EthashConfig {}

// -----------------------------------------------------------------------------
// Clique Configuration
// -----------------------------------------------------------------------------

/// Consensus engine configuration for proof-of-authority based sealing
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CliqueConfig {
  /// Number of seconds between blocks to enforce.
  pub period: u64,
  /// Epoch length to reset votes and checkpoint.
  pub epoch: u64,
}

// -----------------------------------------------------------------------------
// Blob Schedule Configuration
// -----------------------------------------------------------------------------

/// Target and max number of blobs allow per fork.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlobScheduleConfig {
  /// Blob configuration at Cancun hard-fork.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cancun: Option<BlobConfig>,
  /// Blob configuration at Prague hard-fork.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prague: Option<BlobConfig>,
  /// Blob configuration at Osaka hard-fork.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub osaka: Option<BlobConfig>,
  /// Blob configuration at Verkle hard-fork.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verkle: Option<BlobConfig>,
}

impl Debug for BlobScheduleConfig {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("BlobScheduleConfig")
      .field("cancun", &self.cancun.format())
      .field("prague", &self.prague.format())
      .field("osaka", &self.osaka.format())
      .field("verkle", &self.verkle.format())
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Blob Configuration
// -----------------------------------------------------------------------------

/// The target and max blobs per block for the associated fork.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BlobConfig {
  /// The target number of blobs per-block.
  pub target: u64,
  /// The maximum number of blobs per-block.
  pub max: u64,
  /// The update fraction used when calculating excess blob gas.
  pub base_fee_update_fraction: u64,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_ser_tokens;

  use crate::p2p::BlobConfig;
  use crate::p2p::BlobScheduleConfig;
  use crate::p2p::ChainConfig;
  use crate::p2p::EthashConfig;

  #[test]
  fn test_serde() {
    const DEPOSIT: &str = "0x00000000219ab540356cbb839cbe05303d7705fa";

    let a: ChainConfig = Default::default();

    let b: ChainConfig = ChainConfig {
      chain_id: 1,
      chain_name: Some("mainnet".to_owned()),
      consensus: Some("ethash".to_owned()),
      homestead_block: Some(1150000),
      dao_fork_block: Some(1920000),
      dao_fork_support: None,
      eip_150_block: Some(2463000),
      eip_155_block: Some(2675000),
      eip_158_block: None,
      byzantium_block: Some(4370000),
      constantinople_block: Some(7280000),
      petersburg_block: Some(7280000),
      istanbul_block: Some(9069000),
      muir_glacier_block: Some(9200000),
      berlin_block: Some(12244000),
      london_block: Some(12965000),
      arrow_glacier_block: Some(13773000),
      gray_glacier_block: Some(15050000),
      merge_netsplit_block: None,
      shanghai_time: Some(1681338455),
      cancun_time: Some(1710338135),
      prague_time: Some(1746612311),
      osaka_time: None,
      verkle_time: None,
      terminal_total_difficulty: Some(u128::from(u64::MAX)),
      terminal_total_difficulty_passed: Some(true),
      deposit_contract_address: Some(DEPOSIT.parse().unwrap()),
      ethash: Some(EthashConfig {}),
      clique: None,
      blob_schedule: Some(BlobScheduleConfig {
        cancun: Some(BlobConfig {
          target: 3,
          max: 6,
          base_fee_update_fraction: 3338477,
        }),
        prague: Some(BlobConfig {
          target: 6,
          max: 9,
          base_fee_update_fraction: 5007716,
        }),
        osaka: None,
        verkle: None,
      }),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "ChainConfig",
        len: 1,
      },
      Token::Str("chainId"),
      Token::U64(0),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "ChainConfig",
        len: 24,
      },
      Token::Str("chainId"),
      Token::U64(1),
      Token::Str("chainName"),
      Token::Some,
      Token::Str("mainnet"),
      Token::Str("consensus"),
      Token::Some,
      Token::Str("ethash"),
      Token::Str("homesteadBlock"),
      Token::Some,
      Token::U64(1150000),
      Token::Str("daoForkBlock"),
      Token::Some,
      Token::U64(1920000),
      Token::Str("eip150Block"),
      Token::Some,
      Token::U64(2463000),
      Token::Str("eip155Block"),
      Token::Some,
      Token::U64(2675000),
      Token::Str("byzantiumBlock"),
      Token::Some,
      Token::U64(4370000),
      Token::Str("constantinopleBlock"),
      Token::Some,
      Token::U64(7280000),
      Token::Str("petersburgBlock"),
      Token::Some,
      Token::U64(7280000),
      Token::Str("istanbulBlock"),
      Token::Some,
      Token::U64(9069000),
      Token::Str("muirGlacierBlock"),
      Token::Some,
      Token::U64(9200000),
      Token::Str("berlinBlock"),
      Token::Some,
      Token::U64(12244000),
      Token::Str("londonBlock"),
      Token::Some,
      Token::U64(12965000),
      Token::Str("arrowGlacierBlock"),
      Token::Some,
      Token::U64(13773000),
      Token::Str("grayGlacierBlock"),
      Token::Some,
      Token::U64(15050000),
      Token::Str("shanghaiTime"),
      Token::Some,
      Token::U64(1681338455),
      Token::Str("cancunTime"),
      Token::Some,
      Token::U64(1710338135),
      Token::Str("pragueTime"),
      Token::Some,
      Token::U64(1746612311),
      Token::Str("terminalTotalDifficulty"),
      Token::Some,
      Token::U64(u64::MAX),
      Token::Str("terminalTotalDifficultyPassed"),
      Token::Some,
      Token::Bool(true),
      Token::Str("depositContractAddress"),
      Token::Some,
      Token::Str(DEPOSIT),
      Token::Str("ethash"),
      Token::Some,
      Token::Struct {
        name: "EthashConfig",
        len: 0,
      },
      Token::StructEnd,
      Token::Str("blobSchedule"),
      Token::Some,
      Token::Struct {
        name: "BlobScheduleConfig",
        len: 2,
      },
      Token::Str("cancun"),
      Token::Some,
      Token::Struct {
        name: "BlobConfig",
        len: 3,
      },
      Token::Str("target"),
      Token::U64(3),
      Token::Str("max"),
      Token::U64(6),
      Token::Str("baseFeeUpdateFraction"),
      Token::U64(3338477),
      Token::StructEnd,
      Token::Str("prague"),
      Token::Some,
      Token::Struct {
        name: "BlobConfig",
        len: 3,
      },
      Token::Str("target"),
      Token::U64(6),
      Token::Str("max"),
      Token::U64(9),
      Token::Str("baseFeeUpdateFraction"),
      Token::U64(5007716),
      Token::StructEnd,
      Token::StructEnd,
      Token::StructEnd,
    ];

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
  }
}
