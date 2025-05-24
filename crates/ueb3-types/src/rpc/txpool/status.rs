use serde::Deserialize;
use serde::Serialize;

#[expect(clippy::trivially_copy_pass_by_ref, reason = "serde signature")]
#[inline]
const fn is_zero(value: &u64) -> bool {
  *value == 0
}

// -----------------------------------------------------------------------------
// Transaction Pool Status
// -----------------------------------------------------------------------------

/// The number of pending and queued transaction in the pool.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Status {
  /// The total number of pending transactions.
  #[serde(with = "ueb3_serde::quantity")]
  pub pending: u64,
  /// The total number of queued transactions.
  #[serde(with = "ueb3_serde::quantity")]
  pub queued: u64,
  /// The total number of transactions priced below the block base fee.
  #[serde(
    default,
    with = "ueb3_serde::quantity",
    skip_serializing_if = "is_zero"
  )]
  pub base_fee: u64,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::rpc::txpool::Status;

  #[test]
  fn test_serde() {
    let a: Status = Status::default();

    let b: Status = Status {
      pending: 0xab,
      queued: 0xcd,
      base_fee: 0,
    };

    let c: Status = Status {
      pending: 0xab,
      queued: 0xcd,
      base_fee: 0xef,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Status",
        len: 2,
      },
      Token::Str("pending"),
      Token::Str("0x0"),
      Token::Str("queued"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Status",
        len: 2,
      },
      Token::Str("pending"),
      Token::Str("0xab"),
      Token::Str("queued"),
      Token::Str("0xcd"),
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "Status",
        len: 3,
      },
      Token::Str("pending"),
      Token::Str("0xab"),
      Token::Str("queued"),
      Token::Str("0xcd"),
      Token::Str("baseFee"),
      Token::Str("0xef"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
    assert_tokens(&c.readable(), tokens_c);
  }
}
