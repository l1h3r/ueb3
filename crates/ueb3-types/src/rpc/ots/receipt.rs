use serde::Deserialize;
use serde::Serialize;

use crate::rpc::TransactionReceipt;

// -----------------------------------------------------------------------------
// Timestamp-Tagged Receipt
// -----------------------------------------------------------------------------

/// An otter-optimized receipt object.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaggedReceipt {
  /// The full RPC receipt object.
  #[serde(flatten)]
  pub receipt: TransactionReceipt,
  /// The unix timestamp of the block that included the transaction.
  pub timestamp: u64,
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
  use crate::rpc::TransactionReceipt;
  use crate::rpc::ots::TaggedReceipt;

  #[test]
  fn test_serde() {
    const SEND: &str = "0x690b9a9e9aa1c9db991c7721a92d351db4fac990";
    const RECV: &str = "0x5454c37c079c48f9f142fdfd8f937768532eea57";

    const BLOOM: &str = "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
    const T_HASH: &str = "0x9eadc18d110c5d322afe3586bc95bf8b3bb5ad2fcabe2c0cfccbe8258d86f60b";
    const B_HASH: &str = "0xcfec5aa6babeac4e1c195f13eb31443f29a758cf90176b4706dd00603d20e58c";

    let a: TaggedReceipt = TaggedReceipt::default();

    let b: TaggedReceipt = TaggedReceipt {
      timestamp: 1745829383,
      receipt: TransactionReceipt {
        transaction_type: Some(TransactionType::Eip1559),
        transaction_hash: T_HASH.parse().unwrap(),
        transaction_index: 0x53,
        block_hash: B_HASH.parse().unwrap(),
        block_number: 0x15548cc,
        from: SEND.parse().unwrap(),
        to: Some(RECV.parse().unwrap()),
        contract_address: None,
        gas_used: "0x5208".parse().unwrap(),
        cumulative_gas_used: "0x6ccdad".parse().unwrap(),
        effective_gas_price: "0x1b9aea77".parse().unwrap(),
        logs: Vec::new(),
        logs_bloom: BLOOM.parse().unwrap(),
        root: None,
        status: Some(0x1),
        blob_gas_used: None,
        blob_gas_price: None,
      },
    };

    let tokens_a: &[Token] = &[
      Token::Map { len: None },
      Token::Str("transactionHash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("transactionIndex"),
      Token::Str("0x0"),
      Token::Str("blockHash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("blockNumber"),
      Token::Str("0x0"),
      Token::Str("from"),
      Token::Str("0x0000000000000000000000000000000000000000"),
      Token::Str("to"),
      Token::None,
      Token::Str("contractAddress"),
      Token::None,
      Token::Str("gasUsed"),
      Token::Str("0x0"),
      Token::Str("cumulativeGasUsed"),
      Token::Str("0x0"),
      Token::Str("effectiveGasPrice"),
      Token::Str("0x0"),
      Token::Str("logs"),
      Token::Some,
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(BLOOM),
      Token::Str("timestamp"),
      Token::U64(0),
      Token::MapEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Map { len: None },
      Token::Str("type"),
      Token::Some,
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x2",
      },
      Token::Str("transactionHash"),
      Token::Str(T_HASH),
      Token::Str("transactionIndex"),
      Token::Str("0x53"),
      Token::Str("blockHash"),
      Token::Str(B_HASH),
      Token::Str("blockNumber"),
      Token::Str("0x15548cc"),
      Token::Str("from"),
      Token::Str(SEND),
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("contractAddress"),
      Token::None,
      Token::Str("gasUsed"),
      Token::Str("0x5208"),
      Token::Str("cumulativeGasUsed"),
      Token::Str("0x6ccdad"),
      Token::Str("effectiveGasPrice"),
      Token::Str("0x1b9aea77"),
      Token::Str("logs"),
      Token::Some,
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("logsBloom"),
      Token::Some,
      Token::Str(BLOOM),
      Token::Str("status"),
      Token::Some,
      Token::Str("0x1"),
      Token::Str("timestamp"),
      Token::U64(1745829383),
      Token::MapEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
