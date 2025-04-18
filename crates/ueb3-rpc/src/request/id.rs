use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// -----------------------------------------------------------------------------
// Request Id
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 request/response identifier.
///
/// Note: String and NULL identifiers are **not** supported.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct RequestId(u64);

impl RequestId {
  /// Creates a new `RequestId` from the given `value`.
  #[inline]
  pub const fn new(value: u64) -> Self {
    Self(value)
  }

  /// Returns the 64-bit integer value of the request identifier.
  #[inline]
  pub const fn get(self) -> u64 {
    self.0
  }
}

impl Debug for RequestId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "RequestId({})", self.get())
  }
}

impl Display for RequestId {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.get())
  }
}
