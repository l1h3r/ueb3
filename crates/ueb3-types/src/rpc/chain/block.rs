use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eip::eip4895::WithdrawalList;
use crate::eth::Address;
use crate::eth::Bloom;
use crate::eth::Bytes;
use crate::eth::H64;
use crate::eth::H256;
use crate::eth::U256;
use crate::rpc::Transaction;

mod private {
  pub trait Sealed {}
}

pub trait TransactionData: private::Sealed {}

impl private::Sealed for () {}
impl TransactionData for () {}

impl private::Sealed for H256 {}
impl TransactionData for H256 {}

impl private::Sealed for Transaction {}
impl TransactionData for Transaction {}

// -----------------------------------------------------------------------------
// Ethereum Block
// -----------------------------------------------------------------------------

/// A block object.
#[derive(Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Block<T: TransactionData> {
  /// The hash of the block.
  ///
  /// Note: This can be [`H256::NULL`] in the following additional cases:
  ///
  ///  * Retrieving pending blocks from certain nodes
  #[serde(deserialize_with = "ueb3_serde::nullable::default")]
  pub hash: H256,
  /// The block number.
  #[serde(with = "ueb3_serde::quantity")]
  pub number: u64,
  /// The hash of the parent block.
  pub parent_hash: H256,
  /// The Keccak hash of the [`uncles`][Self::uncles] data.
  pub sha3_uncles: H256,
  /// The root hash of the transaction trie of the block.
  pub transactions_root: H256,
  /// The root hash of the final state trie of the block.
  pub state_root: H256,
  /// The root hash of the receipts trie of the block.
  pub receipts_root: H256,
  /// A 256-bit hash used for proof-of-work validation.
  ///
  /// Note: This field was originally deprecated after [The Merge] but
  /// [EIP-4399] repurposed the field as `prevRandao`.
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  /// [EIP-4399]: https://eips.ethereum.org/EIPS/eip-4399
  pub mix_hash: H256,
  /// The total amount of gas used by all transactions in the block.
  #[serde(with = "ueb3_serde::quantity")]
  pub gas_used: u64,
  /// The maximum amount of gas allowed in the block.
  #[serde(with = "ueb3_serde::quantity")]
  pub gas_limit: u64,
  /// The address that collects all fees resulting from a successful inclusion
  /// of the block.
  ///
  /// Note: This can be [`Address::NULL`] in the following additional cases:
  ///
  ///  * Retrieving pending blocks from certain nodes
  #[serde(deserialize_with = "ueb3_serde::nullable::default")]
  pub miner: Address,
  /// A 64-bit hash used for proof-of-work validation.
  ///
  /// Note: This can be [`H64::NULL`] in the following additional cases:
  ///
  ///  * Retrieving pending blocks from certain nodes
  #[serde(deserialize_with = "ueb3_serde::nullable::default")]
  pub nonce: H64,
  /// The bloom filter for block logs.
  ///
  /// Note: This can be [`Bloom::NULL`] in the following additional cases:
  ///
  ///  * Retrieving blocks with `ots_getBlockDetails`
  ///  * Retrieving blocks with `ots_getBlockDetailsByHash`
  ///  * Retrieving blocks with `ots_getBlockTransactions`
  #[serde(deserialize_with = "ueb3_serde::nullable::default")]
  pub logs_bloom: Bloom,
  /// Additional data included with the block.
  pub extra_data: Bytes,
  /// The size of the block (in bytes).
  ///
  /// Note: This can be `0` in the following additional cases:
  ///
  ///  * Retrieving blocks with `erigon_getHeaderByHash`
  ///  * Retrieving blocks with `erigon_getHeaderByNumber`
  #[serde(default = "U256::zero")]
  pub size: U256,
  /// The unix timestamp of the block.
  #[serde(with = "ueb3_serde::quantity")]
  pub timestamp: u64,
  /// The difficulty level of the block.
  pub difficulty: U256,
  /// The total difficulty of the chain until the block.
  ///
  /// Note: This field was removed in [The Merge]
  ///
  /// [The Merge]: https://ethereum.org/en/roadmap/merge
  #[serde(skip_serializing_if = "Option::is_none")]
  pub total_difficulty: Option<U256>,
  /// An array of transactions (objects or hashes) included in this block.
  ///
  /// Note: This can be empty in the following additional cases:
  ///
  ///  * Retrieving uncles with `eth_getUncleByBlockHashAndIndex`
  ///  * Retrieving uncles with `eth_getUncleByBlockNumberAndIndex`
  ///  * Retrieving blocks with `ots_getBlockDetails`
  ///  * Retrieving blocks with `ots_getBlockDetailsByHash`
  ///  * Retrieving blocks with `erigon_getHeaderByHash`
  ///  * Retrieving blocks with `erigon_getHeaderByNumber`
  #[serde(default = "Vec::new")]
  pub transactions: Vec<T>,
  /// An array of uncle hashes.
  ///
  /// Note: This can be empty in the following additional cases:
  ///
  ///  * Retrieving blocks with `erigon_getHeaderByHash`
  ///  * Retrieving blocks with `erigon_getHeaderByNumber`
  #[serde(default = "Vec::new")]
  pub uncles: Vec<H256>,
  /// The base gas fee per unit of gas.
  ///
  /// Only specified for blocks included after [EIP-1559].
  ///
  /// [EIP-1559]: https://eips.ethereum.org/EIPS/eip-1559
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub base_fee_per_gas: Option<u64>,
  /// The root hash of the parent beacon block.
  ///
  /// Only specified for blocks included after [EIP-4788].
  ///
  /// [EIP-4788]: https://eips.ethereum.org/EIPS/eip-4788
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent_beacon_block_root: Option<H256>,
  /// The total amount of blob gas used by all transactions in the block.
  ///
  /// Only specified for blocks included after [EIP-4844].
  ///
  /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub blob_gas_used: Option<u64>,
  /// A running total of excess blob gas consumed prior to the block.
  ///
  /// Only specified for blocks included after [EIP-4844].
  ///
  /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub excess_blob_gas: Option<u64>,
  /// The Keccak hash of the [`withdrawals`][Self::withdrawals] data.
  ///
  /// Only specified for blocks included after [EIP-4895].
  ///
  /// [EIP-4895]: https://eips.ethereum.org/EIPS/eip-4895
  #[serde(skip_serializing_if = "Option::is_none")]
  pub withdrawals_root: Option<H256>,
  /// Validator withdrawals included in the block.
  ///
  /// Only specified for blocks included after [EIP-4895].
  ///
  /// [EIP-4895]: https://eips.ethereum.org/EIPS/eip-4895
  #[serde(skip_serializing_if = "Option::is_none")]
  pub withdrawals: Option<WithdrawalList>,
  /// The commitment hash for a list of request objects.
  ///
  /// Only specified for blocks included after [EIP-7685].
  ///
  /// [EIP-7685]: https://eips.ethereum.org/EIPS/eip-7685
  #[serde(skip_serializing_if = "Option::is_none")]
  pub requests_hash: Option<H256>,
}

impl Block<Transaction> {
  /// Converts `self` into a block of transaction hashes.
  #[inline]
  pub fn flatten(self) -> Block<H256> {
    Block {
      hash: self.hash,
      number: self.number,
      parent_hash: self.parent_hash,
      sha3_uncles: self.sha3_uncles,
      transactions_root: self.transactions_root,
      state_root: self.state_root,
      receipts_root: self.receipts_root,
      mix_hash: self.mix_hash,
      gas_used: self.gas_used,
      gas_limit: self.gas_limit,
      miner: self.miner,
      nonce: self.nonce,
      logs_bloom: self.logs_bloom,
      extra_data: self.extra_data,
      size: self.size,
      timestamp: self.timestamp,
      difficulty: self.difficulty,
      total_difficulty: self.total_difficulty,
      transactions: Self::flatten_txns(self.transactions),
      uncles: self.uncles,
      base_fee_per_gas: self.base_fee_per_gas,
      parent_beacon_block_root: self.parent_beacon_block_root,
      blob_gas_used: self.blob_gas_used,
      excess_blob_gas: self.excess_blob_gas,
      withdrawals_root: self.withdrawals_root,
      withdrawals: self.withdrawals,
      requests_hash: self.requests_hash,
    }
  }

  #[inline]
  fn flatten_txns(transactions: Vec<Transaction>) -> Vec<H256> {
    transactions
      .into_iter()
      .map(|transaction| transaction.header.hash)
      .collect()
  }
}

impl<T: TransactionData> Default for Block<T> {
  #[inline]
  fn default() -> Self {
    Self {
      hash: H256::default(),
      number: 0,
      parent_hash: H256::default(),
      sha3_uncles: H256::default(),
      transactions_root: H256::default(),
      state_root: H256::default(),
      receipts_root: H256::default(),
      mix_hash: H256::default(),
      gas_used: 0,
      gas_limit: 0,
      miner: Address::default(),
      nonce: H64::default(),
      logs_bloom: Bloom::default(),
      extra_data: Bytes::default(),
      size: U256::default(),
      timestamp: 0,
      difficulty: U256::default(),
      total_difficulty: None,
      transactions: Vec::new(),
      uncles: Vec::new(),
      base_fee_per_gas: None,
      parent_beacon_block_root: None,
      blob_gas_used: None,
      excess_blob_gas: None,
      withdrawals_root: None,
      withdrawals: None,
      requests_hash: None,
    }
  }
}

impl<T> Debug for Block<T>
where
  T: TransactionData + Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Block")
      .field("hash", &self.hash)
      .field("number", &self.number)
      .field("parent_hash", &self.parent_hash)
      .field("sha3_uncles", &self.sha3_uncles)
      .field("transactions_root", &self.transactions_root)
      .field("state_root", &self.state_root)
      .field("receipts_root", &self.receipts_root)
      .field("mix_hash", &self.mix_hash)
      .field("gas_used", &self.gas_used)
      .field("gas_limit", &self.gas_limit)
      .field("miner", &self.miner)
      .field("nonce", &self.nonce)
      .field("logs_bloom", &self.logs_bloom)
      .field("extra_data", &self.extra_data)
      .field("size", &self.size)
      .field("timestamp", &self.timestamp)
      .field("difficulty", &self.difficulty)
      .field("total_difficulty", &self.total_difficulty.format())
      .field("transactions", &self.transactions)
      .field("uncles", &self.uncles)
      .field("base_fee_per_gas", &self.base_fee_per_gas.format())
      .field("parent_beacon_block_root", &self.parent_beacon_block_root.format())
      .field("blob_gas_used", &self.blob_gas_used.format())
      .field("excess_blob_gas", &self.excess_blob_gas.format())
      .field("withdrawals_root", &self.withdrawals_root.format())
      .field("withdrawals", &self.withdrawals.format())
      .field("requests_hash", &self.requests_hash.format())
      .finish()
  }
}

impl From<Block<Transaction>> for Block<H256> {
  #[inline]
  fn from(other: Block<Transaction>) -> Self {
    other.flatten()
  }
}
