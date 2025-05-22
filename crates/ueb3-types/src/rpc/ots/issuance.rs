use serde::Deserialize;
use serde::Serialize;

use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Issuance
// -----------------------------------------------------------------------------

/// Information about the ETH issued from a block.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Issuance {
  /// The miner reward.
  pub block_reward: Wei,
  /// The total reward issued to uncle blocks.
  pub uncle_reward: Wei,
  /// The total ETH issued in this block (miner + uncle rewards).
  pub issuance: Wei,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::ots::Issuance;

  #[test]
  fn test_serde() {
    let a: Issuance = Issuance::default();

    let b: Issuance = Issuance {
      block_reward: "0x1000000000000".parse().unwrap(),
      uncle_reward: "0x1000000000000".parse().unwrap(),
      issuance: "0x2000000000000".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Issuance",
        len: 3,
      },
      Token::Str("blockReward"),
      Token::Str("0x0"),
      Token::Str("uncleReward"),
      Token::Str("0x0"),
      Token::Str("issuance"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Issuance",
        len: 3,
      },
      Token::Str("blockReward"),
      Token::Str("0x1000000000000"),
      Token::Str("uncleReward"),
      Token::Str("0x1000000000000"),
      Token::Str("issuance"),
      Token::Str("0x2000000000000"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
