use serde::Serialize;
use serde_json::Error as SerdeError;
use serde_json::Value;
use serde_json::value::RawValue;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

mod private {
  pub trait Sealed {}
}

impl<T: Serialize> private::Sealed for T {}
impl<T: Serialize> Encode for T {}

// -----------------------------------------------------------------------------
// Encode Error
// -----------------------------------------------------------------------------

/// The error type returned when failing to serialize JSON data.
pub struct EncodeError(SerdeError);

impl Debug for EncodeError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.0, f)
  }
}

impl Display for EncodeError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl StdError for EncodeError {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    self.0.source()
  }
}

// -----------------------------------------------------------------------------
// Encode JSON
// -----------------------------------------------------------------------------

/// A convenience wrapper around [`serde_json`] serialization.
pub trait Encode: private::Sealed + Serialize {
  /// Serializes `self` as a JSON string.
  ///
  /// # Errors
  ///
  /// Returns [`EncodeError`] if serialization fails.
  #[inline]
  fn encode(&self) -> Result<String, EncodeError> {
    serde_json::to_string(self).map_err(EncodeError)
  }

  /// Serializes `self` as a JSON byte vector.
  ///
  /// # Errors
  ///
  /// Returns [`EncodeError`] if serialization fails.
  #[inline]
  fn encode_vec(&self) -> Result<Vec<u8>, EncodeError> {
    serde_json::to_vec(self).map_err(EncodeError)
  }

  /// Serializes `self` as a JSON [`value`][Value].
  ///
  /// # Errors
  ///
  /// Returns [`EncodeError`] if serialization fails.
  #[inline]
  fn encode_value(&self) -> Result<Value, EncodeError> {
    serde_json::to_value(self).map_err(EncodeError)
  }

  /// Serializes `self` as a raw JSON [`value`][RawValue].
  ///
  /// # Errors
  ///
  /// Returns [`EncodeError`] if serialization fails.
  #[inline]
  fn encode_raw_value(&self) -> Result<Box<RawValue>, EncodeError> {
    serde_json::value::to_raw_value(self).map_err(EncodeError)
  }
}
