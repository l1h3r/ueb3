use serde::Deserialize;
use serde::Serialize;

use crate::eth::H256;

// -----------------------------------------------------------------------------
// Chain Forks
// -----------------------------------------------------------------------------

/// A set of information describing the blockchain forks that have occured.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Forks {
  /// The genesis block hash.
  pub genesis: H256,
  /// A list of block heights where forks occurred.
  pub height_forks: Vec<u64>,
  /// A list of timestamps where forks occurred.
  pub time_forks: Vec<u64>,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::erigon::Forks;

  #[test]
  fn test_serde() {
    const GENESIS: &str = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";

    let a: Forks = Forks::default();

    let b: Forks = Forks {
      genesis: GENESIS.parse().unwrap(),
      height_forks: vec![
        1150000, 1920000, 2463000, 2675000, 4370000, 7280000, 9069000, 9200000, 12244000, 12965000,
        13773000, 15050000,
      ],
      time_forks: vec![1681338455, 1710338135, 1746612311],
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Forks",
        len: 3,
      },
      Token::Str("genesis"),
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("heightForks"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("timeForks"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Forks",
        len: 3,
      },
      Token::Str("genesis"),
      Token::Str(GENESIS),
      Token::Str("heightForks"),
      Token::Seq { len: Some(12) },
      Token::U64(1150000),
      Token::U64(1920000),
      Token::U64(2463000),
      Token::U64(2675000),
      Token::U64(4370000),
      Token::U64(7280000),
      Token::U64(9069000),
      Token::U64(9200000),
      Token::U64(12244000),
      Token::U64(12965000),
      Token::U64(13773000),
      Token::U64(15050000),
      Token::SeqEnd,
      Token::Str("timeForks"),
      Token::Seq { len: Some(3) },
      Token::U64(1681338455),
      Token::U64(1710338135),
      Token::U64(1746612311),
      Token::SeqEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
