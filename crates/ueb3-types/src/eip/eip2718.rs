//! Extra types defined in [EIP-2718].
//!
//! [EIP-2718]: https://eips.ethereum.org/EIPS/eip-2718

use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

// -----------------------------------------------------------------------------
// Transaction Type
// -----------------------------------------------------------------------------

/// A transaction type.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum TransactionType {
  /// Legacy transaction type.
  #[default]
  #[serde(rename = "0x0", alias = "0x00")]
  Legacy = 0,
  /// EIP-2930 transaction type.
  #[serde(rename = "0x1", alias = "0x01")]
  Eip2930 = 1,
  /// EIP-1559 transaction type.
  #[serde(rename = "0x2", alias = "0x02")]
  Eip1559 = 2,
  /// EIP-4844 transaction type.
  #[serde(rename = "0x3", alias = "0x03")]
  Eip4844 = 3,
  /// EIP-7702 transaction type.
  #[serde(rename = "0x4", alias = "0x04")]
  Eip7702 = 4,
}

impl TransactionType {
  /// Returns `true` if `self` is a legacy transaction type.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip2718::TransactionType;
  ///
  /// assert_eq!(TransactionType::Legacy.is_legacy(), true);
  /// assert_eq!(TransactionType::Eip7702.is_legacy(), false);
  /// ```
  #[inline]
  pub const fn is_legacy(&self) -> bool {
    matches!(self, Self::Legacy)
  }

  /// Returns `true` if `self` is an EIP-2930 transaction type.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip2718::TransactionType;
  ///
  /// assert_eq!(TransactionType::Eip2930.is_eip2930(), true);
  /// assert_eq!(TransactionType::Eip7702.is_eip2930(), false);
  /// ```
  #[inline]
  pub const fn is_eip2930(&self) -> bool {
    matches!(self, Self::Eip2930)
  }

  /// Returns `true` if `self` is an EIP-1559 transaction type.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip2718::TransactionType;
  ///
  /// assert_eq!(TransactionType::Eip1559.is_eip1559(), true);
  /// assert_eq!(TransactionType::Eip7702.is_eip1559(), false);
  /// ```
  #[inline]
  pub const fn is_eip1559(&self) -> bool {
    matches!(self, Self::Eip1559)
  }

  /// Returns `true` if `self` is an EIP-4844 transaction type.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip2718::TransactionType;
  ///
  /// assert_eq!(TransactionType::Eip4844.is_eip4844(), true);
  /// assert_eq!(TransactionType::Eip7702.is_eip4844(), false);
  /// ```
  #[inline]
  pub const fn is_eip4844(&self) -> bool {
    matches!(self, Self::Eip4844)
  }

  /// Returns `true` if `self` is an EIP-7702 transaction type.
  ///
  /// # Examples
  ///
  /// ```
  /// use ueb3_types::eip::eip2718::TransactionType;
  ///
  /// assert_eq!(TransactionType::Eip7702.is_eip7702(), true);
  /// assert_eq!(TransactionType::Legacy.is_eip7702(), false);
  /// ```
  #[inline]
  pub const fn is_eip7702(&self) -> bool {
    matches!(self, Self::Eip7702)
  }
}

impl Display for TransactionType {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Legacy => f.write_str("Legacy"),
      Self::Eip2930 => f.write_str("EIP-2930"),
      Self::Eip1559 => f.write_str("EIP-1559"),
      Self::Eip4844 => f.write_str("EIP-4844"),
      Self::Eip7702 => f.write_str("EIP-7702"),
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Token;
  use serde_test::assert_de_tokens;
  use serde_test::assert_tokens;

  use crate::eip::eip2718::TransactionType;

  #[test]
  fn test_serde() {
    const fn token(variant: &'static str) -> Token {
      Token::UnitVariant {
        name: "TransactionType",
        variant,
      }
    }

    assert_tokens(&TransactionType::Legacy, &[token("0x0")]);
    assert_tokens(&TransactionType::Eip2930, &[token("0x1")]);
    assert_tokens(&TransactionType::Eip1559, &[token("0x2")]);
    assert_tokens(&TransactionType::Eip4844, &[token("0x3")]);
    assert_tokens(&TransactionType::Eip7702, &[token("0x4")]);

    // -------------------------------------------------------------------------
    // Test Aliases
    // -------------------------------------------------------------------------

    assert_de_tokens(&TransactionType::Legacy, &[token("0x00")]);
    assert_de_tokens(&TransactionType::Eip2930, &[token("0x01")]);
    assert_de_tokens(&TransactionType::Eip1559, &[token("0x02")]);
    assert_de_tokens(&TransactionType::Eip4844, &[token("0x03")]);
    assert_de_tokens(&TransactionType::Eip7702, &[token("0x04")]);
  }
}
