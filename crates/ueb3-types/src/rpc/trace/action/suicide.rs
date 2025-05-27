use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Trace Action (suicide)
// -----------------------------------------------------------------------------

/// Trace information for a `suicide` trace.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceActionSuicide {
  /// The address of the contract that was destroyed.
  pub address: Address,
  /// The receiving address of the contract destruction.
  pub refund_address: Address,
  /// The balance of the contract prior to destruction.
  pub balance: U256,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::TraceActionSuicide;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x83ac5e41e3dcb60a0836ad62211063bbc5dd4ff8";

    let a: TraceActionSuicide = TraceActionSuicide::default();

    let b: TraceActionSuicide = TraceActionSuicide {
      address: SEND.parse().unwrap(),
      refund_address: SEND.parse().unwrap(),
      balance: "0x183".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceActionSuicide",
        len: 3,
      },
      Token::Str("address"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("refundAddress"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("balance"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceActionSuicide",
        len: 3,
      },
      Token::Str("address"),
      Token::Str(SEND),
      Token::Str("refundAddress"),
      Token::Str(SEND),
      Token::Str("balance"),
      Token::Str("0x183"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
