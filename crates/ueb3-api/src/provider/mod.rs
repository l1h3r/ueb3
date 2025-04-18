use std::time::Duration;
use ueb3_rpc::Result;
use ueb3_rpc::RpcClient;
use ueb3_types::eip::eip1186::AccountProof;
use ueb3_types::eip::eip1898::BlockId;
use ueb3_types::eip::eip2930::AccessListResult;
use ueb3_types::eth::Address;
use ueb3_types::eth::Bytes;
use ueb3_types::eth::H64;
use ueb3_types::eth::H256;
use ueb3_types::eth::Signature;
use ueb3_types::eth::Slice;
use ueb3_types::eth::U64;
use ueb3_types::eth::U256;
use ueb3_types::eth::Wei;
use ueb3_types::rpc::Block;
use ueb3_types::rpc::BlockNumber;
use ueb3_types::rpc::FeeHistory;
use ueb3_types::rpc::Filter;
use ueb3_types::rpc::SyncResult;
use ueb3_types::rpc::Transaction;
use ueb3_types::rpc::TransactionId;
use ueb3_types::rpc::TransactionLog;
use ueb3_types::rpc::TransactionReceipt;
use ueb3_types::rpc::TransactionRequest;
use ueb3_types::rpc::Value;
use ueb3_types::rpc::WorkBundle;
use ueb3_utils::traits::DecodeOwned;

use crate::future::PendingTransaction;
use crate::stream::FilterId;
use crate::stream::FilterStream;

// -----------------------------------------------------------------------------
// Default Provider Implementations
// -----------------------------------------------------------------------------

impl Provider for RpcClient {
  #[inline]
  fn client(&self) -> &RpcClient {
    self
  }
}

// -----------------------------------------------------------------------------
// Provider
// -----------------------------------------------------------------------------

/// A high-level interface to interect with Ethereum nodes via JSON-RPC 2.0.
pub trait Provider {
  /// Returns a reference to the low-level JSON-RPC 2.0 client interface.
  fn client(&self) -> &RpcClient;

  /// The default polling interval to request filter changes.
  #[inline]
  fn poller(&self) -> Duration {
    Duration::from_millis(7000)
  }

  // ---------------------------------------------------------------------------
  // Block Methods
  // ---------------------------------------------------------------------------

  /// Returns information about a block matching the given block hash.
  ///
  /// See also [`block_summary`][Self::block_summary].
  #[doc(alias = "eth_getBlockByHash")]
  #[inline]
  fn block_summary_by_hash(&self, block: H256) -> Result<Option<Block<H256>>> {
    self.client().rpc("eth_getBlockByHash", (block, false))
  }

  /// Returns information about a block matching the given block number.
  ///
  /// See also [`block_summary`][Self::block_summary].
  #[doc(alias = "eth_getBlockByNumber")]
  #[inline]
  fn block_summary_by_number(&self, block: BlockNumber) -> Result<Option<Block<H256>>> {
    self.client().rpc("eth_getBlockByNumber", (block, false))
  }

  /// Returns information about a block matching the given block id.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "eth_getBlockByHash")]
  #[doc(alias = "eth_getBlockByNumber")]
  #[inline]
  fn block_summary(&self, block: BlockId) -> Result<Option<Block<H256>>> {
    match block {
      BlockId::Hash(id) => Provider::block_summary_by_hash(self, id.to_hash()),
      BlockId::Bnum(id) => Provider::block_summary_by_number(self, id),
    }
  }

  /// Returns information about a block matching the given block hash.
  ///
  /// See also [`block_details`][Self::block_details].
  #[doc(alias = "eth_getBlockByHash")]
  #[inline]
  fn block_details_by_hash(&self, block: H256) -> Result<Option<Block<Transaction>>> {
    self.client().rpc("eth_getBlockByHash", (block, true))
  }

  /// Returns information about a block matching the given block number.
  ///
  /// See also [`block_details`][Self::block_details].
  #[doc(alias = "eth_getBlockByNumber")]
  #[inline]
  fn block_details_by_number(&self, block: BlockNumber) -> Result<Option<Block<Transaction>>> {
    self.client().rpc("eth_getBlockByNumber", (block, true))
  }

  /// Returns information about a block matching the given block id.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "eth_getBlockByHash")]
  #[doc(alias = "eth_getBlockByNumber")]
  #[inline]
  fn block_details(&self, block: BlockId) -> Result<Option<Block<Transaction>>> {
    match block {
      BlockId::Hash(id) => Provider::block_details_by_hash(self, id.to_hash()),
      BlockId::Bnum(id) => Provider::block_details_by_number(self, id),
    }
  }

  /// Returns the receipts of a block by number or hash.
  #[doc(alias = "eth_getBlockReceipts")]
  #[inline]
  fn block_receipts(&self, block: BlockId) -> Result<Option<Vec<TransactionReceipt>>> {
    self.client().rpc("eth_getBlockReceipts", (block,))
  }

  /// Returns the number of transactions in a block matching the given block hash.
  ///
  /// See also [`block_transaction_count`][Self::block_transaction_count].
  #[doc(alias = "eth_getBlockTransactionCountByHash")]
  #[inline]
  fn block_transaction_count_by_hash(&self, block: H256) -> Result<Option<u64>> {
    self
      .client()
      .rpc("eth_getBlockTransactionCountByHash", (block,))
      .map(|value: Option<U64>| value.map(U64::into_u64))
  }

  /// Returns the number of transactions in a block matching the given block number.
  ///
  /// See also [`block_transaction_count`][Self::block_transaction_count].
  #[doc(alias = "eth_getBlockTransactionCountByNumber")]
  #[inline]
  fn block_transaction_count_by_number(&self, block: BlockNumber) -> Result<Option<u64>> {
    self
      .client()
      .rpc("eth_getBlockTransactionCountByNumber", (block,))
      .map(|value: Option<U64>| value.map(U64::into_u64))
  }

  /// Returns the number of transactions in a block matching the given block id.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "eth_getBlockTransactionCountByHash")]
  #[doc(alias = "eth_getBlockTransactionCountByNumber")]
  #[inline]
  fn block_transaction_count(&self, block: BlockId) -> Result<Option<u64>> {
    match block {
      BlockId::Hash(id) => Provider::block_transaction_count_by_hash(self, id.to_hash()),
      BlockId::Bnum(id) => Provider::block_transaction_count_by_number(self, id),
    }
  }

  /// Returns information about an uncle of a block matching the given block hash
  /// and uncle index position.
  ///
  /// See also [`uncle`][Self::uncle].
  #[doc(alias = "eth_getUncleByBlockHashAndIndex")]
  #[inline]
  fn uncle_by_block_hash_and_index(
    &self,
    block: H256,
    index: u64,
  ) -> Result<Option<Block<()>>> {
    self.client().rpc("eth_getUncleByBlockHashAndIndex", (block, U64::from_u64(index)))
  }

  /// Returns information about an uncle of a block matching the given block
  /// number and uncle index position.
  ///
  /// See also [`uncle`][Self::uncle].
  #[doc(alias = "eth_getUncleByBlockNumberAndIndex")]
  #[inline]
  fn uncle_by_block_number_and_index(
    &self,
    block: BlockNumber,
    index: u64,
  ) -> Result<Option<Block<()>>> {
    self.client().rpc("eth_getUncleByBlockNumberAndIndex", (block, U64::from_u64(index)))
  }

  /// Returns information about an uncle of a block matching the given block id
  /// and uncle index position.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "eth_getUncleByBlockHashAndIndex")]
  #[doc(alias = "eth_getUncleByBlockNumberAndIndex")]
  #[inline]
  fn uncle(&self, block: BlockId, index: u64) -> Result<Option<Block<()>>> {
    match block {
      BlockId::Hash(id) => Provider::uncle_by_block_hash_and_index(self, id.to_hash(), index),
      BlockId::Bnum(id) => Provider::uncle_by_block_number_and_index(self, id, index),
    }
  }

  /// Returns the number of uncles in a block matching the given block hash.
  ///
  /// See also [`uncle_count`][Self::uncle_count].
  #[doc(alias = "eth_getUncleCountByBlockHash")]
  #[inline]
  fn uncle_count_by_block_hash(&self, block: H256) -> Result<Option<u64>> {
    self
      .client()
      .rpc("eth_getUncleCountByBlockHash", (block,))
      .map(|value: Option<U64>| value.map(U64::into_u64))
  }

  /// Returns the number of uncles in a block matching the given block number.
  ///
  /// See also [`uncle_count`][Self::uncle_count].
  #[doc(alias = "eth_getUncleCountByBlockNumber")]
  #[inline]
  fn uncle_count_by_block_number(&self, block: BlockNumber) -> Result<Option<u64>> {
    self
      .client()
      .rpc("eth_getUncleCountByBlockNumber", (block,))
      .map(|value: Option<U64>| value.map(U64::into_u64))
  }

  /// Returns the number of uncles in a block matching the given block id.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "eth_getUncleCountByBlockHash")]
  #[doc(alias = "eth_getUncleCountByBlockNumber")]
  #[inline]
  fn uncle_count(&self, block: BlockId) -> Result<Option<u64>> {
    match block {
      BlockId::Hash(id) => Provider::uncle_count_by_block_hash(self, id.to_hash()),
      BlockId::Bnum(id) => Provider::uncle_count_by_block_number(self, id),
    }
  }

  // ---------------------------------------------------------------------------
  // Client Methods
  // ---------------------------------------------------------------------------

  /// Returns a list of addresses owned by the client.
  #[doc(alias = "eth_accounts")]
  #[inline]
  fn accounts(&self) -> Result<Vec<Address>> {
    self.client().rpc("eth_accounts", ())
  }

  /// Returns the number of the most recent block.
  #[doc(alias = "eth_blockNumber")]
  #[inline]
  fn block_number(&self) -> Result<u64> {
    self.client().rpc("eth_blockNumber", ()).map(U64::into_u64)
  }

  /// Returns the chain ID of the current network.
  #[doc(alias = "eth_chainId")]
  #[doc(alias = "EIP-695")]
  #[inline]
  fn chain_id(&self) -> Result<u64> {
    self.client().rpc("eth_chainId", ()).map(U64::into_u64)
  }

  /// Returns the client coinbase address.
  #[doc(alias = "eth_coinbase")]
  #[inline]
  fn coinbase(&self) -> Result<Address> {
    self.client().rpc("eth_coinbase", ())
  }

  /// Returns an object with data about the sync status or `false`.
  #[doc(alias = "eth_syncing")]
  #[inline]
  fn syncing(&self) -> Result<SyncResult<Value>> {
    self.client().rpc("eth_syncing", ())
  }

  // ---------------------------------------------------------------------------
  // Execution Methods
  // ---------------------------------------------------------------------------

  /// Executes a new message call immediately without creating a transaction.
  #[doc(alias = "eth_call")]
  #[inline]
  fn call(&self, request: &TransactionRequest, block: Option<BlockId>) -> Result<Bytes> {
    self.client().rpc("eth_call", (request, block.unwrap_or_default()))
  }

  /// Generates an access list for a transaction.
  #[doc(alias = "eth_createAccessList")]
  #[inline]
  fn create_access_list(
    &self,
    request: &TransactionRequest,
    block: Option<BlockId>,
  ) -> Result<AccessListResult> {
    self.client().rpc("eth_createAccessList", (request, block))
  }

  /// Generates and returns an estimate of how much gas is necessary to allow
  /// the transaction to complete.
  #[doc(alias = "eth_estimateGas")]
  #[inline]
  fn estimate_gas(
    &self,
    request: &TransactionRequest,
    block: Option<BlockId>,
  ) -> Result<U256> {
    self.client().rpc("eth_estimateGas", (request, block.unwrap_or_default()))
  }

  // ---------------------------------------------------------------------------
  // Fee Market Methods
  // ---------------------------------------------------------------------------

  /// Returns the base fee per blob gas.
  #[doc(alias = "eth_blobBaseFee")]
  #[inline]
  fn blob_base_fee(&self) -> Result<Wei> {
    self.client().rpc("eth_blobBaseFee", ())
  }

  /// Returns transaction base fee per gas and effective priority fee per gas
  /// for the requested/supported block range.
  #[doc(alias = "eth_feeHistory")]
  #[inline]
  fn fee_history(&self, count: u64, block: BlockNumber, percentiles: &[f64]) -> Result<FeeHistory> {
    self
      .client()
      .rpc("eth_feeHistory", (U64::from_u64(count), block, percentiles))
  }

  /// Returns the current price per gas.
  #[doc(alias = "eth_gasPrice")]
  #[inline]
  fn gas_price(&self) -> Result<Wei> {
    self.client().rpc("eth_gasPrice", ())
  }

  /// Returns the current `maxPriorityFeePerGas`.
  #[doc(alias = "eth_maxPriorityFeePerGas")]
  #[inline]
  fn max_priority_fee(&self) -> Result<Wei> {
    self.client().rpc("eth_maxPriorityFeePerGas", ())
  }

  // ---------------------------------------------------------------------------
  // Filter Methods
  // ---------------------------------------------------------------------------

  /// Returns an array of logs which occurred since last poll.
  #[doc(alias = "eth_getFilterChanges")]
  #[inline]
  fn filter_changes<R>(&self, id: FilterId) -> Result<Vec<R>>
  where
    R: DecodeOwned,
    Self: Sized, // maintain dyn compatibility
  {
    self.client().rpc("eth_getFilterChanges", (id,))
  }

  /// Returns an array of all logs matching the filter specified by `id`.
  #[doc(alias = "eth_getFilterLogs")]
  #[inline]
  fn filter_logs(&self, id: FilterId) -> Result<Vec<TransactionLog>> {
    self.client().rpc("eth_getFilterLogs", (id,))
  }

  /// Returns an array of all logs matching the specified filter.
  #[doc(alias = "eth_getLogs")]
  #[inline]
  fn logs(&self, filter: &Filter) -> Result<Vec<TransactionLog>> {
    self.client().rpc("eth_getLogs", (filter,))
  }

  /// Creates a filter in the node, to notify when a new block arrives.
  ///
  /// See also [`watch_blocks`][Self::watch_blocks] which returns a `Stream`-compatible type.
  #[doc(alias = "eth_newBlockFilter")]
  #[inline]
  fn new_block_filter(&self) -> Result<FilterId> {
    self.client().rpc("eth_newBlockFilter", ())
  }

  /// Creates a filter object, based on filter options, to notify when the state changes.
  ///
  /// See also [`watch_logs`][Self::watch_logs] which returns a `Stream`-compatible type.
  #[doc(alias = "eth_newFilter")]
  #[inline]
  fn new_log_filter(&self, filter: &Filter) -> Result<FilterId> {
    self.client().rpc("eth_newFilter", (filter,))
  }

  /// Creates a filter in the node, to notify when new pending transactions arrive.
  ///
  /// See also [`watch_transactions`][Self::watch_transactions] which returns a `Stream`-compatible type.
  #[doc(alias = "eth_newPendingTransactionFilter")]
  #[inline]
  fn new_transaction_filter(&self) -> Result<FilterId> {
    self.client().rpc("eth_newPendingTransactionFilter", ())
  }

  /// Uninstalls the filter specified by `id`.
  #[doc(alias = "eth_uninstallFilter")]
  #[inline]
  fn uninstall_filter(&self, id: FilterId) -> Result<bool> {
    self.client().rpc("eth_uninstallFilter", (id,))
  }

  // ---------------------------------------------------------------------------
  // Filter Methods - Streaming
  // ---------------------------------------------------------------------------

  /// Creates a filter stream, to notify when a new block arrives.
  #[doc(alias = "eth_newBlockFilter")]
  #[inline]
  fn watch_blocks(&self) -> Result<FilterStream<'_, Self, H256>>
  where
    Self: Sized, // maintain dyn compatibility
  {
    Ok(FilterStream::new(self, self.new_block_filter()?))
  }

  /// Creates a filter stream, based on filter options, to notify when the state changes.
  #[doc(alias = "eth_newFilter")]
  #[inline]
  fn watch_logs(&self, filter: &Filter) -> Result<FilterStream<'_, Self, TransactionLog>>
  where
    Self: Sized, // maintain dyn compatibility
  {
    Ok(FilterStream::new(self, self.new_log_filter(filter)?))
  }

  /// Creates a filter stream, to notify when new pending transactions arrive.
  #[doc(alias = "eth_newPendingTransactionFilter")]
  #[inline]
  fn watch_transactions(&self) -> Result<FilterStream<'_, Self, H256>>
  where
    Self: Sized, // maintain dyn compatibility
  {
    Ok(FilterStream::new(self, self.new_transaction_filter()?))
  }

  // ---------------------------------------------------------------------------
  // Signing Methods
  // ---------------------------------------------------------------------------

  /// Returns an [EIP-191] signature over the provided data.
  ///
  /// [EIP-191]: https://eips.ethereum.org/EIPS/eip-191
  #[doc(alias = "eth_sign")]
  #[doc(alias = "EIP-191")]
  #[inline]
  fn sign(&self, address: &Address, message: &Slice) -> Result<Signature> {
    self.client().rpc("eth_sign", (address, message))
  }

  /// Returns an RLP encoded transaction signed by the specified account.
  #[doc(alias = "eth_signTransaction")]
  #[inline]
  fn sign_transaction(&self, request: &TransactionRequest) -> Result<Bytes> {
    self.client().rpc("eth_signTransaction", (request,))
  }

  // ---------------------------------------------------------------------------
  // State Methods
  // ---------------------------------------------------------------------------

  /// Returns the balance of the account of the given `address`.
  #[doc(alias = "eth_getBalance")]
  #[inline]
  fn balance(&self, address: &Address, block: Option<BlockId>) -> Result<Wei> {
    self
      .client()
      .rpc("eth_getBalance", (address, block.unwrap_or_default()))
  }

  /// Returns code at a given address.
  #[doc(alias = "eth_getCode")]
  #[inline]
  fn code(&self, address: &Address, block: Option<BlockId>) -> Result<Bytes> {
    self
      .client()
      .rpc("eth_getCode", (address, block.unwrap_or_default()))
  }

  /// Returns the merkle proof for a given account and optionally some storage keys.
  #[doc(alias = "eth_getProof")]
  #[inline]
  fn proof(&self, address: &Address, storage: &[H256], block: Option<BlockId>) -> Result<AccountProof> {
    self.client().rpc("eth_getProof", (address, storage, block.unwrap_or_default()))
  }

  /// Returns the value from a storage position at a given address.
  #[doc(alias = "eth_getStorageAt")]
  #[inline]
  fn storage(&self, address: &Address, storage: U256, block: Option<BlockId>) -> Result<H256> {
    self.client().rpc("eth_getStorageAt", (address, storage, block.unwrap_or_default()))
  }

  /// Returns the nonce of an account in the state.
  ///
  /// NOTE: The name `eth_getTransactionCount` reflects the historical fact that
  /// an account's nonce and sent transaction count were the same. After the
  /// Pectra fork, with the inclusion of [EIP-7702], this is no longer true.
  ///
  /// [EIP-7702]: https://eips.ethereum.org/EIPS/eip-7702
  #[doc(alias = "eth_getTransactionCount")]
  #[inline]
  fn transaction_count(&self, address: &Address, block: Option<BlockId>) -> Result<u64> {
    self
      .client()
      .rpc("eth_getTransactionCount", (address, block.unwrap_or_default()))
      .map(U64::into_u64)
  }

  // ---------------------------------------------------------------------------
  // Submission Methods
  // ---------------------------------------------------------------------------

  /// Submits a raw transaction.
  ///
  /// For [EIP-4844] transactions, the raw form must be the network form.
  /// This means it includes the blobs, KZG commitments, and KZG proofs.
  ///
  /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
  #[doc(alias = "eth_sendRawTransaction")]
  #[inline]
  fn send_raw_transaction(&self, request: &Slice) -> Result<PendingTransaction<'_, Self>>
  where
    Self: Sized, // maintain dyn compatibility
  {
    self
      .client()
      .rpc("eth_sendRawTransaction", (request,))
      .map(|target| PendingTransaction::new(self, target))
  }

  /// Signs and submits a transaction.
  #[doc(alias = "eth_sendTransaction")]
  #[inline]
  fn send_transaction(&self, request: &TransactionRequest) -> Result<PendingTransaction<'_, Self>>
  where
    Self: Sized, // maintain dyn compatibility
  {
    self
      .client()
      .rpc("eth_sendTransaction", (request,))
      .map(|target| PendingTransaction::new(self, target))
  }

  // ---------------------------------------------------------------------------
  // Transaction Methods
  // ---------------------------------------------------------------------------

  /// Returns information about a transaction matching the given block hash and
  /// transaction index position.
  ///
  /// See also [`transaction`][Self::transaction].
  #[doc(alias = "eth_getTransactionByBlockHashAndIndex")]
  #[inline]
  fn transaction_by_block_hash_and_index(
    &self,
    block: H256,
    index: u64,
  ) -> Result<Option<Transaction>> {
    self
      .client()
      .rpc("eth_getTransactionByBlockHashAndIndex", (block, U64::from_u64(index)))
  }

  /// Returns information about a transaction matching the given block number
  /// and transaction index position.
  ///
  /// See also [`transaction`][Self::transaction].
  #[doc(alias = "eth_getTransactionByBlockNumberAndIndex")]
  #[inline]
  fn transaction_by_block_number_and_index(
    &self,
    block: BlockNumber,
    index: u64,
  ) -> Result<Option<Transaction>> {
    self
      .client()
      .rpc("eth_getTransactionByBlockNumberAndIndex", (block, U64::from_u64(index)))
  }

  /// Returns the information about a transaction matching the given transaction hash.
  ///
  /// See also [`transaction`][Self::transaction].
  #[doc(alias = "eth_getTransactionByHash")]
  #[inline]
  fn transaction_by_hash(&self, transaction: H256) -> Result<Option<Transaction>> {
    self
      .client()
      .rpc("eth_getTransactionByHash", (transaction,))
  }

  /// Returns information about a transaction matching the given transaction id.
  ///
  /// Note: [`BlockHash::canonical`] is not supported by this method.
  ///
  /// [`BlockHash::canonical`]: ueb3_types::eip::eip1898::BlockHash::is_canonical
  #[doc(alias = "eth_getTransactionByBlockHashAndIndex")]
  #[doc(alias = "eth_getTransactionByBlockNumberAndIndex")]
  #[doc(alias = "eth_getTransactionByHash")]
  #[inline]
  fn transaction(&self, transaction: TransactionId) -> Result<Option<Transaction>> {
    match transaction {
      TransactionId::Hash(transaction) => {
        Provider::transaction_by_hash(self, transaction)
      }
      TransactionId::Block(BlockId::Hash(block), index) => {
        Provider::transaction_by_block_hash_and_index(self, block.to_hash(), index)
      }
      TransactionId::Block(BlockId::Bnum(block), index) => {
        Provider::transaction_by_block_number_and_index(self, block, index)
      }
    }
  }

  /// Returns the receipt of a transaction matching the given transaction hash.
  #[doc(alias = "eth_getTransactionReceipt")]
  #[inline]
  fn transaction_receipt(&self, transaction: H256) -> Result<Option<TransactionReceipt>> {
    self
      .client()
      .rpc("eth_getTransactionReceipt", (transaction,))
  }

  // ---------------------------------------------------------------------------
  // Legacy Methods (Deprecated)
  // ---------------------------------------------------------------------------

  /// Returns compiled LLL code.
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_compileLLL")]
  #[inline]
  fn compile_lll(&self, code: &str) -> Result<Bytes> {
    self.client().rpc("eth_compileLLL", (code,))
  }

  /// Returns compiled serpent code.
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_compileSerpent")]
  #[inline]
  fn compile_serpent(&self, code: &str) -> Result<Bytes> {
    self.client().rpc("eth_compileSerpent", (code,))
  }

  /// Returns compiled solidity code.
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_compileSolidity")]
  #[inline]
  fn compile_solidity(&self, code: &str) -> Result<Value> {
    self.client().rpc("eth_compileSolidity", (code,))
  }

  /// Returns a list of available compilers in the client.
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_getCompilers")]
  #[inline]
  fn compilers(&self) -> Result<Vec<String>> {
    self.client().rpc("eth_getCompilers", ())
  }

  /// Returns the current Ethereum protocol version.
  ///
  /// Note: This method is not available in Geth.
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_protocolVersion")]
  #[inline]
  fn protocol_version(&self) -> Result<String> {
    self.client().rpc("eth_protocolVersion", ())
  }

  // ---------------------------------------------------------------------------
  // Proof-of-Work Methods (Deprecated)
  // ---------------------------------------------------------------------------

  /// Returns the hash of the current block, the seedHash, and the boundary
  /// condition to be met ("target").
  ///
  /// Note: This may not be available in some clients since [The Merge].
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_getWork")]
  #[inline]
  fn work(&self) -> Result<WorkBundle> {
    self.client().rpc("eth_getWork", ())
  }

  /// Returns the number of hashes per second that the node is mining with.
  ///
  /// Note: This may not be available in some clients since [The Merge].
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_hashrate")]
  #[inline]
  fn hashrate(&self) -> Result<U256> {
    self.client().rpc("eth_hashrate", ())
  }

  /// Returns whether the client is actively mining new blocks.
  ///
  /// Note: This may not be available in some clients since [The Merge].
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_mining")]
  #[inline]
  fn mining(&self) -> Result<bool> {
    self.client().rpc("eth_mining", ())
  }

  /// Used for submitting mining hashrate.
  ///
  /// Note: This may not be available in some clients since [The Merge].
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_submitHashrate")]
  #[inline]
  fn submit_hashrate(&self, hashrate: U256, id: H256) -> Result<bool> {
    self.client().rpc("eth_submitHashrate", (hashrate, id))
  }

  /// Used for submitting a proof-of-work solution.
  ///
  /// Note: This may not be available in some clients since [The Merge].
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[deprecated(since = "TBD")]
  #[doc(alias = "eth_submitWork")]
  #[inline]
  fn submit_work(&self, nonce: H64, hash: H256, digest: H256) -> Result<bool> {
    self.client().rpc("eth_submitWork", (nonce, hash, digest))
  }
}
