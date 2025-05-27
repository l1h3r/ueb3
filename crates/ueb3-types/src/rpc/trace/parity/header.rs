use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::H256;

// -----------------------------------------------------------------------------
// Parity Trace Header
// -----------------------------------------------------------------------------

/// The common properties of a parity-style transaction trace.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TraceHeader {
  /// The hash of the block that includes the transaction.
  pub block_hash: Option<H256>,
  /// The number of the block that includes the transaction.
  pub block_number: Option<u64>,
  /// The hash of the transaction.
  pub transaction_hash: Option<H256>,
  /// The index in the block that includes the transaction.
  pub transaction_position: Option<u64>,
}

impl Debug for TraceHeader {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TraceHeader")
      .field("block_hash", &self.block_hash.format())
      .field("block_number", &self.block_number.format())
      .field("transaction_hash", &self.transaction_hash.format())
      .field("transaction_position", &self.transaction_position.format())
      .finish()
  }
}

impl SerializeFlatten for TraceHeader {
  const NAME: &'static str = "TraceHeader";

  #[inline]
  fn fields(&self) -> usize {
    4
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("blockHash", &self.block_hash)?;
    state.serialize_field("blockNumber", &self.block_number)?;
    state.serialize_field("transactionHash", &self.transaction_hash)?;
    state.serialize_field("transactionPosition", &self.transaction_position)?;

    Ok(())
  }
}

impl Serialize for TraceHeader {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
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

  use crate::rpc::trace::TraceHeader;

  #[test]
  fn test_serde() {
    const B_HASH: &str = "0x7eb25504e4c202cf3d62fd585d3e238f592c780cca82dacb2ed3cb5b38883add";
    const T_HASH: &str = "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3";

    let a: TraceHeader = TraceHeader::default();

    let b: TraceHeader = TraceHeader {
      block_hash: Some(B_HASH.parse().unwrap()),
      block_number: Some(3068185),
      transaction_hash: Some(T_HASH.parse().unwrap()),
      transaction_position: Some(5),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TraceHeader",
        len: 4,
      },
      Token::Str("blockHash"),
      Token::None,
      Token::Str("blockNumber"),
      Token::None,
      Token::Str("transactionHash"),
      Token::None,
      Token::Str("transactionPosition"),
      Token::None,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TraceHeader",
        len: 4,
      },
      Token::Str("blockHash"),
      Token::Some,
      Token::Str(B_HASH),
      Token::Str("blockNumber"),
      Token::Some,
      Token::U64(3068185),
      Token::Str("transactionHash"),
      Token::Some,
      Token::Str(T_HASH),
      Token::Str("transactionPosition"),
      Token::Some,
      Token::U64(5),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
