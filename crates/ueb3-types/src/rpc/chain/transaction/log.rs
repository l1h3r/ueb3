use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::H256;

// -----------------------------------------------------------------------------
// Ethereum Transaction Log
// -----------------------------------------------------------------------------

/// A log object emitted by a transaction.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionLog {
  /// The index in the block that includes the log. `None` when pending.
  #[serde(default, with = "ueb3_serde::quantity::opt")]
  pub log_index: Option<u64>,
  /// The hash of the transaction that created the log. `None` when pending.
  pub transaction_hash: Option<H256>,
  /// The index of the transaction that created the log. `None` when pending.
  #[serde(default, with = "ueb3_serde::quantity::opt")]
  pub transaction_index: Option<u64>,
  /// The hash of the block that includes the log. `None` when pending.
  pub block_hash: Option<H256>,
  /// The number of the block that includes the log. `None` when pending.
  #[serde(default, with = "ueb3_serde::quantity::opt")]
  pub block_number: Option<u64>,
  /// The address which triggered the log emission.
  pub address: Address,
  /// Whether the log was removed due to a chain reorganization.
  pub removed: bool,
  /// A list of 0 to 4 indexed log arguments.
  #[serde(with = "serde_topics")]
  pub topics: Vec<H256>,
  /// Additional non-indexed log arguments.
  pub data: Bytes,
  /// The unix timestamp of the block that includes the log. `None` when pending.
  ///
  /// Note: This is not yet standardized
  ///   <https://github.com/ethereum/execution-apis/issues/295>
  ///   <https://github.com/ethereum/execution-apis/pull/639>
  #[serde(
    default,
    alias = "timestamp", // erigon_getLatestLogs sets `timestamp`
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub block_timestamp: Option<u64>,
}

impl Debug for TransactionLog {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionLog")
      .field("log_index", &self.log_index.format())
      .field("transaction_hash", &self.transaction_hash.format())
      .field("transaction_index", &self.transaction_index.format())
      .field("block_hash", &self.block_hash.format())
      .field("block_number", &self.block_number.format())
      .field("address", &self.address)
      .field("removed", &self.removed)
      .field("topics", &self.topics)
      .field("data", &self.data)
      .field("block_timestamp", &self.block_timestamp.format())
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Serde Utilities
// -----------------------------------------------------------------------------

mod serde_topics {
  use serde::Deserializer;
  use serde::Serializer;
  use serde::de::Error as _;
  use serde::de::Expected;
  use serde::ser::Error as _;
  use ueb3_serde::nullable::sequence;

  use crate::eth::H256;

  static EXPECTED: &str = "an array of 0 to 4 topics";

  #[cold]
  fn invalid_length<S>(length: usize, expected: &dyn Expected) -> S::Error
  where
    S: Serializer,
  {
    S::Error::custom(format_args!("invalid length {length}, expected {expected}"))
  }

  #[inline]
  pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<H256>, D::Error>
  where
    D: Deserializer<'de>,
  {
    // Note: Some nodes return `"topics": null`
    let value: Vec<H256> = sequence::deserialize(deserializer)?;

    if value.len() > 4 {
      return Err(D::Error::invalid_length(value.len(), &EXPECTED));
    }

    Ok(value)
  }

  #[inline]
  pub(super) fn serialize<S>(value: &Vec<H256>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if value.len() > 4 {
      return Err(invalid_length::<S>(value.len(), &EXPECTED));
    }

    sequence::serialize(value, serializer)
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

  use crate::rpc::TransactionLog;

  #[test]
  fn test_serde() {
    const TXN_HASH_A: &str = "0x66e7a140c8fa27fe98fde923defea7562c3ca2d6bb89798aabec65782c08f63d";
    const TXN_HASH_B: &str = "0xdb17aa1c2ce609132f599155d384c0bc5334c988a6c368056d7e167e23eee058";

    const BLOCK_HASH_A: &str = "0xfc139f5e2edee9e9c888d8df9a2d2226133a9bd87c88ccbd9c930d3d4c9f9ef5";
    const BLOCK_HASH_B: &str = "0x98b0ec0f9fea0018a644959accbe69cd046a8582e89402e1ab0ada91cad644ed";

    const DATA_A: &str = "0x0000000000000000000000000000000000000000000000000000000000000004";
    const DATA_B: &str = "0x0000000000000000000000000000000000000000000000000000000000000007";

    const ADDRESS: &str = "0x42699a7612a82f1d9c36148af9c77354759b210b";
    const TOPIC_0: &str = "0x04474795f5b996ff80cb47c148d4c5ccdbe09ef27551820caa9c2f8ed149cce3";

    let a: TransactionLog = TransactionLog {
      log_index: Some(0x0),
      transaction_hash: Some(TXN_HASH_A.parse().unwrap()),
      transaction_index: Some(0x0),
      block_hash: Some(BLOCK_HASH_A.parse().unwrap()),
      block_number: Some(0x233),
      address: ADDRESS.parse().unwrap(),
      removed: false,
      topics: vec![TOPIC_0.parse().unwrap()],
      data: DATA_A.parse().unwrap(),
      block_timestamp: None,
    };

    let b: TransactionLog = TransactionLog {
      log_index: Some(0x0),
      transaction_hash: Some(TXN_HASH_B.parse().unwrap()),
      transaction_index: Some(0x0),
      block_hash: Some(BLOCK_HASH_B.parse().unwrap()),
      block_number: Some(0x238),
      address: ADDRESS.parse().unwrap(),
      removed: false,
      topics: vec![TOPIC_0.parse().unwrap()],
      data: DATA_B.parse().unwrap(),
      block_timestamp: None,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionLog",
        len: 9,
      },
      Token::Str("logIndex"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("transactionHash"),
      Token::Some,
      Token::Str(TXN_HASH_A),
      Token::Str("transactionIndex"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("blockHash"),
      Token::Some,
      Token::Str(BLOCK_HASH_A),
      Token::Str("blockNumber"),
      Token::Some,
      Token::Str("0x233"),
      Token::Str("address"),
      Token::Str(ADDRESS),
      Token::Str("removed"),
      Token::Bool(false),
      Token::Str("topics"),
      Token::Some,
      Token::Seq { len: Some(1) },
      Token::Str(TOPIC_0),
      Token::SeqEnd,
      Token::Str("data"),
      Token::Str(DATA_A),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionLog",
        len: 9,
      },
      Token::Str("logIndex"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("transactionHash"),
      Token::Some,
      Token::Str(TXN_HASH_B),
      Token::Str("transactionIndex"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("blockHash"),
      Token::Some,
      Token::Str(BLOCK_HASH_B),
      Token::Str("blockNumber"),
      Token::Some,
      Token::Str("0x238"),
      Token::Str("address"),
      Token::Str(ADDRESS),
      Token::Str("removed"),
      Token::Bool(false),
      Token::Str("topics"),
      Token::Some,
      Token::Seq { len: Some(1) },
      Token::Str(TOPIC_0),
      Token::SeqEnd,
      Token::Str("data"),
      Token::Str(DATA_B),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
