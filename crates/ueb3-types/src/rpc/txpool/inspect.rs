use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::ops::Deref;

use crate::eth::Address;
use crate::rpc::txpool::ContentFrom;

// -----------------------------------------------------------------------------
// Transaction Pool Inspect
// -----------------------------------------------------------------------------

/// A formatted set of all transactions in the pool.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Inspect {
  /// The pending transactions.
  pub pending: BTreeMap<Address, BTreeMap<String, String>>,
  /// The queued transactions.
  pub queued: BTreeMap<Address, BTreeMap<String, String>>,
}

impl Inspect {
  /// Returns an iterator over the pending transactions.
  #[inline]
  pub fn iter_pending(&self) -> impl Iterator<Item = &str> {
    self
      .pending
      .values()
      .flat_map(BTreeMap::values)
      .map(Deref::deref)
  }

  /// Returns an iterator over the queued transactions.
  #[inline]
  pub fn iter_queued(&self) -> impl Iterator<Item = &str> {
    self
      .queued
      .values()
      .flat_map(BTreeMap::values)
      .map(Deref::deref)
  }

  /// Returns an iterator over the pending transactions, filtered by `address`.
  #[inline]
  pub fn iter_pending_from(&self, address: &Address) -> impl Iterator<Item = &str> {
    self
      .pending
      .get(address)
      .into_iter()
      .flat_map(BTreeMap::values)
      .map(Deref::deref)
  }

  /// Returns an iterator over the queued transactions, filtered by `address`.
  #[inline]
  pub fn iter_queued_from(&self, address: &Address) -> impl Iterator<Item = &str> {
    self
      .queued
      .get(address)
      .into_iter()
      .flat_map(BTreeMap::values)
      .map(Deref::deref)
  }

  /// Converts `self` to an iterator over the pending transactions.
  #[inline]
  pub fn into_iter_pending(self) -> impl Iterator<Item = String> {
    self.pending.into_values().flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the queued transactions.
  #[inline]
  pub fn into_iter_queued(self) -> impl Iterator<Item = String> {
    self.queued.into_values().flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the pending transactions, filtered by `address`.
  #[inline]
  pub fn into_iter_pending_from(mut self, address: &Address) -> impl Iterator<Item = String> {
    self
      .pending
      .remove(address)
      .into_iter()
      .flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the queued transactions, filtered by `address`.
  #[inline]
  pub fn into_iter_queued_from(mut self, address: &Address) -> impl Iterator<Item = String> {
    self
      .queued
      .remove(address)
      .into_iter()
      .flat_map(BTreeMap::into_values)
  }

  /// Removes and returns a subset of transactions from the given `address`.
  #[inline]
  pub fn take(&mut self, address: &Address) -> ContentFrom<String> {
    ContentFrom {
      pending: self.pending.remove(address).unwrap_or_default(),
      queued: self.queued.remove(address).unwrap_or_default(),
      base_fee: BTreeMap::new(),
    }
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
  use std::collections::BTreeMap;

  use crate::rpc::txpool::Inspect;

  #[test]
  fn test_serde() {
    const ADDRESS_1: &str = "0xbc0ca4f217e052753614d6b019948824d0d8688b";
    const ADDRESS_2: &str = "0xea674fdde714fd979de3edf0f56aa9716b898ec8";
    const ADDRESS_3: &str = "0x0f6000de1578619320aba5e392706b131fb1de6f";

    const KEY_1: &str = "0";
    const KEY_2: &str = "70148";
    const KEY_3: &str = "6";

    const VAL_1: &str = "0x2910543af39aba0cd09dbb2d50200b3e800a63d2: 1000000000000000000 wei + 50000 gas × 1171602790622 wei";
    const VAL_2: &str = "0xe39c55ead9f997f7fa20ebe40fb4649943d7db66: 1000767667434026200 wei + 90000 gas × 20000000000 wei";
    const VAL_3: &str = "0x8383534d0bcd0186d326c993031311c0ac0d9b2d: 9000000000000000000 wei + 21000 gas × 20000000000 wei";

    let a: Inspect = Inspect::default();

    let b: Inspect = Inspect {
      pending: BTreeMap::from([
        (
          ADDRESS_1.parse().unwrap(),
          [(KEY_1.to_owned(), VAL_1.to_owned())].into(),
        ),
        (
          ADDRESS_2.parse().unwrap(),
          [(KEY_2.to_owned(), VAL_2.to_owned())].into(),
        ),
      ]),
      queued: BTreeMap::from([(
        ADDRESS_3.parse().unwrap(),
        [(KEY_3.to_owned(), VAL_3.to_owned())].into(),
      )]),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Inspect",
        len: 2,
      },
      Token::Str("pending"),
      Token::Map { len: Some(0) },
      Token::MapEnd,
      Token::Str("queued"),
      Token::Map { len: Some(0) },
      Token::MapEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "Inspect",
        len: 2,
      },
      Token::Str("pending"),
      Token::Map { len: Some(2) },
      Token::Str(ADDRESS_1),
      Token::Map { len: Some(1) },
      Token::Str(KEY_1),
      Token::Str(VAL_1),
      Token::MapEnd,
      Token::Str(ADDRESS_2),
      Token::Map { len: Some(1) },
      Token::Str(KEY_2),
      Token::Str(VAL_2),
      Token::MapEnd,
      Token::MapEnd,
      Token::Str("queued"),
      Token::Map { len: Some(1) },
      Token::Str(ADDRESS_3),
      Token::Map { len: Some(1) },
      Token::Str(KEY_3),
      Token::Str(VAL_3),
      Token::MapEnd,
      Token::MapEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
