use serde::Deserialize;
use serde_json::Error as SerdeError;
use serde_json::Value;
use serde_json::value::RawValue;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Read;

mod private {
  pub trait Sealed {}
}

impl<'de, T: Deserialize<'de>> private::Sealed for T {}
impl<'de, T: Deserialize<'de>> Decode<'de> for T {}
impl<T: for<'de> Decode<'de>> DecodeOwned for T {}

// -----------------------------------------------------------------------------
// Decode Error
// -----------------------------------------------------------------------------

/// The error type returned when failing to deserialize JSON data.
pub struct DecodeError(SerdeError);

impl Debug for DecodeError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.0, f)
  }
}

impl Display for DecodeError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl StdError for DecodeError {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    self.0.source()
  }
}

// -----------------------------------------------------------------------------
// Decode JSON
// -----------------------------------------------------------------------------

/// A convenience wrapper around [`serde_json`] deserialization.
pub trait Decode<'de>: private::Sealed + Deserialize<'de> {
  /// Deserializes `Self` from a string of JSON text.
  ///
  /// # Errors
  ///
  /// Returns [`DecodeError`] if deserialization fails.
  #[inline]
  fn decode(string: &'de str) -> Result<Self, DecodeError> {
    serde_json::from_str(string).map_err(DecodeError)
  }

  /// Deserializes `Self` from a slice of JSON text.
  ///
  /// # Errors
  ///
  /// Returns [`DecodeError`] if deserialization fails.
  #[inline]
  fn decode_slice(slice: &'de [u8]) -> Result<Self, DecodeError> {
    serde_json::from_slice(slice).map_err(DecodeError)
  }

  /// Deserializes `Self` from a raw JSON [`value`][RawValue].
  ///
  /// # Errors
  ///
  /// Returns [`DecodeError`] if deserialization fails.
  #[inline]
  fn decode_raw_value(value: &'de RawValue) -> Result<Self, DecodeError> {
    Self::deserialize(value).map_err(DecodeError)
  }
}

// -----------------------------------------------------------------------------
// Decode Owned JSON
// -----------------------------------------------------------------------------

/// A convenience wrapper around [`serde_json`] deserialization.
pub trait DecodeOwned: for<'de> Decode<'de> {
  /// Deserializes `Self` from a JSON I/O stream.
  ///
  /// # Errors
  ///
  /// Returns [`DecodeError`] if deserialization fails.
  #[inline]
  fn decode_reader<R>(reader: R) -> Result<Self, DecodeError>
  where
    R: Read,
  {
    serde_json::from_reader(reader).map_err(DecodeError)
  }

  /// Deserializes `Self` from a JSON [`value`][Value].
  ///
  /// # Errors
  ///
  /// Returns [`DecodeError`] if deserialization fails.
  #[inline]
  fn decode_value(value: Value) -> Result<Self, DecodeError> {
    serde_json::from_value(value).map_err(DecodeError)
  }

  /// Deserializes `self` from a JSON byte vector.
  ///
  /// # Errors
  ///
  /// Returns [`DecodeError`] if deserialization fails.
  #[inline]
  fn decode_vec(value: Vec<u8>) -> Result<Self, DecodeError> {
    Self::decode_slice(value.as_slice())
  }
}
