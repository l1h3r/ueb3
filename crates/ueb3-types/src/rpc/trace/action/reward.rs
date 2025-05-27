use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Trace Action (reward)
// -----------------------------------------------------------------------------

/// Trace information for a `reward` trace.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceActionReward {
  /// The receiving address of the block reward.
  pub author: Address,
  /// The type indentifier of the block reward.
  pub reward_type: RewardType,
  /// The amount of ETH received as a block reward.
  pub value: U256,
}

// -----------------------------------------------------------------------------
// Reward Type
// -----------------------------------------------------------------------------

/// A reward type identifier.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum RewardType {
  /// `block` reward type.
  #[default]
  Block,
  /// `uncle` reward type.
  Uncle,
  /// `external` reward type.
  External,
  /// `emptyStep` reward type.
  EmptyStep,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::RewardType;
  use crate::rpc::trace::TraceActionReward;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x83ac5e41e3dcb60a0836ad62211063bbc5dd4ff8";

    let a: TraceActionReward = TraceActionReward::default();

    let b: TraceActionReward = TraceActionReward {
      author: SEND.parse().unwrap(),
      reward_type: RewardType::Uncle,
      value: "0x183".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceActionReward",
        len: 3,
      },
      Token::Str("author"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("rewardType"),
      Token::UnitVariant {
        name: "RewardType",
        variant: "block",
      },
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceActionReward",
        len: 3,
      },
      Token::Str("author"),
      Token::Str(SEND),
      Token::Str("rewardType"),
      Token::UnitVariant {
        name: "RewardType",
        variant: "uncle",
      },
      Token::Str("value"),
      Token::Str("0x183"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
