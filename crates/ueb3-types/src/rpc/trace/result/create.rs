use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::Bytes;

// -----------------------------------------------------------------------------
// Trace Result (create)
// -----------------------------------------------------------------------------

/// Trace result for a `create` trace.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceResultCreate {
  /// The address of the created contract.
  pub address: Address,
  /// The raw code of the created contract.
  pub code: Bytes,
  /// The amount of gas used by the contract creation.
  #[serde(with = "ueb3_serde::quantity")]
  pub gas_used: u64,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::TraceResultCreate;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x83ac5e41e3dcb60a0836ad62211063bbc5dd4ff8";
    const DATA: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";

    let a: TraceResultCreate = TraceResultCreate::default();

    let b: TraceResultCreate = TraceResultCreate {
      address: SEND.parse().unwrap(),
      code: DATA.parse().unwrap(),
      gas_used: 0x183,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceResultCreate",
        len: 3,
      },
      Token::Str("address"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("code"),
      Token::Str("0x"),
      Token::Str("gasUsed"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceResultCreate",
        len: 3,
      },
      Token::Str("address"),
      Token::Str(SEND),
      Token::Str("code"),
      Token::Str(DATA),
      Token::Str("gasUsed"),
      Token::Str("0x183"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
