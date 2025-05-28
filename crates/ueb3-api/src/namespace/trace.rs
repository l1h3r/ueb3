use std::collections::HashSet;
use ueb3_rpc::Result;
use ueb3_types::eip::eip1898::BlockId;
use ueb3_types::eth::H256;
use ueb3_types::eth::Slice;
use ueb3_types::eth::U64;
use ueb3_types::rpc::TransactionRequest;
use ueb3_types::rpc::trace;

use crate::provider::Provider;

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: Provider {}
impl<T> NsTrace for T where T: Provider {}

// -----------------------------------------------------------------------------
// Namespace (trace)
// -----------------------------------------------------------------------------

/// Provides access to the non-standard `trace` namespace.
pub trait NsTrace: private::Sealed + Provider {
  /// Executes the given call and returns a number of possible traces for it.
  #[doc(alias = "trace_call")]
  #[inline]
  fn call(
    &self,
    call: TransactionRequest,
    types: HashSet<trace::TraceType>,
    block: Option<BlockId>,
  ) -> Result<trace::TraceCallResult> {
    self
      .client()
      .rpc("trace_call", (call, types, block.unwrap_or_default()))
  }

  /// Performs multiple call traces on top of the same block.
  ///
  /// i.e. transaction `n` will be executed on top of a pending block with all
  /// `n-1` transactions applied (traced) first.
  #[doc(alias = "trace_callMany")]
  #[inline]
  fn call_many(
    &self,
    calls: &[(TransactionRequest, HashSet<trace::TraceType>)],
    block: Option<BlockId>,
  ) -> Result<Vec<trace::TraceCallResult>> {
    self
      .client()
      .rpc("trace_callMany", (calls, block.unwrap_or_default()))
  }

  /// Traces a call to `eth_sendRawTransaction` without making the call,
  /// returning the traces.
  #[doc(alias = "trace_rawTransaction")]
  #[inline]
  fn raw_transaction(
    &self,
    transaction: &Slice,
    types: HashSet<trace::TraceType>,
  ) -> Result<trace::TraceCallResult> {
    self
      .client()
      .rpc("trace_rawTransaction", (transaction, types))
  }

  /// Replays all transactions in a block returning the requested traces for each transaction.
  #[doc(alias = "trace_replayBlockTransactions")]
  #[inline]
  fn replay_block_transactions(
    &self,
    block: BlockId,
    types: HashSet<trace::TraceType>,
  ) -> Result<Option<Vec<trace::TraceCallResult>>> {
    self
      .client()
      .rpc("trace_replayBlockTransactions", (block, types))
  }

  /// Replays a transaction, returning the traces.
  #[doc(alias = "trace_replayTransaction")]
  #[inline]
  fn replay_transaction(
    &self,
    transaction: H256,
    types: HashSet<trace::TraceType>,
  ) -> Result<Option<trace::TraceCallResult>> {
    self
      .client()
      .rpc("trace_replayTransaction", (transaction, types))
  }

  /// Returns traces created at a given block.
  #[doc(alias = "trace_block")]
  #[inline]
  fn block(&self, block: BlockId) -> Result<Option<Vec<trace::ParityTrace>>> {
    self.client().rpc("trace_block", (block,))
  }

  /// Returns traces matching the given filter.
  #[doc(alias = "trace_filter")]
  #[inline]
  fn filter(&self, request: trace::TraceFilterRequest) -> Result<Option<Vec<trace::ParityTrace>>> {
    self.client().rpc("trace_filter", (request,))
  }

  /// Returns traces at a given position.
  #[doc(alias = "trace_get")]
  #[inline]
  fn get(&self, transaction: H256, positions: &[u64]) -> Result<Option<trace::ParityTrace>> {
    fn encode(positions: &[u64]) -> Vec<U64> {
      positions.iter().copied().map(U64::from_u64).collect()
    }

    self
      .client()
      .rpc("trace_get", (transaction, encode(positions)))
  }

  /// Returns all traces of a given transaction.
  #[doc(alias = "trace_transaction")]
  #[inline]
  fn transaction(&self, transaction: H256) -> Result<Option<Vec<trace::ParityTrace>>> {
    self.client().rpc("trace_transaction", (transaction,))
  }

  // ---------------------------------------------------------------------------
  // Reth Extensions
  // ---------------------------------------------------------------------------

  /// Returns the opcode gas usage of all transactions in the given block.
  #[doc(alias = "trace_blockOpcodeGas")]
  #[inline]
  fn block_opcode_gas(&self, block: BlockId) -> Result<Option<trace::BlockOpcodeGas>> {
    self.client().rpc("trace_blockOpcodeGas", (block,))
  }

  /// Returns all opcodes with their count and combined gas usage
  /// for the given transaction in no particular order.
  #[doc(alias = "trace_transactionOpcodeGas")]
  #[inline]
  fn transaction_opcode_gas(
    &self,
    transaction: H256,
  ) -> Result<Option<trace::TransactionOpcodeGas>> {
    self
      .client()
      .rpc("trace_transactionOpcodeGas", (transaction,))
  }
}
