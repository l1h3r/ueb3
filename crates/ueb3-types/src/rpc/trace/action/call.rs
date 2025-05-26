use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Trace Action (call)
// -----------------------------------------------------------------------------

/// Trace information for a `call` trace.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceActionCall {
  /// The address that triggered the call.
  pub from: Address,
  /// The type indentifier of the call.
  pub call_type: CallType,
  /// The amount of gas provided to the call.
  #[serde(with = "ueb3_serde::quantity")]
  pub gas: u64,
  /// The raw input data provided to the call.
  pub input: Bytes,
  /// The address that received the call.
  pub to: Address,
  /// The amount of ETH provided to the call.
  pub value: U256,
}

// -----------------------------------------------------------------------------
// Call Type
// -----------------------------------------------------------------------------

/// A call type identifier.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CallType {
  /// `call` call type.
  #[serde(rename = "call")]
  Call,
  /// `callcode` call type.
  #[serde(rename = "callcode")]
  CallCode,
  /// `delegatecall` call type.
  #[serde(rename = "delegatecall")]
  Delegate,
  /// `staticcall` call type.
  #[serde(rename = "staticcall")]
  Static,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::CallType;
  use crate::rpc::trace::TraceActionCall;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x83ac5e41e3dcb60a0836ad62211063bbc5dd4ff8";
    const DATA: &str = "0x0000000000000000000000000000000000000000000000000000000000000001";

    let a: TraceActionCall = TraceActionCall {
      from: Default::default(),
      call_type: CallType::Call,
      gas: Default::default(),
      input: Default::default(),
      to: Default::default(),
      value: Default::default(),
    };

    let b: TraceActionCall = TraceActionCall {
      from: SEND.parse().unwrap(),
      call_type: CallType::Call,
      gas: 0x183,
      input: DATA.parse().unwrap(),
      to: SEND.parse().unwrap(),
      value: "0x183".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceActionCall",
        len: 6,
      },
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("callType"),
      Token::UnitVariant {
        name: "CallType",
        variant: "call",
      },
      Token::Str("gas"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("to"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceActionCall",
        len: 6,
      },
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("callType"),
      Token::UnitVariant {
        name: "CallType",
        variant: "call",
      },
      Token::Str("gas"),
      Token::Str("0x183"),
      Token::Str("input"),
      Token::Str(DATA),
      Token::Str("to"),
      Token::Str(SEND),
      Token::Str("value"),
      Token::Str("0x183"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
