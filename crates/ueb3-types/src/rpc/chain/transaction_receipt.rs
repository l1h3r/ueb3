use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eip::eip2718::TransactionType;
use crate::eth::Address;
use crate::eth::Bloom;
use crate::eth::H256;
use crate::eth::U64;
use crate::eth::U256;
use crate::eth::Wei;
use crate::rpc::TransactionLog;

// -----------------------------------------------------------------------------
// Ethereum Transaction Receipt
// -----------------------------------------------------------------------------

/// A transaction receipt object.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionReceipt {
  /// The transaction type identifier.
  ///
  /// Only specified for transactions included after [EIP-2718].
  ///
  /// [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub transaction_type: Option<TransactionType>,
  /// The hash of the transaction.
  pub transaction_hash: H256,
  /// The index in the block that includes the transaction.
  #[serde(with = "ueb3_serde::quantity")]
  pub transaction_index: u64,
  /// The hash of the block that includes the transaction.
  pub block_hash: H256,
  /// The number of the block that includes the transaction.
  #[serde(with = "ueb3_serde::quantity")]
  pub block_number: u64,
  /// The address of the transaction sender.
  pub from: Address,
  /// The address of the transaction receiver. `None` when contract creation.
  pub to: Option<Address>,
  /// The contract address, if one was created by the transaction.
  pub contract_address: Option<Address>,
  /// The amount of gas used for this specific transaction alone.
  pub gas_used: U256,
  /// The sum of gas used by this transaction and all preceding transactions in
  /// the same block.
  pub cumulative_gas_used: U256,
  /// The actual value per gas deducted from the sender's account. Before
  /// [EIP-1559], this is equal to the transaction's gas price. After, it is
  /// equal to `baseFeePerGas + min(maxFeePerGas - baseFeePerGas, maxPriorityFeePerGas)`.
  ///
  /// [EIP-1559]: https://eips.ethereum.org/EIPS/eip-1559
  pub effective_gas_price: Wei,
  /// An array of log objects emitted by the transaction.
  ///
  /// Note: This can be empty in the following additional cases:
  ///
  ///  * Retrieving receipts with `ots_getBlockTransactions`
  #[serde(deserialize_with = "ueb3_serde::nullable::sequence")]
  pub logs: Vec<TransactionLog>,
  /// The bloom filter for log object.
  ///
  /// Note: This can be empty in the following additional cases:
  ///
  ///  * Retrieving receipts with `ots_getBlockTransactions`
  #[serde(deserialize_with = "ueb3_serde::nullable::default")]
  pub logs_bloom: Bloom,
  /// The post-transaction state root.
  ///
  /// Only specified for transactions included before [EIP-658].
  ///
  /// [EIP-658]: https://eips.ethereum.org/EIPS/eip-658
  #[serde(skip_serializing_if = "Option::is_none")]
  pub root: Option<H256>,
  /// Either 1 (success) or 0 (failure).
  ///
  /// Only specified for transactions included after [EIP-658].
  ///
  /// [EIP-658]: https://eips.ethereum.org/EIPS/eip-658
  #[serde(skip_serializing_if = "Option::is_none")]
  pub status: Option<U64>,
  /// The amount of blob gas used for this specific transaction.
  ///
  /// Only specified for blob transactions as defined by [EIP-4844].
  ///
  /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blob_gas_used: Option<U256>,
  /// The actual value per gas deducted from the sender's account for blob gas.
  ///
  /// Only specified for blob transactions as defined by [EIP-4844].
  ///
  /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blob_gas_price: Option<U256>,
}

impl Debug for TransactionReceipt {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionReceipt")
      .field("transaction_type", &self.transaction_type.format())
      .field("transaction_hash", &self.transaction_hash)
      .field("transaction_index", &self.transaction_index)
      .field("block_hash", &self.block_hash)
      .field("block_number", &self.block_number)
      .field("from", &self.from)
      .field("to", &self.to.format())
      .field("contract_address", &self.contract_address.format())
      .field("gas_used", &self.gas_used)
      .field("cumulative_gas_used", &self.cumulative_gas_used)
      .field("effective_gas_price", &self.effective_gas_price)
      .field("logs", &self.logs)
      .field("logs_bloom", &self.logs_bloom)
      .field("root", &self.root.format())
      .field("status", &self.status.format())
      .field("blob_gas_used", &self.blob_gas_used.format())
      .field("blob_gas_price", &self.blob_gas_price.format())
      .finish()
  }
}
