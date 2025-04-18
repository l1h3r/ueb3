//! Extra types defined in [EIP-2930].
//!
//! [EIP-2930]: https://eips.ethereum.org/EIPS/eip-2930

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Address;
use crate::eth::H256;
use crate::eth::U256;

// -----------------------------------------------------------------------------
// Access List Item
// -----------------------------------------------------------------------------

/// An address and list of storage keys that a transaction plans to access.
///
/// Note: Accesses outside the list are possible, but become more expensive.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Access {
  /// The account address to load at the start of transaction execution.
  pub address: Address,
  /// The list of storage keys to load at the start of transaction execution.
  pub storage_keys: Vec<H256>,
}

// -----------------------------------------------------------------------------
// Access List
// -----------------------------------------------------------------------------

define_list! {
  AccessList = [Access]
}

// -----------------------------------------------------------------------------
// Access List Result
// -----------------------------------------------------------------------------

/// An `AccessList` with additional gas requirement.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AccessListResult {
  /// The generated access list.
  pub access_list: AccessList,
  /// An estimate of the amount of gas used by the access list.
  pub gas_used: U256,
  /// The error message if the transaction would have failed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<String>,
}

impl Debug for AccessListResult {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("AccessListResult")
      .field("access_list", &self.access_list)
      .field("gas_used", &self.gas_used)
      .field("error", &self.error.format())
      .finish()
  }
}
