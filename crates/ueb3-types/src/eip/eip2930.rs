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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::eip::eip2930::Access;
  use crate::eip::eip2930::AccessList;
  use crate::eip::eip2930::AccessListResult;
  use crate::eth::U256;

  #[test]
  fn test_serde() {
    const ADDRESS: &str = "0xa02457e5dfd32bda5fc7e1f1b008aa5979568150";
    const STORAGE: &str = "0x0000000000000000000000000000000000000000000000000000000000000081";

    let a: AccessListResult = AccessListResult::default();

    let b: AccessListResult = AccessListResult {
      access_list: AccessList::from([Access {
        address: ADDRESS.parse().unwrap(),
        storage_keys: vec![STORAGE.parse().unwrap()],
      }]),
      gas_used: U256::from(0x125f8),
      error: None,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "AccessListResult",
        len: 2,
      },
      Token::Str("accessList"),
      Token::Seq { len: Some(0) },
      Token::SeqEnd,
      Token::Str("gasUsed"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "AccessListResult",
        len: 2,
      },
      Token::Str("accessList"),
      Token::Seq { len: Some(1) },
      Token::Struct {
        name: "Access",
        len: 2,
      },
      Token::Str("address"),
      Token::Str(ADDRESS),
      Token::Str("storageKeys"),
      Token::Seq { len: Some(1) },
      Token::Str(STORAGE),
      Token::SeqEnd,
      Token::StructEnd,
      Token::SeqEnd,
      Token::Str("gasUsed"),
      Token::Str("0x125f8"),
      Token::StructEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
