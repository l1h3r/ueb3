use serde::Deserialize;
use serde::Serialize;

use crate::eth::H256;

// -----------------------------------------------------------------------------
// Block Opcode Gas
// -----------------------------------------------------------------------------

/// Block opcode gas usage information.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BlockOpcodeGas {
  /// The hash of the block.
  pub block_hash: H256,
  /// The number of the block.
  pub block_number: u64,
  /// A list of transaction opcode gas usages.
  pub transactions: Vec<TransactionOpcodeGas>,
}

// -----------------------------------------------------------------------------
// Transaction Opcode Gas
// -----------------------------------------------------------------------------

/// Transaction opcode gas usage information.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionOpcodeGas {
  /// The hash of the transaction.
  pub transaction_hash: H256,
  /// A list of opcode gas usages.
  pub opcode_gas: Vec<OpcodeGas>,
}

// -----------------------------------------------------------------------------
// Opcode Gas
// -----------------------------------------------------------------------------

/// Opcode gas usage information.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OpcodeGas {
  /// The opcode name.
  pub opcode: String,
  /// The total number of opcode executions.
  pub count: u64,
  /// The total amount of gas used by the opcode.
  pub gas_used: u64,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::trace::BlockOpcodeGas;
  use crate::rpc::trace::OpcodeGas;
  use crate::rpc::trace::TransactionOpcodeGas;

  #[test]
  fn test_serde() {
    const B_HASH: &str = "0x7eb25504e4c202cf3d62fd585d3e238f592c780cca82dacb2ed3cb5b38883add";
    const T_HASH: &str = "0x17104ac9d3312d8c136b7f44d4b8b47852618065ebfa534bd2d3b5ef218ca1f3";

    let a: BlockOpcodeGas = BlockOpcodeGas::default();

    let b: BlockOpcodeGas = BlockOpcodeGas {
      block_hash: B_HASH.parse().unwrap(),
      block_number: 3068185,
      transactions: vec![TransactionOpcodeGas {
        transaction_hash: T_HASH.parse().unwrap(),
        opcode_gas: vec![OpcodeGas {
          opcode: "MSTORE".to_owned(),
          count: 5,
          gas_used: 39,
        }],
      }],
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "BlockOpcodeGas",
        len: 3,
      },
      Token::Str("blockHash"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("blockNumber"),
      Token::U64(0),
      Token::Str("transactions"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "BlockOpcodeGas",
        len: 3,
      },
      Token::Str("blockHash"),
      Token::Str(B_HASH),
      Token::Str("blockNumber"),
      Token::U64(3068185),
      Token::Str("transactions"),
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "TransactionOpcodeGas",
        len: 2,
      },
      Token::Str("transactionHash"),
      Token::Str(T_HASH),
      Token::Str("opcodeGas"),
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "OpcodeGas",
        len: 3,
      },
      Token::Str("opcode"),
      Token::Str("MSTORE"),
      Token::Str("count"),
      Token::U64(5),
      Token::Str("gasUsed"),
      Token::U64(39),
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
