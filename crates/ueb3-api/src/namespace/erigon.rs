use std::collections::HashMap;
use ueb3_rpc::Result;
use ueb3_types::eip::eip1898::BlockId;
use ueb3_types::eth::Address;
use ueb3_types::eth::H256;
use ueb3_types::eth::U64;
use ueb3_types::eth::Wei;
use ueb3_types::p2p::NodeInfo;
use ueb3_types::rpc::Block;
use ueb3_types::rpc::BlockNumber;
use ueb3_types::rpc::Filter;
use ueb3_types::rpc::Transaction;
use ueb3_types::rpc::TransactionLog;
use ueb3_types::rpc::TransactionReceipt;
use ueb3_types::rpc::erigon;

use crate::provider::Provider;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsErigon for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (erigon)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `erigon` namespace.
pub trait NsErigon: private::Sealed + Provider {
  /// Returns the number of the latest block.
  #[doc(alias = "erigon_blockNumber")]
  #[inline]
  fn block_number(&self, block: Option<BlockNumber>) -> Result<u64> {
    let value: U64 = match block {
      Some(BlockNumber::Integer(value)) => return Ok(value),
      Some(block) => self.client().rpc("erigon_blockNumber", (block,))?,
      None => self.client().rpc("erigon_blockNumber", ())?,
    };

    Ok(value.into_u64())
  }

  /// Returns the details of blockchain forks, including genesis block hash,
  /// height forks, and time forks.
  #[doc(alias = "erigon_forks")]
  #[inline]
  fn forks(&self) -> Result<erigon::Forks> {
    self.client().rpc("erigon_forks", ())
  }

  /// Returns all ETH balance changes in a block matching the given block id.
  #[doc(alias = "erigon_getBalanceChangesInBlock")]
  #[inline]
  fn balance_changes_in_block(&self, block: BlockId) -> Result<HashMap<Address, Wei>> {
    self
      .client()
      .rpc("erigon_getBalanceChangesInBlock", (block,))
  }

  /// Returns information about a block matching the given timestamp.
  #[doc(alias = "erigon_getBlockByTimestamp")]
  #[inline]
  fn block_summary_by_timestamp(&self, timestamp: u64) -> Result<Block<H256>> {
    self
      .client()
      .rpc("erigon_getBlockByTimestamp", (timestamp, false))
  }

  /// Returns information about a block matching the given timestamp.
  #[doc(alias = "erigon_getBlockByTimestamp")]
  #[inline]
  fn block_details_by_timestamp(&self, timestamp: u64) -> Result<Block<Transaction>> {
    self
      .client()
      .rpc("erigon_getBlockByTimestamp", (timestamp, true))
  }

  /// Returns the receipts for all transactions in a block matching the given block hash.
  #[doc(alias = "erigon_getBlockReceiptsByBlockHash")]
  #[inline]
  fn block_receipts_by_block_hash(&self, block: H256) -> Result<Vec<TransactionReceipt>> {
    self
      .client()
      .rpc("erigon_getBlockReceiptsByBlockHash", (block,))
  }

  /// Returns the header of a block matching the given block hash.
  ///
  /// See also [`header`][Self::header].
  #[doc(alias = "erigon_getHeaderByHash")]
  #[inline]
  fn header_by_hash(&self, block: H256) -> Result<Block<()>> {
    self.client().rpc("erigon_getHeaderByHash", (block,))
  }

  /// Returns the header of a block matching the given block number.
  ///
  /// See also [`header`][Self::header].
  #[doc(alias = "erigon_getHeaderByNumber")]
  #[inline]
  fn header_by_number(&self, block: BlockNumber) -> Result<Block<()>> {
    self.client().rpc("erigon_getHeaderByNumber", (block,))
  }

  /// Returns the header of a block matching the given block hash.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "erigon_getHeaderByHash")]
  #[doc(alias = "erigon_getHeaderByNumber")]
  #[inline]
  fn header(&self, block: BlockId) -> Result<Block<()>> {
    match block {
      BlockId::Hash(id) => NsErigon::header_by_hash(self, id.to_hash()),
      BlockId::Bnum(id) => NsErigon::header_by_number(self, id),
    }
  }

  /// Returns an array of all logs matching a given filter object.
  #[doc(alias = "erigon_getLatestLogs")]
  #[inline]
  fn latest_logs(
    &self,
    filter: &Filter,
    options: erigon::FilterOptions,
  ) -> Result<Vec<TransactionLog>> {
    self.client().rpc("erigon_getLatestLogs", (filter, options))
  }

  /// Returns an array of all logs matching a given filter object.
  #[doc(alias = "erigon_getLogs")]
  #[inline]
  fn logs(&self, filter: &Filter) -> Result<Vec<TransactionLog>> {
    self.client().rpc("erigon_getLogs", (filter,))
  }

  /// Returns an array of all logs matching the given block hash.
  #[doc(alias = "erigon_getLogsByHash")]
  #[inline]
  fn logs_by_hash(&self, block: H256) -> Result<Vec<TransactionLog>> {
    #[inline]
    fn flatten(value: Vec<Option<Vec<TransactionLog>>>) -> Vec<TransactionLog> {
      value.into_iter().flatten().flatten().collect()
    }

    self
      .client()
      .rpc("erigon_getLogsByHash", (block,))
      .map(flatten)
  }

  /// Returns all the information known about the running Erigon node at the
  /// networking granularity.
  #[doc(alias = "erigon_nodeInfo")]
  #[inline]
  fn node_info(&self) -> Result<Vec<NodeInfo>> {
    self.client().rpc("erigon_nodeInfo", ())
  }
}
