use serde::Serialize;
use serde::Serializer;
use std::array::TryFromSliceError;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
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
use std::ptr;
use std::rc::Rc;
use std::slice;
use std::string;
use std::sync::Arc;
use std::vec;
use ueb3_utils::base16::FormatHex;

use crate::eth::Bytes;

// -----------------------------------------------------------------------------
// Generic Borrowed Bytes
// -----------------------------------------------------------------------------

/// A dynamically-sized view into a contiguous sequence of bytes.
#[derive(Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Slice([u8]);

impl Slice {
  /// Creates a new `Slice` from a borrowed byte slice.
  #[inline]
  pub const fn new(value: &[u8]) -> &Self {
    // SAFETY: Slice is a DST with the same representation as `value`.
    unsafe { &*(ptr::from_ref(value) as *const Self) }
  }

  /// Creates a new mutable `Slice` from a borrowed byte slice.
  #[inline]
  pub const fn new_mut(value: &mut [u8]) -> &mut Self {
    // SAFETY: Slice is a DST with the same representation as `value`.
    unsafe { &mut *(ptr::from_mut(value) as *mut Self) }
  }

  /// Returns a reference to the underlying byte slice.
  #[inline]
  pub const fn as_slice(&self) -> &[u8] {
    &self.0
  }

  /// Returns a mutable reference to the underlying byte slice.
  #[inline]
  pub const fn as_mut_slice(&mut self) -> &mut [u8] {
    &mut self.0
  }
}

// -----------------------------------------------------------------------------
// Implementations - Default
// -----------------------------------------------------------------------------

impl Default for &Slice {
  #[inline]
  fn default() -> Self {
    Slice::new(&[])
  }
}

impl Default for &mut Slice {
  #[inline]
  fn default() -> Self {
    Slice::new_mut(&mut [])
  }
}

// -----------------------------------------------------------------------------
// Implementations - Debug/Display/LowerHex/UpperHex
// -----------------------------------------------------------------------------

impl Debug for Slice {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerHex::fmt(self, f)
  }
}

impl Display for Slice {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    LowerHex::fmt(self, f)
  }
}

impl LowerHex for Slice {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    FormatHex::fmt_lower(self, f)
  }
}

impl UpperHex for Slice {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    FormatHex::fmt_upper(self, f)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Deref/DerefMut
// -----------------------------------------------------------------------------

impl Deref for Slice {
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl DerefMut for Slice {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - AsRef
// -----------------------------------------------------------------------------

impl AsRef<Self> for Slice {
  #[inline]
  fn as_ref(&self) -> &Self {
    self
  }
}

impl AsRef<Slice> for [u8] {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self)
  }
}

impl<const N: usize> AsRef<Slice> for [u8; N] {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl AsRef<Slice> for Vec<u8> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl AsRef<Slice> for Box<[u8]> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self)
  }
}

impl AsRef<Slice> for Rc<[u8]> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self)
  }
}

impl AsRef<Slice> for Arc<[u8]> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self)
  }
}

impl AsRef<Slice> for String {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_bytes())
  }
}

impl AsRef<Slice> for str {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_bytes())
  }
}

impl AsRef<Slice> for slice::Iter<'_, u8> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl AsRef<Slice> for slice::IterMut<'_, u8> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl AsRef<Slice> for string::Drain<'_> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_str().as_bytes())
  }
}

impl AsRef<Slice> for vec::Drain<'_, u8> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl AsRef<Slice> for vec::IntoIter<u8> {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl AsRef<[u8]> for Slice {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self.as_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - AsMut
// -----------------------------------------------------------------------------

impl AsMut<Self> for Slice {
  #[inline]
  fn as_mut(&mut self) -> &mut Self {
    self
  }
}

impl AsMut<Slice> for [u8] {
  #[inline]
  fn as_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self)
  }
}

impl<const N: usize> AsMut<Slice> for [u8; N] {
  #[inline]
  fn as_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self.as_mut_slice())
  }
}

impl AsMut<Slice> for Vec<u8> {
  #[inline]
  fn as_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self.as_mut_slice())
  }
}

impl AsMut<Slice> for Box<[u8]> {
  #[inline]
  fn as_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Borrow
// -----------------------------------------------------------------------------

impl Borrow<Slice> for [u8] {
  #[inline]
  fn borrow(&self) -> &Slice {
    Slice::new(self)
  }
}

impl<const N: usize> Borrow<Slice> for [u8; N] {
  #[inline]
  fn borrow(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl Borrow<Slice> for Vec<u8> {
  #[inline]
  fn borrow(&self) -> &Slice {
    Slice::new(self.as_slice())
  }
}

impl Borrow<Slice> for Box<[u8]> {
  #[inline]
  fn borrow(&self) -> &Slice {
    Slice::new(self)
  }
}

impl Borrow<Slice> for Rc<[u8]> {
  #[inline]
  fn borrow(&self) -> &Slice {
    Slice::new(self)
  }
}

impl Borrow<Slice> for Arc<[u8]> {
  #[inline]
  fn borrow(&self) -> &Slice {
    Slice::new(self)
  }
}

// -----------------------------------------------------------------------------
// Implementations - BorrowMut
// -----------------------------------------------------------------------------

impl BorrowMut<Slice> for [u8] {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self)
  }
}

impl<const N: usize> BorrowMut<Slice> for [u8; N] {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self.as_mut_slice())
  }
}

impl BorrowMut<Slice> for Vec<u8> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self.as_mut_slice())
  }
}

impl BorrowMut<Slice> for Box<[u8]> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut Slice {
    Slice::new_mut(self)
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (slice -> Self)
// -----------------------------------------------------------------------------

impl<'a> From<&'a [u8]> for &'a Slice {
  #[inline]
  fn from(other: &'a [u8]) -> Self {
    Slice::new(other)
  }
}

impl<'a> From<&'a mut [u8]> for &'a Slice {
  #[inline]
  fn from(other: &'a mut [u8]) -> Self {
    Slice::new(other)
  }
}

impl<'a> From<&'a mut [u8]> for &'a mut Slice {
  #[inline]
  fn from(other: &'a mut [u8]) -> Self {
    Slice::new_mut(other)
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (array -> Self)
// -----------------------------------------------------------------------------

impl<'a, const N: usize> From<&'a [u8; N]> for &'a Slice {
  #[inline]
  fn from(other: &'a [u8; N]) -> Self {
    Slice::new(other)
  }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for &'a Slice {
  #[inline]
  fn from(other: &'a mut [u8; N]) -> Self {
    Slice::new(other)
  }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for &'a mut Slice {
  #[inline]
  fn from(other: &'a mut [u8; N]) -> Self {
    Slice::new_mut(other)
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (Self -> slice)
// -----------------------------------------------------------------------------

impl<'a> From<&'a Slice> for &'a [u8] {
  #[inline]
  fn from(other: &'a Slice) -> Self {
    other.as_slice()
  }
}

impl<'a> From<&'a mut Slice> for &'a [u8] {
  #[inline]
  fn from(other: &'a mut Slice) -> Self {
    other.as_slice()
  }
}

impl<'a> From<&'a mut Slice> for &'a mut [u8] {
  #[inline]
  fn from(other: &'a mut Slice) -> Self {
    other.as_mut_slice()
  }
}

// -----------------------------------------------------------------------------
// Implementations - From (Self -> <container>)
// -----------------------------------------------------------------------------

impl From<&Slice> for Vec<u8> {
  #[inline]
  fn from(other: &Slice) -> Self {
    other.to_vec()
  }
}

impl From<&mut Slice> for Vec<u8> {
  #[inline]
  fn from(other: &mut Slice) -> Self {
    other.to_vec()
  }
}

impl From<&Slice> for Box<[u8]> {
  #[inline]
  fn from(other: &Slice) -> Self {
    other.as_slice().into()
  }
}

impl From<&mut Slice> for Box<[u8]> {
  #[inline]
  fn from(other: &mut Slice) -> Self {
    other.as_slice().into()
  }
}

impl From<&Slice> for Rc<[u8]> {
  #[inline]
  fn from(other: &Slice) -> Self {
    other.as_slice().into()
  }
}

impl From<&mut Slice> for Rc<[u8]> {
  #[inline]
  fn from(other: &mut Slice) -> Self {
    other.as_slice().into()
  }
}

impl From<&Slice> for Arc<[u8]> {
  #[inline]
  fn from(other: &Slice) -> Self {
    other.as_slice().into()
  }
}

impl From<&mut Slice> for Arc<[u8]> {
  #[inline]
  fn from(other: &mut Slice) -> Self {
    other.as_slice().into()
  }
}

// -----------------------------------------------------------------------------
// Implementations - TryFrom (Self -> array)
// -----------------------------------------------------------------------------

impl<const N: usize> TryFrom<&Slice> for [u8; N] {
  type Error = TryFromSliceError;

  #[inline]
  fn try_from(other: &Slice) -> Result<Self, Self::Error> {
    <[u8; N]>::try_from(other.as_slice())
  }
}

impl<const N: usize> TryFrom<&mut Slice> for [u8; N] {
  type Error = TryFromSliceError;

  #[inline]
  fn try_from(other: &mut Slice) -> Result<Self, Self::Error> {
    <[u8; N]>::try_from(other.as_slice())
  }
}

impl<'a, const N: usize> TryFrom<&'a Slice> for &'a [u8; N] {
  type Error = TryFromSliceError;

  #[inline]
  fn try_from(other: &'a Slice) -> Result<Self, Self::Error> {
    <&[u8; N]>::try_from(other.as_slice())
  }
}

impl<'a, const N: usize> TryFrom<&'a mut Slice> for &'a [u8; N] {
  type Error = TryFromSliceError;

  #[inline]
  fn try_from(other: &'a mut Slice) -> Result<Self, Self::Error> {
    <&[u8; N]>::try_from(other.as_slice())
  }
}

impl<'a, const N: usize> TryFrom<&'a mut Slice> for &'a mut [u8; N] {
  type Error = TryFromSliceError;

  #[inline]
  fn try_from(other: &'a mut Slice) -> Result<Self, Self::Error> {
    <&mut [u8; N]>::try_from(other.as_mut_slice())
  }
}

// -----------------------------------------------------------------------------
// Implementations - Index/IndexMut
// -----------------------------------------------------------------------------

impl<I> Index<I> for Slice
where
  [u8]: Index<I>,
{
  type Output = <[u8] as Index<I>>::Output;

  #[inline]
  fn index(&self, index: I) -> &Self::Output {
    <[u8] as Index<I>>::index(self, index)
  }
}

impl<I> IndexMut<I> for Slice
where
  [u8]: IndexMut<I>,
{
  #[inline]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    <[u8] as IndexMut<I>>::index_mut(self, index)
  }
}

// -----------------------------------------------------------------------------
// Implementations - IntoIterator
// -----------------------------------------------------------------------------

impl<'a> IntoIterator for &'a Slice {
  type Item = &'a u8;
  type IntoIter = slice::Iter<'a, u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Slice {
  type Item = &'a mut u8;
  type IntoIter = slice::IterMut<'a, u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<'a> IntoIterator for &'a Box<Slice> {
  type Item = &'a u8;
  type IntoIter = slice::Iter<'a, u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Box<Slice> {
  type Item = &'a mut u8;
  type IntoIter = slice::IterMut<'a, u8>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

// -----------------------------------------------------------------------------
// Implementations - ToOwned
// -----------------------------------------------------------------------------

impl ToOwned for Slice {
  type Owned = Bytes;

  #[inline]
  fn to_owned(&self) -> Self::Owned {
    Bytes::from_boxed(self.into())
  }
}

// -----------------------------------------------------------------------------
// Implementations - Base16 Encoding
// -----------------------------------------------------------------------------

impl FormatHex for Slice {
  type Buffer = Vec<u8>;

  #[inline]
  fn buffer(length: usize) -> Self::Buffer {
    vec![b'0'; length]
  }
}

// -----------------------------------------------------------------------------
// Implementations - Serde
// -----------------------------------------------------------------------------

impl Serialize for Slice {
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
  use serde_test::assert_ser_tokens;

  use crate::eth::Bytes;
  use crate::eth::Slice;

  #[test]
  fn test_new() {
    let input: &[u8] = b"hello world";
    let slice: &Slice = Slice::new(input);

    assert_eq!(slice.as_slice(), input);
  }

  #[test]
  fn test_to_hex() {
    let input: &[u8] = &[222, 173, 190, 175];
    let slice: &Slice = Slice::new(input);

    assert_eq!(format!("{slice}"), "0xdeadbeaf");
  }

  #[test]
  fn test_default() {
    let slice: &Slice = Default::default();

    assert_eq!(slice.len(), 0);
    assert_eq!(format!("{slice}"), "0x");
  }

  #[test]
  fn test_display_as_hex() {
    let input: &[u8] = &[222, 173, 190, 175];
    let slice: &Slice = Slice::new(input);
    let print: String = format!("{}", slice);

    assert_eq!(print, "0xdeadbeaf");
  }

  #[test]
  fn test_debug_as_hex() {
    let input: &[u8] = &[222, 173, 190, 175];
    let slice: &Slice = Slice::new(input);
    let print: String = format!("{:?}", slice);

    assert_eq!(print, "0xdeadbeaf");
  }

  #[test]
  fn test_deref_slice() {
    let slice: &Slice = Slice::new(&[]);
    let bytes: &[u8] = &**slice;

    assert_eq!(bytes.as_ptr(), slice.as_ptr());
  }

  #[test]
  fn test_as_ref() {
    let input: [u8; 11] = *b"hello world";
    let slice: &Slice = Slice::new(&input);
    let bytes: Bytes = Bytes::new(input.to_vec());
    let string_slice: &str = "hello world";
    let string_owned: String = string_slice.to_owned();

    assert_eq!(slice, slice.as_ref());
    assert_eq!(slice, input.as_slice().as_ref());
    assert_eq!(slice, input.as_ref());
    assert_eq!(slice, input.to_vec().as_ref());
    assert_eq!(slice, input.to_vec().into_boxed_slice().as_ref());
    assert_eq!(slice, bytes.as_ref());
    assert_eq!(slice, string_slice.as_ref());
    assert_eq!(slice, string_owned.as_ref());
  }

  #[test]
  fn test_to_owned() {
    let input: [u8; 4] = [222, 173, 190, 175];
    let slice: &Slice = Slice::new(&input);
    let bytes: Bytes = slice.to_owned();

    assert_eq!(&slice[..], &bytes[..]);
  }

  #[test]
  fn test_serde() {
    let input: &[u8] = &[222, 173, 190, 175];
    let slice: &Slice = Slice::new(input);

    assert_ser_tokens(&slice.readable(), &[Token::Str("0xdeadbeaf")]);
    assert_ser_tokens(&slice.compact(), &[Token::Bytes(input)]);
  }
}
