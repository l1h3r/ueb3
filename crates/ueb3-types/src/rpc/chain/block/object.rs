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
  #[serde(with = "ueb3_serde::nullable::default")]
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
  #[serde(with = "ueb3_serde::nullable::default")]
  pub miner: Address,
  /// A 64-bit hash used for proof-of-work validation.
  ///
  /// Note: This can be [`H64::NULL`] in the following additional cases:
  ///
  ///  * Retrieving pending blocks from certain nodes
  #[serde(with = "ueb3_serde::nullable::default")]
  pub nonce: H64,
  /// The bloom filter for block logs.
  ///
  /// Note: This can be [`Bloom::NULL`] in the following additional cases:
  ///
  ///  * Retrieving blocks with `ots_getBlockDetails`
  ///  * Retrieving blocks with `ots_getBlockDetailsByHash`
  ///  * Retrieving blocks with `ots_getBlockTransactions`
  #[serde(with = "ueb3_serde::nullable::default")]
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip4895::Withdrawal;
  use crate::eip::eip4895::WithdrawalList;
  use crate::eth::H256;
  use crate::rpc::Block;

  #[test]
  fn test_serde() {
    const BLOOM_A: &str = "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const BLOOM_B: &str = "0x00000000000000100000004080000000000500000000000000020000100000000800001000000004000001000000000000000800040010000020100000000400000010000000000000000040000000000000040000000000000000000000000000000400002400000000000000000000000000000004000004000000000000840000000800000080010004000000001000000800000000000000000000000000000000000800000000000040000000020000000000000000000800000400000000000000000000000600000400000000002000000000000000000000004000000000000000100000000000000000000000000000000000040000900010000000";

    const HASH: &str = "0xd5f1812548be429cbdc6376b29611fc49e06f1359758c4ceaaa3b393e2239f9c";
    const PARENT: &str = "0x1f68ac259155e2f38211ddad0f0a15394d55417b185a93923e2abe71bb7a4d6d";
    const UNCLES: &str = "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347";
    const TRANSACTIONS: &str = "0x4d0c8e91e16bdff538c03211c5c73632ed054d00a7e210c0eb25146c20048126";
    const STATE: &str = "0x91309efa7e42c1f137f31fe9edbe88ae087e6620d0d59031324da3e2f4f93233";
    const RECEIPTS: &str = "0x68461ab700003503a305083630a8fb8d14927238f0bc8b6b3d246c0c64f21f4a";
    const MIX_HASH: &str = "0x24900fb3da77674a861c428429dce0762707ecb6052325bbd9b3c64e74b5af9d";
    const BEACON: &str = "0x95c4dbd5b19f6fe3cbc3183be85ff4e85ebe75c5b4fc911f1c91e5b7a554a685";
    const WITHDRAWALS: &str = "0x7a4ecf19774d15cf9c15adf0dd8e8a250c128b26c9e2ab2a08d6c9c8ffbd104f";

    const EXTRA_DATA: &str = "0xd583010502846765746885676f312e37856c696e7578";

    const VALIDATOR: &str = "0xb42b6c4a95406c78ff892d270ad20b22642e102d";
    const RECIPIENT: &str = "0xb9d7934878b5fb9610b3fe8a5e441e8fad7e293f";

    const TXN_1: &str = "0xa0807e117a8dd124ab949f460f08c36c72b710188f01609595223b325e58e0fc";
    const TXN_2: &str = "0xeae6d797af50cb62a596ec3939114d63967c374fa57de9bc0f4e2b576ed6639d";

    let a: Block<()> = Block::default();

    let b: Block<H256> = Block {
      hash: HASH.parse().unwrap(),
      number: 0x68b3,
      parent_hash: PARENT.parse().unwrap(),
      sha3_uncles: UNCLES.parse().unwrap(),
      transactions_root: TRANSACTIONS.parse().unwrap(),
      state_root: STATE.parse().unwrap(),
      receipts_root: RECEIPTS.parse().unwrap(),
      mix_hash: MIX_HASH.parse().unwrap(),
      gas_used: 0x37993,
      gas_limit: 0x47e7c4,
      miner: VALIDATOR.parse().unwrap(),
      nonce: "0x378da40ff335b070".parse().unwrap(),
      logs_bloom: BLOOM_B.parse().unwrap(),
      extra_data: EXTRA_DATA.parse().unwrap(),
      size: "0x334".parse().unwrap(),
      timestamp: 0x5835c54d,
      difficulty: "0x66e619a".parse().unwrap(),
      total_difficulty: Some("0x1e875d746ae".parse().unwrap()),
      transactions: vec![TXN_1.parse().unwrap(), TXN_2.parse().unwrap()],
      uncles: Vec::new(),
      base_fee_per_gas: Some(0x7),
      parent_beacon_block_root: Some(BEACON.parse().unwrap()),
      blob_gas_used: Some(0),
      excess_blob_gas: Some(0),
      withdrawals_root: Some(WITHDRAWALS.parse().unwrap()),
      withdrawals: Some(WithdrawalList::from([Withdrawal {
        index: 0x0,
        validator: 0x9d8c0,
        address: RECIPIENT.parse().unwrap(),
        amount: 0x11a33e3760,
      }])),
      requests_hash: None,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Block",
        len: 19,
      },
      Token::Str("hash"),
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("number"),
      Token::Str("0x0"),
      Token::Str("parentHash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("sha3Uncles"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("transactionsRoot"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("stateRoot"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("receiptsRoot"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("mixHash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("gasUsed"),
      Token::Str("0x0"),
      Token::Str("gasLimit"),
      Token::Str("0x0"),
      Token::Str("miner"),
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("nonce"),
      Token::Some,
      Token::Str("0x0000000000000000"),
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(BLOOM_A),
      Token::Str("extraData"),
      Token::Str("0x"),
      Token::Str("size"),
      Token::Str("0x0"),
      Token::Str("timestamp"),
      Token::Str("0x0"),
      Token::Str("difficulty"),
      Token::Str("0x0"),
      Token::Str("transactions"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("uncles"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Block",
        len: 26,
      },
      Token::Str("hash"),
      Token::Some,
      Token::Str(HASH),
      Token::Str("number"),
      Token::Str("0x68b3"),
      Token::Str("parentHash"),
      Token::Str(PARENT),
      Token::Str("sha3Uncles"),
      Token::Str(UNCLES),
      Token::Str("transactionsRoot"),
      Token::Str(TRANSACTIONS),
      Token::Str("stateRoot"),
      Token::Str(STATE),
      Token::Str("receiptsRoot"),
      Token::Str(RECEIPTS),
      Token::Str("mixHash"),
      Token::Str(MIX_HASH),
      Token::Str("gasUsed"),
      Token::Str("0x37993"),
      Token::Str("gasLimit"),
      Token::Str("0x47e7c4"),
      Token::Str("miner"),
      Token::Some,
      Token::Str(VALIDATOR),
      Token::Str("nonce"),
      Token::Some,
      Token::Str("0x378da40ff335b070"),
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(BLOOM_B),
      Token::Str("extraData"),
      Token::Str(EXTRA_DATA),
      Token::Str("size"),
      Token::Str("0x334"),
      Token::Str("timestamp"),
      Token::Str("0x5835c54d"),
      Token::Str("difficulty"),
      Token::Str("0x66e619a"),
      Token::Str("totalDifficulty"),
      Token::Some,
      Token::Str("0x1e875d746ae"),
      Token::Str("transactions"),
      Token::Seq { len: Some(2) },
      Token::Str(TXN_1),
      Token::Str(TXN_2),
      Token::SeqEnd,
      Token::Str("uncles"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("baseFeePerGas"),
      Token::Some,
      Token::Str("0x7"),
      Token::Str("parentBeaconBlockRoot"),
      Token::Some,
      Token::Str(BEACON),
      Token::Str("blobGasUsed"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("excessBlobGas"),
      Token::Some,
      Token::Str("0x0"),
      Token::Str("withdrawalsRoot"),
      Token::Some,
      Token::Str(WITHDRAWALS),
      Token::Str("withdrawals"),
      Token::Some,
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "Withdrawal",
        len: 4,
      },
      Token::Str("index"),
      Token::Str("0x0"),
      Token::Str("validatorIndex"),
      Token::Str("0x9d8c0"),
      Token::Str("address"),
      Token::Str(RECIPIENT),
      Token::Str("amount"),
      Token::Str("0x11a33e3760"),
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
