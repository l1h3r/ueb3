//! Serde utility functions for possibly `null` values.

use serde::Deserialize;
use serde::Deserializer;

/// Deserializes an instance of `T` that might be received as `null`.
#[inline]
pub fn default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: Deserialize<'de> + Default,
  D: Deserializer<'de>,
{
  Deserialize::deserialize(deserializer).map(Option::unwrap_or_default)
}

/// Deserializes a sequence of `T` that might be received as `null`.
#[inline]
pub fn sequence<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
  T: Deserialize<'de>,
  D: Deserializer<'de>,
{
  Deserialize::deserialize(deserializer).map(Option::unwrap_or_default)
}
