use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eip::eip2718::TransactionType;
use crate::eip::eip2930::AccessList;
use crate::eip::eip7702::AuthorizationList;
use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::H256;
use crate::eth::U256;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Ethereum Transaction Request
// -----------------------------------------------------------------------------

/// A transaction request object.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionRequest {
  /// The transaction type identifier.
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub transaction_type: Option<TransactionType>,
  /// A unique identifier for which chain the transaction occured on.
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub chain_id: Option<u64>,
  /// The address of the transaction sender.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<Address>,
  /// The address of the transaction receiver.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Address>,
  /// The number of prior transactions made by the sender.
  #[serde(
    default,
    skip_serializing_if = "Option::is_none",
    with = "ueb3_serde::quantity::opt"
  )]
  pub nonce: Option<u64>,
  /// The total amount of gas provided by the sender.
  #[serde(rename = "gas", skip_serializing_if = "Option::is_none")]
  pub gas_limit: Option<U256>,
  /// The gas price willing to be paid by the sender.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub gas_price: Option<Wei>,
  /// The maximum fee per gas the sender is willing to pay to miners.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_priority_fee_per_gas: Option<Wei>,
  /// The maximum total fee per gas the sender is willing to pay (includes the
  /// network / base fee and miner / priority fee).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_fee_per_gas: Option<Wei>,
  /// The maximum total fee per gas the sender is willing to pay for blob gas.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_fee_per_blob_gas: Option<Wei>,
  /// The amount of ETH transferred in the transaction.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<Wei>,
  /// Additional data included with the transaction.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub input: Option<Bytes>,
  /// A list of addresses/storage keys that the transaction plans to access.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access_list: Option<AccessList>,
  /// A list of versioned blob hashes associated with the transaction's data blobs.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blob_versioned_hashes: Option<Vec<H256>>,
  /// Raw blob data.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blobs: Option<Vec<Bytes>>,
  /// A list of authorizations for the transaction.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub authorization_list: Option<AuthorizationList>,
}

impl Debug for TransactionRequest {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionRequest")
      .field("chain_id", &self.chain_id.format())
      .field("transaction_type", &self.transaction_type.format())
      .field("from", &self.from.format())
      .field("to", &self.to.format())
      .field("nonce", &self.nonce.format())
      .field("gas_limit", &self.gas_limit.format())
      .field("gas_price", &self.gas_price.format())
      .field("max_priority_fee_per_gas", &self.max_priority_fee_per_gas.format())
      .field("max_fee_per_gas", &self.max_fee_per_gas.format())
      .field("max_fee_per_blob_gas", &self.max_fee_per_blob_gas.format())
      .field("value", &self.value.format())
      .field("input", &self.input.format())
      .field("access_list", &self.access_list.format())
      .field("blob_versioned_hashes", &self.blob_versioned_hashes.format())
      .field("blobs", &self.blobs.format())
      .field("authorization_list", &self.authorization_list.format())
      .finish()
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

  use crate::rpc::TransactionRequest;

  #[test]
  fn test_serde() {
    const SEND: &str = "0xb60e8dd61c5d32be8058bb8eb970870f07233155";
    const RECV: &str = "0xd46e8dd67c5d32be8058bb8eb970870f07244567";
    const DATA: &str = "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675";

    let a: TransactionRequest = TransactionRequest {
      to: Some(RECV.parse().unwrap()),
      value: Some("0x1".parse().unwrap()),
      ..TransactionRequest::default()
    };

    let b: TransactionRequest = TransactionRequest {
      from: Some(SEND.parse().unwrap()),
      input: Some("0x608060806080608155".parse().unwrap()),
      ..TransactionRequest::default()
    };

    let c: TransactionRequest = TransactionRequest {
      from: Some(SEND.parse().unwrap()),
      to: Some(RECV.parse().unwrap()),
      value: Some("0x1".parse().unwrap()),
      ..TransactionRequest::default()
    };

    let d: TransactionRequest = TransactionRequest {
      from: Some(SEND.parse().unwrap()),
      to: Some(RECV.parse().unwrap()),
      gas_limit: Some("0x76c0".parse().unwrap()),
      gas_price: Some("0x9184e72a000".parse().unwrap()),
      value: Some("0x9184e72a".parse().unwrap()),
      input: Some(DATA.parse().unwrap()),
      ..TransactionRequest::default()
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionRequest",
        len: 2,
      },
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("value"),
      Token::Some,
      Token::Str("0x1"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionRequest",
        len: 2,
      },
      Token::Str("from"),
      Token::Some,
      Token::Str(SEND),
      Token::Str("input"),
      Token::Some,
      Token::Str("0x608060806080608155"),
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "TransactionRequest",
        len: 3,
      },
      Token::Str("from"),
      Token::Some,
      Token::Str(SEND),
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("value"),
      Token::Some,
      Token::Str("0x1"),
      Token::StructEnd,
    ];

    let tokens_d: &[Token] = &[
      Token::Struct {
        name: "TransactionRequest",
        len: 6,
      },
      Token::Str("from"),
      Token::Some,
      Token::Str(SEND),
      Token::Str("to"),
      Token::Some,
      Token::Str(RECV),
      Token::Str("gas"),
      Token::Some,
      Token::Str("0x76c0"),
      Token::Str("gasPrice"),
      Token::Some,
      Token::Str("0x9184e72a000"),
      Token::Str("value"),
      Token::Some,
      Token::Str("0x9184e72a"),
      Token::Str("input"),
      Token::Some,
      Token::Str(DATA),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
    assert_tokens(&c.readable(), tokens_c);
    assert_tokens(&d.readable(), tokens_d);
  }
}
