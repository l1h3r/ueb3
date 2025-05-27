use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Trace Action (create)
// -----------------------------------------------------------------------------

/// Trace information for a `create` trace.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceActionCreate {
  /// The address that triggered the contract creation.
  pub from: Address,
  /// The amount of gas provided to the contract creation.
  #[serde(with = "ueb3_serde::quantity")]
  pub gas: u64,
  /// The raw input data provided to the contract creation.
  pub init: Bytes,
  /// The amount of ETH provided to the contract creation.
  pub value: U256,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::TraceActionCreate;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x83ac5e41e3dcb60a0836ad62211063bbc5dd4ff8";
    const DATA: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";

    let a: TraceActionCreate = TraceActionCreate::default();

    let b: TraceActionCreate = TraceActionCreate {
      from: SEND.parse().unwrap(),
      gas: 0x183,
      init: DATA.parse().unwrap(),
      value: "0x183".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceActionCreate",
        len: 4,
      },
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("init"),
      Token::Str("0x"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceActionCreate",
        len: 4,
      },
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("gas"),
      Token::Str("0x183"),
      Token::Str("init"),
      Token::Str(DATA),
      Token::Str("value"),
      Token::Str("0x183"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
