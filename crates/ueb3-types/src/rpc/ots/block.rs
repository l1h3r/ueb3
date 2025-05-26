use serde::Deserialize;
use serde::Serialize;

use crate::eth::Wei;
use crate::rpc::Block;
use crate::rpc::Transaction;
use crate::rpc::TransactionReceipt;
use crate::rpc::ots::Issuance;
use crate::rpc::ots::TaggedReceipt;

// -----------------------------------------------------------------------------
// Block Details
// -----------------------------------------------------------------------------

/// An otter-optimized block object and ETH issuance information.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BlockDetails {
  /// The Ethereum block object.
  pub block: Block<()>,
  /// Additional ETH issuance information.
  pub issuance: Issuance,
  /// The sum of fees paid by senders in this block.
  pub total_fees: Wei,
}

// -----------------------------------------------------------------------------
// Block Transactions
// -----------------------------------------------------------------------------

/// An otter-optimized block object and list of transaction receipts.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockTransactions {
  /// The Ethereum block object.
  pub fullblock: Block<Transaction>,
  /// A list of transaction receipts.
  pub receipts: Vec<TransactionReceipt>,
}

// -----------------------------------------------------------------------------
// Block Transaction Search
// -----------------------------------------------------------------------------

/// A list of transaction objects and receipts.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BlockTransactionSearch {
  /// A list of transaction objects.
  #[serde(rename = "txs")]
  pub transactions: Vec<Transaction>,
  /// A list of transaction receipts.
  pub receipts: Vec<TaggedReceipt>,
  /// Whether this is the first page of transaction results.
  pub first_page: bool,
  /// Whether this is the last page of transaction results.
  pub last_page: bool,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip4895::WithdrawalList;
  use crate::eth::Bloom;
  use crate::eth::H64;
  use crate::eth::U256;
  use crate::rpc::Block;
  use crate::rpc::ots::BlockDetails;
  use crate::rpc::ots::Issuance;

  #[test]
  fn test_serde() {
    const BLOOM: &str = "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    const HASH: &str = "0xf05aa8b73b005314684595adcff8e6149917b3239b6316247ce5e88eba9fd3f5";
    const PARENT: &str = "0x0c14a6b05dcd7de00cb87ba149692a49d13593a93563738ec2e2993653191b8c";
    const UNCLES: &str = "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347";
    const TRANSACTIONS: &str = "0xf653129134f8877916d421416a5c24362088f304b00670583cf65c99514d8e85";
    const STATE: &str = "0x203628710422ba4db84d28687504e0f068c51792fe60384c6a1612acd4199aa5";
    const RECEIPTS: &str = "0x9b96f3dab7a5133aa66639f7d7fce8292081f67f4e879835c60ce0889c04cd7f";
    const MIX_HASH: &str = "0x52093d700a6a1c098b2f8528dc11b8ddb94660e208580161645a596c92b2e854";
    const WITHDRAWALS: &str = "0x6e99aa43c101628c9022cca0e7350126659e815fb7caf2f64a90e3c41651f993";

    const VALIDATOR: &str = "0x690b9a9e9aa1c9db991c7721a92d351db4fac990";

    let a: BlockDetails = BlockDetails::default();

    let b: BlockDetails = BlockDetails {
      block: Block {
        hash: HASH.parse().unwrap(),
        number: 0x1106fe7,
        parent_hash: PARENT.parse().unwrap(),
        sha3_uncles: UNCLES.parse().unwrap(),
        transactions_root: TRANSACTIONS.parse().unwrap(),
        state_root: STATE.parse().unwrap(),
        receipts_root: RECEIPTS.parse().unwrap(),
        mix_hash: MIX_HASH.parse().unwrap(),
        gas_used: 0xb2e277,
        gas_limit: 0x1c9c380,
        miner: VALIDATOR.parse().unwrap(),
        nonce: H64::NULL,
        logs_bloom: Bloom::NULL,
        extra_data: "0x6275696c64657230783639".parse().unwrap(),
        size: U256::from(0xd307),
        timestamp: 0x64cf4c1b,
        difficulty: U256::zero(),
        total_difficulty: None,
        transactions: Vec::new(),
        uncles: Vec::new(),
        base_fee_per_gas: Some(0x2e3fa245d),
        parent_beacon_block_root: None,
        blob_gas_used: None,
        excess_blob_gas: None,
        withdrawals_root: Some(WITHDRAWALS.parse().unwrap()),
        withdrawals: Some(WithdrawalList::new()),
        requests_hash: None,
        transaction_count: Some(169),
      },
      issuance: Issuance {
        block_reward: Default::default(),
        uncle_reward: Default::default(),
        issuance: Default::default(),
      },
      total_fees: "0x234719375f04301".parse().unwrap(),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "BlockDetails",
        len: 3,
      },
      Token::Str("block"),
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
      Token::Str(BLOOM),
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
      Token::Str("issuance"),
      Token::Struct {
        name: "Issuance",
        len: 3,
      },
      Token::Str("blockReward"),
      Token::Str("0x0"),
      Token::Str("uncleReward"),
      Token::Str("0x0"),
      Token::Str("issuance"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::Str("totalFees"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "BlockDetails",
        len: 3,
      },
      Token::Str("block"),
      Token::Struct {
        name: "Block",
        len: 23,
      },
      Token::Str("hash"),
      Token::Some,
      Token::Str(HASH),
      Token::Str("number"),
      Token::Str("0x1106fe7"),
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
      Token::Str("0xb2e277"),
      Token::Str("gasLimit"),
      Token::Str("0x1c9c380"),
      Token::Str("miner"),
      Token::Some,
      Token::Str(VALIDATOR),
      Token::Str("nonce"),
      Token::Some,
      Token::Str("0x0000000000000000"),
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(BLOOM),
      Token::Str("extraData"),
      Token::Str("0x6275696c64657230783639"),
      Token::Str("size"),
      Token::Str("0xd307"),
      Token::Str("timestamp"),
      Token::Str("0x64cf4c1b"),
      Token::Str("difficulty"),
      Token::Str("0x0"),
      Token::Str("transactions"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("uncles"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("baseFeePerGas"),
      Token::Some,
      Token::Str("0x2e3fa245d"),
      Token::Str("withdrawalsRoot"),
      Token::Some,
      Token::Str(WITHDRAWALS),
      Token::Str("withdrawals"),
      Token::Some,
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("transactionCount"),
      Token::Some,
      Token::U64(169),
      Token::StructEnd,
      Token::Str("issuance"),
      Token::Struct {
        name: "Issuance",
        len: 3,
      },
      Token::Str("blockReward"),
      Token::Str("0x0"),
      Token::Str("uncleReward"),
      Token::Str("0x0"),
      Token::Str("issuance"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::Str("totalFees"),
      Token::Str("0x234719375f04301"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
