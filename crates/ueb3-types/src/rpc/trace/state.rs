use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::eth::Bytes;
use crate::eth::H256;
use crate::eth::U64;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Change
// -----------------------------------------------------------------------------

/// The before/after values of a state change.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Change<T> {
  /// The value of `T` *before* the state change.
  pub from: T,
  /// The value of `T` *after* the state change.
  pub to: T,
}

// -----------------------------------------------------------------------------
// State Diff
// -----------------------------------------------------------------------------

/// The value transition of a state change.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum StateDiff<T> {
  /// The values are equal.
  #[default]
  #[serde(rename = "=")]
  Equal,
  /// The value was inserted.
  #[serde(rename = "+")]
  Insert(T),
  /// The value was removed.
  #[serde(rename = "-")]
  Remove(T),
  /// The value changed.
  #[serde(rename = "*")]
  Change(Change<T>),
}

impl<T> Debug for StateDiff<T>
where
  T: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Equal => f.write_str("Equal"),
      Self::Insert(inner) => write!(f, "Insert({inner:?})"),
      Self::Remove(inner) => write!(f, "Remove({inner:?})"),
      Self::Change(Change { from, to }) => write!(f, "Change({from:?} -> {to:?})"),
    }
  }
}

// -----------------------------------------------------------------------------
// State Diff Account
// -----------------------------------------------------------------------------

/// The account transition of a state change.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct StateDiffAccount {
  /// The change made to account `balance`.
  pub balance: StateDiff<U256>,
  /// The change made to account `code`.
  pub code: StateDiff<Bytes>,
  /// The change made to account `nonce`.
  pub nonce: StateDiff<U64>,
  /// The changes made to storage values.
  pub storage: BTreeMap<H256, StateDiff<H256>>,
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;
  use std::collections::BTreeMap;

  use crate::eth::H256;
  use crate::rpc::trace::Change;
  use crate::rpc::trace::StateDiff;
  use crate::rpc::trace::StateDiffAccount;

  #[test]
  fn test_serde() {
    const NULL: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

    const KEY_1: &str = "0x98af170d2fe1f73c638d070776fb485cc93ce983958d37cb72397982fdb58b40";
    const KEY_2: &str = "0x98af170d2fe1f73c638d070776fb485cc93ce983958d37cb72397982fdb58b41";

    const VAL_1: &str = "0x00000000000000000000000000472c1e4275230354dbe5007a5976053f12610a";
    const VAL_2: &str = "0x00000000000000000000000000000000000000000000000000000000588a27e6";

    let a: StateDiffAccount = StateDiffAccount::default();

    let b: StateDiffAccount = StateDiffAccount {
      balance: StateDiff::Change(Change {
        from: 0x2ad99d025e910036f6_u128.into(),
        to: 0x2ad9a1b06db77876f6_u128.into(),
      }),
      code: StateDiff::Equal,
      nonce: StateDiff::Equal,
      storage: {
        let mut data: BTreeMap<H256, StateDiff<H256>> = BTreeMap::new();

        let item_1: StateDiff<H256> = StateDiff::Change(Change {
          from: NULL.parse().unwrap(),
          to: VAL_1.parse().unwrap(),
        });

        let item_2: StateDiff<H256> = StateDiff::Change(Change {
          from: NULL.parse().unwrap(),
          to: VAL_2.parse().unwrap(),
        });

        let _skip: Option<_> = data.insert(KEY_1.parse().unwrap(), item_1);
        let _skip: Option<_> = data.insert(KEY_2.parse().unwrap(), item_2);

        data
      },
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "StateDiffAccount",
        len: 4,
      },
      Token::Str("balance"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("code"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("nonce"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("storage"),
      Token::Map { len: Some(0) },
      Token::MapEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "StateDiffAccount",
        len: 4,
      },
      Token::Str("balance"),
      Token::NewtypeVariant {
        name: "StateDiff",
        variant: "*",
      },
      Token::Struct {
        name: "Change",
        len: 2,
      },
      Token::Str("from"),
      Token::Str("0x2ad99d025e910036f6"),
      Token::Str("to"),
      Token::Str("0x2ad9a1b06db77876f6"),
      Token::StructEnd,
      Token::Str("code"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("nonce"),
      Token::UnitVariant {
        name: "StateDiff",
        variant: "=",
      },
      Token::Str("storage"),
      Token::Map { len: Some(2) },
      Token::Str(KEY_1),
      Token::NewtypeVariant {
        name: "StateDiff",
        variant: "*",
      },
      Token::Struct {
        name: "Change",
        len: 2,
      },
      Token::Str("from"),
      Token::Str(NULL),
      Token::Str("to"),
      Token::Str(VAL_1),
      Token::StructEnd,
      Token::Str(KEY_2),
      Token::NewtypeVariant {
        name: "StateDiff",
        variant: "*",
      },
      Token::Struct {
        name: "Change",
        len: 2,
      },
      Token::Str("from"),
      Token::Str(NULL),
      Token::Str("to"),
      Token::Str(VAL_2),
      Token::StructEnd,
      Token::MapEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
