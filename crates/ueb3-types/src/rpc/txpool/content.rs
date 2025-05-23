use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

use crate::eth::Address;
use crate::rpc::Transaction;

// -----------------------------------------------------------------------------
// Transaction Pool Content
// -----------------------------------------------------------------------------

/// A set of all transactions in the pool.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Content {
  /// The pending transactions.
  pub pending: BTreeMap<Address, BTreeMap<String, Transaction>>,
  /// The queued transactions.
  pub queued: BTreeMap<Address, BTreeMap<String, Transaction>>,
  /// The transactions priced below the block base fee.
  #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
  pub base_fee: BTreeMap<Address, BTreeMap<String, Transaction>>,
}

impl Content {
  /// Returns an iterator over the pending transactions.
  #[inline]
  pub fn iter_pending(&self) -> impl Iterator<Item = &Transaction> {
    self.pending.values().flat_map(BTreeMap::values)
  }

  /// Returns an iterator over the queued transactions.
  #[inline]
  pub fn iter_queued(&self) -> impl Iterator<Item = &Transaction> {
    self.queued.values().flat_map(BTreeMap::values)
  }

  /// Returns an iterator over the transactions priced below the block base fee.
  #[inline]
  pub fn iter_base_fee(&self) -> impl Iterator<Item = &Transaction> {
    self.base_fee.values().flat_map(BTreeMap::values)
  }

  /// Returns an iterator over the pending transactions, filtered by `address`.
  #[inline]
  pub fn iter_pending_from(&self, address: &Address) -> impl Iterator<Item = &Transaction> {
    self
      .pending
      .get(address)
      .into_iter()
      .flat_map(BTreeMap::values)
  }

  /// Returns an iterator over the queued transactions, filtered by `address`.
  #[inline]
  pub fn iter_queued_from(&self, address: &Address) -> impl Iterator<Item = &Transaction> {
    self
      .queued
      .get(address)
      .into_iter()
      .flat_map(BTreeMap::values)
  }

  /// Returns an iterator over the transactions priced below the block base fee,
  /// filtered by `address`.
  #[inline]
  pub fn iter_base_fee_from(&self, address: &Address) -> impl Iterator<Item = &Transaction> {
    self
      .base_fee
      .get(address)
      .into_iter()
      .flat_map(BTreeMap::values)
  }

  /// Converts `self` to an iterator over the pending transactions.
  #[inline]
  pub fn into_iter_pending(self) -> impl Iterator<Item = Transaction> {
    self.pending.into_values().flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the queued transactions.
  #[inline]
  pub fn into_iter_queued(self) -> impl Iterator<Item = Transaction> {
    self.queued.into_values().flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the transactions priced below the
  /// block base fee.
  #[inline]
  pub fn into_iter_base_fee(self) -> impl Iterator<Item = Transaction> {
    self.base_fee.into_values().flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the pending transactions, filtered by `address`.
  #[inline]
  pub fn into_iter_pending_from(mut self, address: &Address) -> impl Iterator<Item = Transaction> {
    self
      .pending
      .remove(address)
      .into_iter()
      .flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the queued transactions, filtered by `address`.
  #[inline]
  pub fn into_iter_queued_from(mut self, address: &Address) -> impl Iterator<Item = Transaction> {
    self
      .queued
      .remove(address)
      .into_iter()
      .flat_map(BTreeMap::into_values)
  }

  /// Converts `self` to an iterator over the transactions priced below the
  /// block base fee, filtered by `address`.
  #[inline]
  pub fn into_iter_base_fee_from(mut self, address: &Address) -> impl Iterator<Item = Transaction> {
    self
      .base_fee
      .remove(address)
      .into_iter()
      .flat_map(BTreeMap::into_values)
  }

  /// Removes and returns a subset of transactions from the given `address`.
  #[inline]
  pub fn take(&mut self, address: &Address) -> ContentFrom {
    ContentFrom {
      pending: self.pending.remove(address).unwrap_or_default(),
      queued: self.queued.remove(address).unwrap_or_default(),
      base_fee: self.base_fee.remove(address).unwrap_or_default(),
    }
  }
}

// -----------------------------------------------------------------------------
// Transaction Pool Content (Address-Specific)
// -----------------------------------------------------------------------------

/// A set of address-specific transactions in the pool.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentFrom<T = Transaction> {
  /// The pending transactions.
  pub pending: BTreeMap<String, T>,
  /// The queued transactions.
  pub queued: BTreeMap<String, T>,
  /// The transactions priced below the block base fee.
  #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
  pub base_fee: BTreeMap<String, T>,
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

  use crate::eth::U256;
  use crate::rpc::Transaction;
  use crate::rpc::TransactionHeader;
  use crate::rpc::TransactionLegacy;
  use crate::rpc::TransactionPayload;
  use crate::rpc::TransactionSigner;
  use crate::rpc::txpool::Content;

  #[test]
  fn test_serde() {
    const SEND_1: &str = "0x0216d5032f356960cd3749c31ab34eeff21b3395";
    const SEND_2: &str = "0x24d407e5a0b506e1cb2fae163100b5de01f5193c";
    const SEND_3: &str = "0x976a3fc5d6f7d259ebfb4cc2ae75115475e9867c";
    const SEND_4: &str = "0x9b11bf0459b0c4b2f87f8cebca4cfc26f294b63a";

    const RECV_1: &str = "0x7f69a91a3cf4be60020fb58b893b7cbb65376db8";
    const RECV_2: &str = "0x7320785200f74861b69c49e4ab32399a71b34f1a";
    const RECV_3: &str = "0x346fb27de7e7370008f5da379f74dd49f5f2f80f";
    const RECV_4: &str = "0x24a461f25ee6a318bdef7f33de634a67bb67ac9d";

    const HASH_1: &str = "0xaf953a2d01f55cfe080c0c94150a60105e8ac3d51153058a1f03dd239dd08586";
    const HASH_2: &str = "0xb5b8b853af32226755a65ba0602f7ed0e8be2211516153b75e9ed640a7d359fe";
    const HASH_3: &str = "0x57b30c59fc39a50e1cba90e3099286dfa5aaf60294a629240b5bbec6e2e66576";
    const HASH_4: &str = "0x3a3c0698552eec2455ed3190eac3996feccc806970a4a056106deaf6ceb1e5e3";

    const KEY_1: &str = "806";
    const KEY_2: &str = "34";
    const KEY_3: &str = "3";
    const KEY_4: &str = "2";

    const INPUT: &str = "0xb61d27f600000000000000000000000024d407e5a0b506e1cb2fae163100b5de01f5193c00000000000000000000000000000000000000000000000053444835ec580000000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

    const SIGNER: TransactionSigner = TransactionSigner {
      v: 0,
      r: U256::zero(),
      s: U256::zero(),
      y: None,
    };

    fn header(hash: &str) -> TransactionHeader {
      TransactionHeader {
        hash: hash.parse().unwrap(),
        index: None,
        block_hash: Some(Default::default()),
        block_number: None,
      }
    }

    let value_1: Transaction = Transaction {
      header: header(HASH_1),
      object: TransactionPayload::Legacy(TransactionLegacy {
        chain_id: None,
        from: SEND_1.parse().unwrap(),
        to: Some(RECV_1.parse().unwrap()),
        nonce: 0x326,
        gas_limit: "0x5208".parse().unwrap(),
        gas_price: "0xba43b7400".parse().unwrap(),
        value: "0x19a99f0cf456000".parse().unwrap(),
        input: Default::default(),
      }),
      signer: SIGNER,
    };

    let value_2: Transaction = Transaction {
      header: header(HASH_2),
      object: TransactionPayload::Legacy(TransactionLegacy {
        chain_id: None,
        from: SEND_2.parse().unwrap(),
        to: Some(RECV_2.parse().unwrap()),
        nonce: 0x22,
        gas_limit: "0x44c72".parse().unwrap(),
        gas_price: "0x4a817c800".parse().unwrap(),
        value: Default::default(),
        input: INPUT.parse().unwrap(),
      }),
      signer: SIGNER,
    };

    let value_3: Transaction = Transaction {
      header: header(HASH_3),
      object: TransactionPayload::Legacy(TransactionLegacy {
        chain_id: None,
        from: SEND_3.parse().unwrap(),
        to: Some(RECV_3.parse().unwrap()),
        nonce: 0x3,
        gas_limit: "0x15f90".parse().unwrap(),
        gas_price: "0x4a817c800".parse().unwrap(),
        value: "0x1f161421c8e0000".parse().unwrap(),
        input: Default::default(),
      }),
      signer: SIGNER,
    };

    let value_4: Transaction = Transaction {
      header: header(HASH_4),
      object: TransactionPayload::Legacy(TransactionLegacy {
        chain_id: None,
        from: SEND_4.parse().unwrap(),
        to: Some(RECV_4.parse().unwrap()),
        nonce: 0x2,
        gas_limit: "0x15f90".parse().unwrap(),
        gas_price: "0xba43b7400".parse().unwrap(),
        value: "0xebec21ee1da40000".parse().unwrap(),
        input: Default::default(),
      }),
      signer: SIGNER,
    };

    let a: Content = Content::default();

    let b: Content = Content {
      pending: BTreeMap::from([
        (
          SEND_1.parse().unwrap(),
          [(KEY_1.to_owned(), value_1)].into(),
        ),
        (
          SEND_2.parse().unwrap(),
          [(KEY_2.to_owned(), value_2)].into(),
        ),
      ]),
      queued: BTreeMap::from([(
        SEND_3.parse().unwrap(),
        [(KEY_3.to_owned(), value_3)].into(),
      )]),
      base_fee: BTreeMap::from([(
        SEND_4.parse().unwrap(),
        [(KEY_4.to_owned(), value_4)].into(),
      )]),
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Content",
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
        name: "Content",
        len: 3,
      },
      Token::Str("pending"),
      Token::Map { len: Some(2) },
      Token::Str(SEND_1),
      Token::Map { len: Some(1) },
      Token::Str(KEY_1),
      Token::Struct {
        name: "Transaction",
        len: 15,
      },
      Token::Str("hash"),
      Token::Str(HASH_1),
      Token::Str("transactionIndex"),
      Token::None,
      Token::Str("blockHash"),
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("blockNumber"),
      Token::None,
      Token::BorrowedStr("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x0",
      },
      Token::BorrowedStr("from"),
      Token::Str(SEND_1),
      Token::BorrowedStr("to"),
      Token::Some,
      Token::Str(RECV_1),
      Token::BorrowedStr("nonce"),
      Token::Str("0x326"),
      Token::BorrowedStr("gas"),
      Token::Str("0x5208"),
      Token::BorrowedStr("gasPrice"),
      Token::Str("0xba43b7400"),
      Token::BorrowedStr("value"),
      Token::Str("0x19a99f0cf456000"),
      Token::BorrowedStr("input"),
      Token::Str("0x"),
      Token::Str("v"),
      Token::Str("0x0"),
      Token::Str("r"),
      Token::Str("0x0"),
      Token::Str("s"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::MapEnd,
      Token::Str(SEND_2),
      Token::Map { len: Some(1) },
      Token::Str(KEY_2),
      Token::Struct {
        name: "Transaction",
        len: 15,
      },
      Token::Str("hash"),
      Token::Str(HASH_2),
      Token::Str("transactionIndex"),
      Token::None,
      Token::Str("blockHash"),
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("blockNumber"),
      Token::None,
      Token::BorrowedStr("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x0",
      },
      Token::BorrowedStr("from"),
      Token::Str(SEND_2),
      Token::BorrowedStr("to"),
      Token::Some,
      Token::Str(RECV_2),
      Token::BorrowedStr("nonce"),
      Token::Str("0x22"),
      Token::BorrowedStr("gas"),
      Token::Str("0x44c72"),
      Token::BorrowedStr("gasPrice"),
      Token::Str("0x4a817c800"),
      Token::BorrowedStr("value"),
      Token::Str("0x0"),
      Token::BorrowedStr("input"),
      Token::Str(INPUT),
      Token::Str("v"),
      Token::Str("0x0"),
      Token::Str("r"),
      Token::Str("0x0"),
      Token::Str("s"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::MapEnd,
      Token::MapEnd,
      Token::Str("queued"),
      Token::Map { len: Some(1) },
      Token::Str(SEND_3),
      Token::Map { len: Some(1) },
      Token::Str(KEY_3),
      Token::Struct {
        name: "Transaction",
        len: 15,
      },
      Token::Str("hash"),
      Token::Str(HASH_3),
      Token::Str("transactionIndex"),
      Token::None,
      Token::Str("blockHash"),
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("blockNumber"),
      Token::None,
      Token::BorrowedStr("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x0",
      },
      Token::BorrowedStr("from"),
      Token::Str(SEND_3),
      Token::BorrowedStr("to"),
      Token::Some,
      Token::Str(RECV_3),
      Token::BorrowedStr("nonce"),
      Token::Str("0x3"),
      Token::BorrowedStr("gas"),
      Token::Str("0x15f90"),
      Token::BorrowedStr("gasPrice"),
      Token::Str("0x4a817c800"),
      Token::BorrowedStr("value"),
      Token::Str("0x1f161421c8e0000"),
      Token::BorrowedStr("input"),
      Token::Str("0x"),
      Token::Str("v"),
      Token::Str("0x0"),
      Token::Str("r"),
      Token::Str("0x0"),
      Token::Str("s"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::MapEnd,
      Token::MapEnd,
      Token::Str("baseFee"),
      Token::Map { len: Some(1) },
      Token::Str(SEND_4),
      Token::Map { len: Some(1) },
      Token::Str(KEY_4),
      Token::Struct {
        name: "Transaction",
        len: 15,
      },
      Token::Str("hash"),
      Token::Str(HASH_4),
      Token::Str("transactionIndex"),
      Token::None,
      Token::Str("blockHash"),
      Token::Some,
      Token::Str("0x0000000000000000000000000000000000000000000000000000000000000000"),
      Token::Str("blockNumber"),
      Token::None,
      Token::BorrowedStr("type"),
      Token::UnitVariant {
        name: "TransactionType",
        variant: "0x0",
      },
      Token::BorrowedStr("from"),
      Token::Str(SEND_4),
      Token::BorrowedStr("to"),
      Token::Some,
      Token::Str(RECV_4),
      Token::BorrowedStr("nonce"),
      Token::Str("0x2"),
      Token::BorrowedStr("gas"),
      Token::Str("0x15f90"),
      Token::BorrowedStr("gasPrice"),
      Token::Str("0xba43b7400"),
      Token::BorrowedStr("value"),
      Token::Str("0xebec21ee1da40000"),
      Token::BorrowedStr("input"),
      Token::Str("0x"),
      Token::Str("v"),
      Token::Str("0x0"),
      Token::Str("r"),
      Token::Str("0x0"),
      Token::Str("s"),
      Token::Str("0x0"),
      Token::StructEnd,
      Token::MapEnd,
      Token::MapEnd,
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
