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

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eth::U256;
  use crate::rpc::FeeHistory;

  #[test]
  fn test_serde() {
    let value: FeeHistory = FeeHistory {
      oldest_block: 1234567890,
      base_fee_per_gas: vec![U256::from(1500000000), U256::from(1500000000)],
      base_fee_per_blob_gas: vec![U256::from(1), U256::from(1)],
      gas_used_ratio: vec![0.5555555555555555],
      blob_gas_used_ratio: vec![0.6666666666666666],
      reward: vec![vec![U256::from(2000000000)]],
    };

    assert_tokens(
      &value.readable(),
      &[
        Token::Struct {
          name: "FeeHistory",
          len: 6,
        },
        Token::Str("oldestBlock"),
        Token::Str("0x499602d2"),
        Token::Str("baseFeePerGas"),
        Token::Seq { len: Some(2) },
        Token::Str("0x59682f00"),
        Token::Str("0x59682f00"),
        Token::SeqEnd,
        Token::Str("baseFeePerBlobGas"),
        Token::Seq { len: Some(2) },
        Token::Str("0x1"),
        Token::Str("0x1"),
        Token::SeqEnd,
        Token::Str("gasUsedRatio"),
        Token::Seq { len: Some(1) },
        Token::F64(0.5555555555555555),
        Token::SeqEnd,
        Token::Str("blobGasUsedRatio"),
        Token::Seq { len: Some(1) },
        Token::F64(0.6666666666666666),
        Token::SeqEnd,
        Token::Str("reward"),
        Token::Seq { len: Some(1) },
        Token::Seq { len: Some(1) },
        Token::Str("0x77359400"),
        Token::SeqEnd,
        Token::SeqEnd,
        Token::StructEnd,
      ],
    );
  }
}
