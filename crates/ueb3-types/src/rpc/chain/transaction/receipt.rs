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
  #[serde(with = "ueb3_serde::nullable::sequence")]
  pub logs: Vec<TransactionLog>,
  /// The bloom filter for log object.
  ///
  /// Note: This can be empty in the following additional cases:
  ///
  ///  * Retrieving receipts with `ots_getBlockTransactions`
  #[serde(with = "ueb3_serde::nullable::default")]
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
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub status: Option<u8>,
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip2718::TransactionType;
  use crate::eth::Bloom;
  use crate::eth::U256;
  use crate::eth::Wei;
  use crate::rpc::TransactionLog;
  use crate::rpc::TransactionReceipt;

  #[test]
  fn test_serde() {
    const TXN_HASH_A: &str = "0xefb83b4e3f1c317e8da0f8e2fbb2fe964f34ee184466032aeecac79f20eacaf6";
    const TXN_HASH_B: &str = "0x4a481e4649da999d92db0585c36cba94c18a33747e95dc235330e6c737c6f975";

    const BLOCK_HASH: &str = "0x19514ce955c65e4dd2cd41f435a75a46a08535b8fc16bc660f8092b32590b182";

    const FROM_A: &str = "0x712e3a792c974b3e3dbe41229ad4290791c75a82";
    const FROM_B: &str = "0x22896bfc68814bfd855b1a167255ee497006e730";

    const TO_A: &str = "0xd42e2b1c14d02f1df5369a9827cb8e6f3f75f338";
    const TO_B: &str = "0xfd584430cafa2f451b4e2ebcf3986a21fff04350";

    const LOGS_BLOOM_A: &str = "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const LOGS_BLOOM_B: &str = "0x00000004000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000080020000000000000200010000000000000000000001000000800000000000000000000000000000000000000000000000000000100100000000000000000000008000000000000000000000000000000002000000000000000000000";

    const TOPIC_O: &str = "0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d";
    const TOPIC_1: &str = "0x4be29e0e4eb91f98f709d98803cba271592782e293b84a625e025cbb40197ba8";
    const TOPIC_2: &str = "0x000000000000000000000000835281a2563db4ebf1b626172e085dc406bfc7d2";
    const TOPIC_3: &str = "0x00000000000000000000000022896bfc68814bfd855b1a167255ee497006e730";

    let a: TransactionReceipt = TransactionReceipt {
      transaction_type: Some(TransactionType::Eip1559),
      transaction_hash: TXN_HASH_A.parse().unwrap(),
      transaction_index: 0x1,
      block_hash: BLOCK_HASH.parse().unwrap(),
      block_number: 0x6f55,
      from: FROM_A.parse().unwrap(),
      to: Some(TO_A.parse().unwrap()),
      contract_address: None,
      gas_used: U256::from(0x5208),
      cumulative_gas_used: U256::from(0x1de3e),
      effective_gas_price: Wei::from(U256::from(0x9502f907_u64)),
      logs: Vec::new(),
      logs_bloom: Bloom::NULL,
      root: None,
      status: Some(0x1),
      blob_gas_used: Some(U256::from(0x20000)),
      blob_gas_price: Some(U256::from(0x32)),
    };

    let b: TransactionReceipt = TransactionReceipt {
      transaction_type: Some(TransactionType::Legacy),
      transaction_hash: TXN_HASH_B.parse().unwrap(),
      transaction_index: 0x0,
      block_hash: BLOCK_HASH.parse().unwrap(),
      block_number: 0x6f55,
      from: FROM_B.parse().unwrap(),
      to: Some(TO_B.parse().unwrap()),
      contract_address: None,
      gas_used: U256::from(0x18c36),
      cumulative_gas_used: U256::from(0x18c36),
      effective_gas_price: Wei::from(U256::from(0x9502f907_u64)),
      logs: vec![TransactionLog {
        log_index: Some(0x0),
        transaction_hash: Some(TXN_HASH_B.parse().unwrap()),
        transaction_index: Some(0x0),
        block_hash: Some(BLOCK_HASH.parse().unwrap()),
        block_number: Some(0x6f55),
        address: TO_B.parse().unwrap(),
        removed: false,
        topics: vec![
          TOPIC_O.parse().unwrap(),
          TOPIC_1.parse().unwrap(),
          TOPIC_2.parse().unwrap(),
          TOPIC_3.parse().unwrap(),
        ],
        data: "0x".parse().unwrap(),
        block_timestamp: None,
      }],
      logs_bloom: LOGS_BLOOM_B.parse().unwrap(),
      root: None,
      status: Some(0x1),
      blob_gas_used: Some(U256::from(0x20000)),
      blob_gas_price: Some(U256::from(0x32)),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionReceipt",
        len: 16,
      },
      Token::Str("type"),
      Token::Some,
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x2",
      },
      Token::Str("transactionHash"),
      Token::Str(TXN_HASH_A),
      Token::Str("transactionIndex"),
      Token::Str("0x1"),
      Token::Str("blockHash"),
      Token::Str(BLOCK_HASH),
      Token::Str("blockNumber"),
      Token::Str("0x6f55"),
      Token::Str("from"),
      Token::Str(FROM_A),
      Token::Str("to"),
      Token::Some,
      Token::Str(TO_A),
      Token::Str("contractAddress"),
      Token::None,
      Token::Str("gasUsed"),
      Token::Str("0x5208"),
      Token::Str("cumulativeGasUsed"),
      Token::Str("0x1de3e"),
      Token::Str("effectiveGasPrice"),
      Token::Str("0x9502f907"),
      Token::Str("logs"),
      Token::Some,
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(LOGS_BLOOM_A),
      Token::Str("status"),
      Token::Some,
      Token::Str("0x1"),
      Token::Str("blobGasUsed"),
      Token::Some,
      Token::Str("0x20000"),
      Token::Str("blobGasPrice"),
      Token::Some,
      Token::Str("0x32"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionReceipt",
        len: 16,
      },
      Token::Str("type"),
      Token::Some,
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x0",
      },
      Token::Str("transactionHash"),
      Token::Str(TXN_HASH_B),
      Token::Str("transactionIndex"),
      Token::Str("0x0"),
      Token::Str("blockHash"),
      Token::Str(BLOCK_HASH),
      Token::Str("blockNumber"),
      Token::Str("0x6f55"),
      Token::Str("from"),
      Token::Str(FROM_B),
      Token::Str("to"),
      Token::Some,
      Token::Str(TO_B),
      Token::Str("contractAddress"),
      Token::None,
      Token::Str("gasUsed"),
      Token::Str("0x18c36"),
      Token::Str("cumulativeGasUsed"),
      Token::Str("0x18c36"),
      Token::Str("effectiveGasPrice"),
      Token::Str("0x9502f907"),
      Token::Str("logs"),
      Token::Some,
      Token::Seq { len: Some(1) },
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
      Token::Str(BLOCK_HASH),
      Token::Str("blockNumber"),
      Token::Some,
      Token::Str("0x6f55"),
      Token::Str("address"),
      Token::Str(TO_B),
      Token::Str("removed"),
      Token::Bool(false),
      Token::Str("topics"),
      Token::Some,
      Token::Seq { len: Some(4) },
      Token::Str(TOPIC_O),
      Token::Str(TOPIC_1),
      Token::Str(TOPIC_2),
      Token::Str(TOPIC_3),
      Token::SeqEnd,
      Token::Str("data"),
      Token::Str("0x"),
      Token::StructEnd,
      Token::SeqEnd,
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(LOGS_BLOOM_B),
      Token::Str("status"),
      Token::Some,
      Token::Str("0x1"),
      Token::Str("blobGasUsed"),
      Token::Some,
      Token::Str("0x20000"),
      Token::Str("blobGasPrice"),
      Token::Some,
      Token::Str("0x32"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
