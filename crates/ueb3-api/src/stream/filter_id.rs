use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;
use ueb3_types::eth::U256;
use ueb3_utils::base16::FromHex;
use ueb3_utils::base16::FromHexError;

// -----------------------------------------------------------------------------
// Filter Identifier
// -----------------------------------------------------------------------------

/// A unique identifier representing a remote filter.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct FilterId(U256);

impl FilterId {
  /// Creates a new `FilterId` value from a 256-bit integer.
  #[inline]
  pub const fn from_u256(value: U256) -> Self {
    Self(value)
  }

  /// Converts `self` into a 256-bit integer.
  #[inline]
  pub const fn into_u256(self) -> U256 {
    self.0
  }
}

impl Debug for FilterId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "FilterId({})", self.0)
  }
}

impl Display for FilterId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl From<U256> for FilterId {
  #[inline]
  fn from(other: U256) -> Self {
    Self::from_u256(other)
  }
}

impl From<FilterId> for U256 {
  #[inline]
  fn from(other: FilterId) -> Self {
    other.into_u256()
  }
}

impl FromStr for FilterId {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    FromHex::from_str(input)
  }
}

impl FromHex for FilterId {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::from_u256)
  }
}
