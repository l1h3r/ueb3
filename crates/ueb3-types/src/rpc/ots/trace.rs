use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Transaction Trace
// -----------------------------------------------------------------------------

/// A transaction trace tree object.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionTrace {
  /// The type of call.
  #[serde(rename = "type")]
  pub kind: String,
  /// The depth of the transaction trace in the call stack.
  pub depth: u64,
  /// The ETH sender or contract creator.
  pub from: Address,
  /// The ETH receiver or newly created contract address.
  pub to: Address,
  /// The amount of ETH transferred.
  pub value: Wei,
  /// The input data of the transaction.
  pub input: Bytes,
  /// The output data of the transaction.
  pub output: Bytes,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::ots::TransactionTrace;

  #[test]
  fn test_serde() {
    const SEND: &str = "0xbc0bbe7cf03d64a43e729bdeb36da5c254125e84";
    const RECV: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    const DATA: &str = "0xa9059cbb000000000000000000000000445d088e2ceee3973f3c200988ca38213c8796e30000000000000000000000000000000000000000000000000000000007e53c36";

    let a: TransactionTrace = TransactionTrace::default();

    let b: TransactionTrace = TransactionTrace {
      kind: "CALL".to_owned(),
      depth: 0,
      from: SEND.parse().unwrap(),
      to: RECV.parse().unwrap(),
      value: Default::default(),
      input: DATA.parse().unwrap(),
      output: Default::default(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionTrace",
        len: 7,
      },
      Token::Str("type"),
      Token::Str(""),
      Token::Str("depth"),
      Token::U64(0),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str("0x"),
      Token::Str("output"),
      Token::Str("0x"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionTrace",
        len: 7,
      },
      Token::Str("type"),
      Token::Str("CALL"),
      Token::Str("depth"),
      Token::U64(0),
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("to"),
      Token::Str(RECV),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::Str("input"),
      Token::Str(DATA),
      Token::Str("output"),
      Token::Str("0x"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
