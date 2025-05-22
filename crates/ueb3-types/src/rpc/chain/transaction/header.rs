use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::H256;
use crate::eth::U64;

// -----------------------------------------------------------------------------
// Ethereum Transaction Header
// -----------------------------------------------------------------------------

/// An object of basic identifying information relevant to a transaction.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct TransactionHeader {
  /// The hash of the transaction.
  pub hash: H256,
  /// The index in the block that includes the transaction. `None` when pending.
  pub index: Option<u64>,
  /// The hash of the block that includes the transaction. `None` when pending.
  pub block_hash: Option<H256>,
  /// The number of the block that includes the transaction. `None` when pending.
  pub block_number: Option<u64>,
}

impl Debug for TransactionHeader {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionHeader")
      .field("hash", &self.hash)
      .field("index", &self.index.format())
      .field("block_hash", &self.block_hash.format())
      .field("block_number", &self.block_number.format())
      .finish()
  }
}

impl SerializeFlatten for TransactionHeader {
  const NAME: &'static str = "TransactionHeader";

  #[inline]
  fn fields(&self) -> usize {
    4
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("hash", &self.hash)?;
    state.serialize_field("transactionIndex", &self.index.map(U64::from_u64))?;
    state.serialize_field("blockHash", &self.block_hash)?;
    state.serialize_field("blockNumber", &self.block_number.map(U64::from_u64))?;

    Ok(())
  }
}

impl Serialize for TransactionHeader {
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
  use serde_test::assert_ser_tokens;

  use crate::rpc::TransactionHeader;

  #[test]
  fn test_serde() {
    const B_HASH: &str = "0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd";
    const T_HASH: &str = "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060";

    let a: TransactionHeader = TransactionHeader::default();

    let b: TransactionHeader = TransactionHeader {
      hash: T_HASH.parse().unwrap(),
      index: Some(0x0),
      block_hash: Some(B_HASH.parse().unwrap()),
      block_number: Some(0xb443),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionHeader",
        len: 4,
      },
      Token::Str("hash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("transactionIndex"),
      Token::None,
      Token::Str("blockHash"),
      Token::None,
      Token::Str("blockNumber"),
      Token::None,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionHeader",
        len: 4,
      },
      Token::Str("hash"),
      Token::Str(T_HASH),
      Token::Str("transactionIndex"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("blockHash"),
      Token::Some,
      Token::Str(B_HASH),
      Token::Str("blockNumber"),
      Token::Some,
      Token::Str("0xb443"),
      Token::StructEnd,
    ];

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
  }
}
