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
  /// An array of 0 to 4 indexed log arguments.
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
  use serde::Deserialize;
  use serde::Deserializer;
  use serde::Serialize;
  use serde::Serializer;
  use serde::de::Error as _;
  use serde::de::Expected;
  use serde::ser::Error as _;

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
    let value: Option<Vec<H256>> = Deserialize::deserialize(deserializer)?;
    let value: Vec<H256> = value.unwrap_or_default();

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

    Serialize::serialize(value, serializer)
  }
}
