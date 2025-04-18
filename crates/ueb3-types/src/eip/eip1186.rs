//! Extra types defined in [EIP-1186].
//!
//! [EIP-1186]: https://eips.ethereum.org/EIPS/eip-1186

use serde::Deserialize;
use serde::Serialize;

use crate::eth::Address;
use crate::eth::Bytes;
use crate::eth::H256;
use crate::eth::U256;
use crate::eth::Wei;

// -----------------------------------------------------------------------------
// Account Proof
// -----------------------------------------------------------------------------

/// An account proof object.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AccountProof {
  /// The account address.
  pub address: Address,
  /// The balance of the account.
  pub balance: Wei,
  /// The nonce of the account.
  #[serde(with = "ueb3_serde::quantity")]
  pub nonce: u64,
  /// The hash of the code of the account.
  pub code_hash: H256,
  /// The SHA3 of the storage root.
  pub storage_hash: H256,
  /// An array of rlp-serialized merkle tree nodes.
  pub account_proof: Vec<Bytes>,
  /// An array of storage entries.
  pub storage_proof: Vec<StorageProof>,
}

// -----------------------------------------------------------------------------
// Storage Proof
// -----------------------------------------------------------------------------

/// A storage proof object.
#[derive(Clone, Default, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorageProof {
  /// The storage key.
  pub key: H256,
  /// The storage value.
  pub value: U256,
  /// An array of rlp-serialized merkle tree nodes.
  #[serde(deserialize_with = "ueb3_serde::nullable::sequence")]
  pub proof: Vec<Bytes>,
}
