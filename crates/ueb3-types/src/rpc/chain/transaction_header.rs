use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::H256;

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
