use ueb3_rpc::Result;
use ueb3_types::eip::eip1898::BlockId;
use ueb3_types::eth::Address;
use ueb3_types::eth::Bytes;
use ueb3_types::eth::H256;
use ueb3_types::rpc::BlockNumber;
use ueb3_types::rpc::ots;

use crate::provider::Provider;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsOts for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (ots)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `ots` namespace.
pub trait NsOts: private::Sealed + Provider {
  /// Returns the current API level of the Ethereum network.
  #[doc(alias = "ots_getApiLevel")]
  #[inline]
  fn api_level(&self) -> Result<u64> {
    self.client().rpc("ots_getApiLevel", ())
  }

  /// Returns the list of internal operations in the block with the specified block number.
  ///
  /// This traces internal ETH transfers, contract creations (CREATE/CREATE2)
  /// and self-destructs for a certain transaction.
  #[doc(alias = "ots_getInternalOperations")]
  #[inline]
  fn internal_operations(&self, transaction: H256) -> Result<Vec<ots::Operation>> {
    self
      .client()
      .rpc("ots_getInternalOperations", (transaction,))
  }

  /// Checks if an Ethereum address contains a deployed code.
  #[doc(alias = "ots_hasCode")]
  #[inline]
  fn has_code(&self, address: &Address, block: Option<BlockId>) -> Result<bool> {
    self
      .client()
      .rpc("ots_hasCode", (address, block.unwrap_or_default()))
  }

  /// Returns the raw revert reason of the given transaction.
  ///
  /// The returned bytes should be ABI decoded in order to be presented a user.
  #[doc(alias = "ots_getTransactionError")]
  #[inline]
  fn transaction_error(&self, transaction: H256) -> Result<Option<Bytes>> {
    self.client().rpc("ots_getTransactionError", (transaction,))
  }

  /// Traces a transaction and generates a trace call tree.
  #[doc(alias = "ots_traceTransaction")]
  #[inline]
  fn trace_transaction(&self, transaction: H256) -> Result<Vec<ots::TransactionTrace>> {
    self.client().rpc("ots_traceTransaction", (transaction,))
  }

  /// Returns information about a block matching the given block hash.
  ///
  /// See also [`block_details`][Self::block_details].
  #[doc(alias = "ots_getBlockDetailsByHash")]
  #[inline]
  fn block_details_by_hash(&self, block: H256) -> Result<Option<ots::BlockDetails>> {
    self.client().rpc("ots_getBlockDetailsByHash", (block,))
  }

  /// Returns information about a block matching the given block number.
  ///
  /// See also [`block_details`][Self::block_details].
  #[doc(alias = "ots_getBlockDetails")]
  #[inline]
  fn block_details_by_number(&self, block: BlockNumber) -> Result<Option<ots::BlockDetails>> {
    self.client().rpc("ots_getBlockDetails", (block,))
  }

  /// Returns information about a block matching the given block id.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "ots_getBlockDetailsByHash")]
  #[doc(alias = "ots_getBlockDetails")]
  #[inline]
  fn block_details(&self, block: BlockId) -> Result<Option<ots::BlockDetails>> {
    match block {
      BlockId::Hash(id) => NsOts::block_details_by_hash(self, id.to_hash()),
      BlockId::Bnum(id) => NsOts::block_details_by_number(self, id),
    }
  }

  /// Returns the list of transactions in the block with the specified block number.
  #[doc(alias = "ots_getBlockTransactions")]
  #[inline]
  fn block_transactions(
    &self,
    block: u64,
    page: u8,
    count: u8,
  ) -> Result<Option<ots::BlockTransactions>> {
    self
      .client()
      .rpc("ots_getBlockTransactions", (block, page, count))
  }

  /// Returns a list of transactions that were sent before the specified block number.
  #[doc(alias = "ots_searchTransactionsBefore")]
  #[inline]
  fn search_transactions_before(
    &self,
    address: &Address,
    block: u64,
    count: u16,
  ) -> Result<ots::BlockTransactionSearch> {
    self
      .client()
      .rpc("ots_searchTransactionsBefore", (address, block, count))
  }

  /// Returns a list of transactions that were sent after the specified block number.
  #[doc(alias = "ots_searchTransactionsAfter")]
  #[inline]
  fn search_transactions_after(
    &self,
    address: &Address,
    block: u64,
    count: u16,
  ) -> Result<ots::BlockTransactionSearch> {
    self
      .client()
      .rpc("ots_searchTransactionsAfter", (address, block, count))
  }

  /// Gets the transaction hash for a certain sender address and nonce.
  #[doc(alias = "ots_getTransactionBySenderAndNonce")]
  #[inline]
  fn transaction_by_sender_and_nonce(&self, address: &Address, nonce: u64) -> Result<Option<H256>> {
    self
      .client()
      .rpc("ots_getTransactionBySenderAndNonce", (address, nonce))
  }

  /// Returns the transaction hash and the direct address that initiated the
  /// creation of the contract when given an Ethereum contract address.
  #[doc(alias = "ots_getContractCreator")]
  #[inline]
  fn contract_creator(&self, address: &Address) -> Result<Option<ots::ContractCreator>> {
    self.client().rpc("ots_getContractCreator", (address,))
  }
}
