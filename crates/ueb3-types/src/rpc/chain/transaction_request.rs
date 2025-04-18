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

/// a transaction request object.
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
      .field("authorization_list", &self.authorization_list.format())
      .finish()
  }
}
