use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use serde::de::Unexpected;

use crate::eth::Address;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Internal Operation
// -----------------------------------------------------------------------------

/// The internal ETH transfers inside a transaction.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
  /// The type of internal operation.
  #[serde(rename = "type")]
  pub kind: OperationType,
  /// The ETH sender, contract creator or contract address being self-destructed.
  pub from: Address,
  /// The ETH receiver, newly created contract address or the target ETH receiver
  /// of a self-destruct.
  pub to: Address,
  /// The amount of ETH transferred.
  pub value: Wei,
}

// -----------------------------------------------------------------------------
// Internal Operation Type
// -----------------------------------------------------------------------------

/// An internal operation type.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum OperationType {
  /// `transfer` operation type.
  Transfer = 0,
  /// `self-destruct` operation type.
  SelfDestruct = 1,
  /// `create` operation type.
  Create = 2,
  /// `create2` operation type.
  Create2 = 3,
}

impl<'de> Deserialize<'de> for OperationType {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let kind: Self = match u8::deserialize(deserializer)? {
      0 => Self::Transfer,
      1 => Self::SelfDestruct,
      2 => Self::Create,
      3 => Self::Create2,
      value => {
        return Err(D::Error::invalid_value(
          Unexpected::Unsigned(u64::from(value)),
          &"an unsigned integer from `0` to `3`",
        ));
      }
    };

    Ok(kind)
  }
}

impl Serialize for OperationType {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u8(*self as u8)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::ots::Operation;
  use crate::rpc::ots::OperationType;

  #[test]
  fn test_serde_type() {
    assert_tokens(&OperationType::Transfer, &[Token::U8(0)]);
    assert_tokens(&OperationType::SelfDestruct, &[Token::U8(1)]);
    assert_tokens(&OperationType::Create, &[Token::U8(2)]);
    assert_tokens(&OperationType::Create2, &[Token::U8(3)]);
  }

  #[test]
  fn test_serde() {
    const SEND: &str = "0x3fc91a3afd70395cd496c647d5a6cc9d4b2b7fad";
    const RECV: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";

    let a: Operation = Operation {
      kind: OperationType::Create,
      from: Default::default(),
      to: Default::default(),
      value: Default::default(),
    };

    let b: Operation = Operation {
      kind: OperationType::Transfer,
      from: SEND.parse().unwrap(),
      to: RECV.parse().unwrap(),
      value: "0xf8b0a10e470000".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Operation",
        len: 4,
      },
      Token::Str("type"),
      Token::U8(2),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("value"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Operation",
        len: 4,
      },
      Token::Str("type"),
      Token::U8(0),
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("to"),
      Token::Str(RECV),
      Token::Str("value"),
      Token::Str("0xf8b0a10e470000"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
