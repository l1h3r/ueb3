use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;
use std::str::FromStr;
use ueb3_utils::base16;
use ueb3_utils::base16::FromHex;
use ueb3_utils::base16::FromHexError;

use crate::eth::Slice;

// -----------------------------------------------------------------------------
// Generic Owned Bytes
// -----------------------------------------------------------------------------

/// A heap-allocated sequence of bytes.
#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct Bytes {
  inner: Box<[u8]>,
}

impl Bytes {
  /// Creates a new `Bytes` buffer from an owned vector of bytes.
  #[inline]
  pub fn new(inner: Vec<u8>) -> Self {
    Self::from_boxed(inner.into_boxed_slice())
  }

  /// Creates a new `Bytes` buffer from a boxed slice of bytes.
  #[inline]
  pub const fn from_boxed(inner: Box<[u8]>) -> Self {
    Self { inner }
  }

  /// Returns a reference to the underlying byte slice.
  #[inline]
  pub const fn as_slice(&self) -> &Slice {
    Slice::new(&self.inner)
  }
}

impl Debug for Bytes {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self.as_slice(), f)
  }
}

impl Display for Bytes {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_slice(), f)
  }
}

impl Deref for Bytes {
  type Target = Slice;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl Borrow<Slice> for Bytes {
  #[inline]
  fn borrow(&self) -> &Slice {
    self.as_slice()
  }
}

impl FromStr for Bytes {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    FromHex::from_str(input)
  }
}

impl FromHex for Bytes {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::new)
  }
}

impl<'de> Deserialize<'de> for Bytes {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    base16::deserialize(deserializer).map(Self::from_boxed)
  }
}

impl Serialize for Bytes {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_slice().serialize(serializer)
  }
}

#[cfg(test)]
mod tests {
  use std::borrow::Borrow;
  use ueb3_utils::base16::FromHex;

  use crate::eth::Bytes;
  use crate::eth::Slice;

  #[test]
  fn test_new() {
    let input: Vec<u8> = vec![222, 173, 190, 175];
    let bytes: Bytes = Bytes::new(input.clone());

    assert_eq!(&bytes[..], &input[..]);
  }

  #[test]
  fn test_new_boxed() {
    let input: Box<[u8]> = Box::new([222, 173, 190, 175]);
    let bytes: Bytes = Bytes::from_boxed(input.clone());

    assert_eq!(&bytes[..], &input[..]);
  }

  #[test]
  fn test_display_as_hex() {
    let input: Vec<u8> = vec![222, 173, 190, 175];
    let bytes: Bytes = Bytes::new(input.clone());
    let print: String = format!("{}", bytes);

    assert_eq!(print, "0xdeadbeaf");
  }

  #[test]
  fn test_debug_as_hex() {
    let input: Vec<u8> = vec![222, 173, 190, 175];
    let bytes: Bytes = Bytes::new(input.clone());
    let print: String = format!("{:?}", bytes);

    assert_eq!(print, "0xdeadbeaf");
  }

  #[test]
  fn test_deref_slice() {
    let bytes: Bytes = Bytes::new(Vec::new());
    let slice: &Slice = &*bytes;

    assert_eq!(bytes.as_ptr(), slice.as_ptr());
  }

  #[test]
  fn test_borrow() {
    let input: Vec<u8> = vec![222, 173, 190, 175];
    let bytes: Bytes = Bytes::new(input);

    assert_eq!(bytes.as_slice(), bytes.borrow());
  }

  #[test]
  fn test_from_str() {
    let input: &str = "0xdeadbeaf";
    let bytes: Bytes = input.parse().unwrap();

    assert_eq!(bytes.to_string(), input);
  }

  #[test]
  fn test_from_hex() {
    let input: &str = "0xdeadbeaf";
    let bytes: Bytes = FromHex::from_str(input).unwrap();

    assert_eq!(bytes.to_string(), input);
  }

  #[test]
  fn test_serde() {
    let input: Vec<u8> = vec![222, 173, 190, 175];
    let bytes: Bytes = Bytes::new(input);

    let into_json: String = serde_json::to_string(&bytes).unwrap();
    let from_json: Bytes = serde_json::from_str(into_json.as_str()).unwrap();

    assert_eq!(into_json, "\"0xdeadbeaf\"");
    assert_eq!(from_json, bytes);
  }
}
