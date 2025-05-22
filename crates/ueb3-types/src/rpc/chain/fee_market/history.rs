use serde::Deserialize;
use serde::Serialize;

use crate::eth::U256;

// -----------------------------------------------------------------------------
// Ethereum Transaction Fee History
// -----------------------------------------------------------------------------

/// A transaction fee history object.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FeeHistory {
  /// The lowest number block of returned range.
  #[serde(with = "ueb3_serde::quantity")]
  pub oldest_block: u64,
  /// An array of block base fees per gas.
  ///
  /// This includes the next block after the newest of the returned range,
  /// because this value can be derived from the newest block. Zeroes are
  /// returned for pre-[EIP-1559] blocks.
  ///
  /// [EIP-1559]: https://eips.ethereum.org/EIPS/eip-1559
  pub base_fee_per_gas: Vec<U256>,
  /// An array of block base fees per blob gas.
  ///
  /// This includes the next block after the newest of the returned range,
  /// because this value can be derived from the newest block. Zeroes are
  /// returned for pre-[EIP-4844] blocks.
  ///
  /// [EIP-4844]: https://eips.ethereum.org/EIPS/eip-4844
  pub base_fee_per_blob_gas: Vec<U256>,
  /// An array of block gas used ratios.
  ///
  /// These are calculated as the ratio of `gasUsed` and `gasLimit`.
  pub gas_used_ratio: Vec<f64>,
  /// An array of block blob gas used ratios.
  ///
  /// These are calculated as the ratio of `blobGasUsed` and the max blob gas
  /// per block.
  pub blob_gas_used_ratio: Vec<f64>,
  /// A two-dimensional array of effective priority fees per gas at the
  /// requested block percentiles.
  pub reward: Vec<Vec<U256>>,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eth::U256;
  use crate::rpc::FeeHistory;

  #[test]
  fn test_serde() {
    let a: FeeHistory = FeeHistory {
      oldest_block: 0,
      base_fee_per_gas: Vec::new(),
      base_fee_per_blob_gas: Vec::new(),
      gas_used_ratio: Vec::new(),
      blob_gas_used_ratio: Vec::new(),
      reward: Vec::new(),
    };

    let b: FeeHistory = FeeHistory {
      oldest_block: 0x10b52f,
      base_fee_per_gas: vec![
        U256::from(0x3fa63a3f_u64),
        U256::from(0x37f999ee_u64),
        U256::from(0x3e36f20a_u64),
        U256::from(0x4099f79a_u64),
        U256::from(0x430d532d_u64),
        U256::from(0x46fcd4a4_u64),
      ],
      base_fee_per_blob_gas: vec![
        U256::from(0x7b7609c19_u64),
        U256::from(0x6dbe41789_u64),
        U256::from(0x7223341d4_u64),
        U256::from(0x6574a002c_u64),
        U256::from(0x7223341d4_u64),
        U256::from(0x6574a002c_u64),
      ],
      gas_used_ratio: vec![
        0.017712333333333333,
        0.9458865666666667,
        0.6534561,
        0.6517375666666667,
        0.7347769666666667,
      ],
      blob_gas_used_ratio: vec![0.0, 0.6666666666666666, 0.0, 1.0, 0.0],
      reward: vec![
        vec![U256::from(0x3b9aca00), U256::from(0x59682f00)],
        vec![U256::from(0x3a13012), U256::from(0x3a13012)],
        vec![U256::from(0x3a13012), U256::from(0x3a13012)],
        vec![U256::from(0xf4240), U256::from(0xf4240)],
        vec![U256::from(0xf4240), U256::from(0xf4240)],
      ],
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "FeeHistory",
        len: 6,
      },
      Token::Str("oldestBlock"),
      Token::Str("0x0"),
      Token::Str("baseFeePerGas"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("baseFeePerBlobGas"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("gasUsedRatio"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("blobGasUsedRatio"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("reward"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "FeeHistory",
        len: 6,
      },
      Token::Str("oldestBlock"),
      Token::Str("0x10b52f"),
      Token::Str("baseFeePerGas"),
      Token::Seq { len: Some(6) },
      Token::Str("0x3fa63a3f"),
      Token::Str("0x37f999ee"),
      Token::Str("0x3e36f20a"),
      Token::Str("0x4099f79a"),
      Token::Str("0x430d532d"),
      Token::Str("0x46fcd4a4"),
      Token::SeqEnd,
      Token::Str("baseFeePerBlobGas"),
      Token::Seq { len: Some(6) },
      Token::Str("0x7b7609c19"),
      Token::Str("0x6dbe41789"),
      Token::Str("0x7223341d4"),
      Token::Str("0x6574a002c"),
      Token::Str("0x7223341d4"),
      Token::Str("0x6574a002c"),
      Token::SeqEnd,
      Token::Str("gasUsedRatio"),
      Token::Seq { len: Some(5) },
      Token::F64(0.017712333333333333),
      Token::F64(0.9458865666666667),
      Token::F64(0.6534561),
      Token::F64(0.6517375666666667),
      Token::F64(0.7347769666666667),
      Token::SeqEnd,
      Token::Str("blobGasUsedRatio"),
      Token::Seq { len: Some(5) },
      Token::F64(0.0),
      Token::F64(0.6666666666666666),
      Token::F64(0.0),
      Token::F64(1.0),
      Token::F64(0.0),
      Token::SeqEnd,
      Token::Str("reward"),
      Token::Seq { len: Some(5) },
      Token::Seq { len: Some(2) },
      Token::Str("0x3b9aca00"),
      Token::Str("0x59682f00"),
      Token::SeqEnd,
      Token::Seq { len: Some(2) },
      Token::Str("0x3a13012"),
      Token::Str("0x3a13012"),
      Token::SeqEnd,
      Token::Seq { len: Some(2) },
      Token::Str("0x3a13012"),
      Token::Str("0x3a13012"),
      Token::SeqEnd,
      Token::Seq { len: Some(2) },
      Token::Str("0xf4240"),
      Token::Str("0xf4240"),
      Token::SeqEnd,
      Token::Seq { len: Some(2) },
      Token::Str("0xf4240"),
      Token::Str("0xf4240"),
      Token::SeqEnd,
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
