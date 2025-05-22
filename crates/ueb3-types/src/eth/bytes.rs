use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;
use serde::de::Visitor;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::LowerHex;
use std::fmt::Result as FmtResult;
use std::fmt::UpperHex;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::rc::Rc;
use std::slice;
use std::str::FromStr;
use std::sync::Arc;
use std::vec;
use ueb3_utils::base16::FormatHex;
use ueb3_utils::base16::FromHex;
use ueb3_utils::base16::FromHexError;

use crate::eth::Slice;

// -----------------------------------------------------------------------------
// Generic Owned Bytes
// -----------------------------------------------------------------------------

/// A heap-allocated sequence of bytes.
#[derive(Clone, Default, Hash, PartialEq, Eq)]
pub struct Bytes(Box<[u8]>);

impl Bytes {
  /// Creates a new `Bytes` buffer from an owned vector of bytes.
  #[inline]
  pub fn new(value: Vec<u8>) -> Self {
    Self::from_boxed(value.into_boxed_slice())
  }

  /// Creates a new `Bytes` buffer from a boxed slice of bytes.
  #[inline]
  pub const fn from_boxed(value: Box<[u8]>) -> Self {
    Self(value)
  }

  /// Returns a reference to the underlying byte slice.
  #[inline]
  pub const fn as_slice(&self) -> &Slice {
    Slice::new(&self.0)
  }

  /// Returns a mutable reference to the underlying byte slice.
  #[inline]
  pub const fn as_mut_slice(&mut self) -> &mut Slice {
    Slice::new_mut(&mut self.0)
  }

  /// Converts `self` into a boxed slice of bytes.
  #[inline]
  pub fn into_boxed_slice(self) -> Box<[u8]> {
    self.0
  }
}

// -----------------------------------------------------------------------------
// Implementations - Debug/Display/LowerHex/UpperHex
// -----------------------------------------------------------------------------

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

impl LowerHex for Bytes {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerHex::fmt(self.as_slice(), f)
  }
}

impl UpperHex for Bytes {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    UpperHex::fmt(self.as_slice(), f)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Deref/DerefMut
// -----------------------------------------------------------------------------

impl Deref for Bytes {
  type Target = Slice;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl DerefMut for Bytes {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - AsRef
// -----------------------------------------------------------------------------

impl AsRef<Self> for Bytes {
  #[inline]
  fn as_ref(&self) -> &Self {
    self
  }
}

impl AsRef<Slice> for Bytes {
  #[inline]
  fn as_ref(&self) -> &Slice {
    self.as_slice()
  }
}

impl AsRef<[u8]> for Bytes {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self.as_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - AsMut
// -----------------------------------------------------------------------------

impl AsMut<Self> for Bytes {
  #[inline]
  fn as_mut(&mut self) -> &mut Self {
    self
  }
}

impl AsMut<Slice> for Bytes {
  #[inline]
  fn as_mut(&mut self) -> &mut Slice {
    self.as_mut_slice()
  }
}

impl AsMut<[u8]> for Bytes {
  #[inline]
  fn as_mut(&mut self) -> &mut [u8] {
    self.as_mut_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - Borrow
// -----------------------------------------------------------------------------

impl Borrow<Slice> for Bytes {
  #[inline]
  fn borrow(&self) -> &Slice {
    self.as_slice()
  }
}

impl Borrow<[u8]> for Bytes {
  #[inline]
  fn borrow(&self) -> &[u8] {
    self.as_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - BorrowMut
// -----------------------------------------------------------------------------

impl BorrowMut<Slice> for Bytes {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Slice {
    self.as_mut_slice()
  }
}

impl BorrowMut<[u8]> for Bytes {
  #[inline]
  fn borrow_mut(&mut self) -> &mut [u8] {
    self.as_mut_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (slice -> Self)
// -----------------------------------------------------------------------------

impl From<&[u8]> for Bytes {
  #[inline]
  fn from(other: &[u8]) -> Self {
    Self::from_boxed(other.into())
  }
}

impl From<&mut [u8]> for Bytes {
  #[inline]
  fn from(other: &mut [u8]) -> Self {
    Self::from_boxed(other.into())
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (Slice -> Self)
// -----------------------------------------------------------------------------

impl From<&Slice> for Bytes {
  #[inline]
  fn from(other: &Slice) -> Self {
    Self::from_boxed(other.into())
  }
}

impl From<&mut Slice> for Bytes {
  #[inline]
  fn from(other: &mut Slice) -> Self {
    Self::from_boxed(other.into())
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (array -> Self)
// -----------------------------------------------------------------------------

impl<const N: usize> From<&[u8; N]> for Bytes {
  #[inline]
  fn from(other: &[u8; N]) -> Self {
    Self::from_boxed(other.as_slice().into())
  }
}

impl<const N: usize> From<&mut [u8; N]> for Bytes {
  #[inline]
  fn from(other: &mut [u8; N]) -> Self {
    Self::from_boxed(other.as_slice().into())
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (<container> -> Self)
// -----------------------------------------------------------------------------

impl From<Vec<u8>> for Bytes {
  #[inline]
  fn from(other: Vec<u8>) -> Self {
    Self::new(other)
  }
}

impl From<Box<[u8]>> for Bytes {
  #[inline]
  fn from(other: Box<[u8]>) -> Self {
    Self::from_boxed(other)
  }
}

impl From<String> for Bytes {
  #[inline]
  fn from(other: String) -> Self {
    Self::new(other.into_bytes())
  }
}

impl From<&str> for Bytes {
  #[inline]
  fn from(other: &str) -> Self {
    Self::from_boxed(other.as_bytes().into())
  }
}

impl<'a> From<Cow<'a, [u8]>> for Bytes {
  #[inline]
  fn from(other: Cow<'a, [u8]>) -> Self {
    Self::new(other.into_owned())
  }
}

impl<'a> From<Cow<'a, Slice>> for Bytes {
  #[inline]
  fn from(other: Cow<'a, Slice>) -> Self {
    other.into_owned()
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (Self -> <container>)
// -----------------------------------------------------------------------------

impl From<Bytes> for Vec<u8> {
  #[inline]
  fn from(other: Bytes) -> Self {
    other.into_boxed_slice().into()
  }
}

impl From<Bytes> for Box<[u8]> {
  #[inline]
  fn from(other: Bytes) -> Self {
    other.into_boxed_slice()
  }
}

impl From<Bytes> for Rc<[u8]> {
  #[inline]
  fn from(other: Bytes) -> Self {
    other.into_boxed_slice().into()
  }
}

impl From<Bytes> for Arc<[u8]> {
  #[inline]
  fn from(other: Bytes) -> Self {
    other.into_boxed_slice().into()
  }
}

impl<'a> From<&'a Bytes> for Cow<'a, [u8]> {
  #[inline]
  fn from(other: &'a Bytes) -> Self {
    Cow::Borrowed(other.as_slice())
  }
}

impl From<Bytes> for Cow<'_, [u8]> {
  #[inline]
  fn from(other: Bytes) -> Self {
    Cow::Owned(other.into())
  }
}

impl<'a> From<&'a Bytes> for Cow<'a, Slice> {
  #[inline]
  fn from(other: &'a Bytes) -> Self {
    Cow::Borrowed(other.as_slice())
  }
}

impl From<Bytes> for Cow<'_, Slice> {
  #[inline]
  fn from(other: Bytes) -> Self {
    Cow::Owned(other)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Index/IndexMut
// -----------------------------------------------------------------------------

impl<I> Index<I> for Bytes
where
  Slice: Index<I>,
{
  type Output = <Slice as Index<I>>::Output;

  #[inline]
  fn index(&self, index: I) -> &Self::Output {
    <Slice as Index<I>>::index(self, index)
  }
}

impl<I> IndexMut<I> for Bytes
where
  Slice: IndexMut<I>,
{
  #[inline]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    <Slice as IndexMut<I>>::index_mut(self, index)
  }
}

// -----------------------------------------------------------------------------
// Implementations - IntoIterator
// -----------------------------------------------------------------------------

impl IntoIterator for Bytes {
  type Item = u8;
  type IntoIter = vec::IntoIter<u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.into_boxed_slice().into_iter()
  }
}

impl<'a> IntoIterator for &'a Bytes {
  type Item = &'a u8;
  type IntoIter = slice::Iter<'a, u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Bytes {
  type Item = &'a mut u8;
  type IntoIter = slice::IterMut<'a, u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

// -----------------------------------------------------------------------------
// Implementations - FromIterator
// -----------------------------------------------------------------------------

impl FromIterator<u8> for Bytes {
  #[inline]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = u8>,
  {
    Self::new(Vec::from_iter(iter))
  }
}

// -----------------------------------------------------------------------------
// Implementations - FromStr
// -----------------------------------------------------------------------------

impl FromStr for Bytes {
  type Err = FromHexError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    FromHex::from_str(input)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Base16 Encoding
// -----------------------------------------------------------------------------

impl FormatHex for Bytes {
  type Buffer = <Slice as FormatHex>::Buffer;

  #[inline]
  fn buffer(length: usize) -> Self::Buffer {
    <Slice as FormatHex>::buffer(length)
  }
}

impl FromHex for Bytes {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::new)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Serde
// -----------------------------------------------------------------------------

impl<'de> Deserialize<'de> for Bytes {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl Visitor<'_> for __Visitor {
      type Value = Bytes;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a hex-encoded string or bytes")
      }

      fn visit_byte_buf<E>(self, data: Vec<u8>) -> Result<Self::Value, E>
      where
        E: Error,
      {
        Ok(data.into())
      }

      fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
      where
        E: Error,
      {
        Ok(data.into())
      }

      fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
      where
        E: Error,
      {
        FromStr::from_str(data).map_err(E::custom)
      }
    }

    if deserializer.is_human_readable() {
      deserializer.deserialize_str(__Visitor)
    } else {
      deserializer.deserialize_byte_buf(__Visitor)
    }
  }
}

impl Serialize for Bytes {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if serializer.is_human_readable() {
      serializer.collect_str(self)
    } else {
      serializer.serialize_bytes(self)
    }
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
    let slice: &[u8] = &[222, 173, 190, 175];
    let input: Vec<u8> = slice.to_vec();
    let bytes: Bytes = Bytes::new(input);

    assert_tokens(&bytes.clone().readable(), &[Token::Str("0xdeadbeaf")]);
    assert_tokens(&bytes.compact(), &[Token::Bytes(slice)]);
  }
}
