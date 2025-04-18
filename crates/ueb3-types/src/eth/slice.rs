use serde::Serialize;
use serde::Serializer;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;
use ueb3_utils::base16;

use crate::eth::Bytes;

// -----------------------------------------------------------------------------
// Generic Borrowed Bytes
// -----------------------------------------------------------------------------

/// A dynamically-sized view into a contiguous sequence of bytes.
#[derive(Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct Slice {
  inner: [u8],
}

impl Slice {
  /// Creates a new `Slice` from a borrowed byte slice.
  #[inline]
  pub const fn new(inner: &[u8]) -> &Self {
    // SAFETY: Slice is a DST with the same representation as `inner`.
    unsafe { &*(ptr::from_ref(inner) as *const Self) }
  }

  /// Creates a new mutable `Slice` from a borrowed byte slice.
  #[inline]
  pub const fn new_mut(inner: &mut [u8]) -> &mut Self {
    // SAFETY: Slice is a DST with the same representation as `inner`.
    unsafe { &mut *(ptr::from_mut(inner) as *mut Self) }
  }

  /// Returns a reference to the underlying byte slice.
  #[inline]
  pub const fn as_slice(&self) -> &[u8] {
    &self.inner
  }

  /// Returns a mutable reference to the underlying byte slice.
  #[inline]
  pub const fn as_mut_slice(&mut self) -> &mut [u8] {
    &mut self.inner
  }

  #[inline]
  fn to_hex(&self) -> String {
    base16::encode_str::<Self, true>(self)
  }
}

impl Default for &Slice {
  #[inline]
  fn default() -> Self {
    Slice::new(&[])
  }
}

impl Debug for Slice {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.to_hex(), f)
  }
}

impl Display for Slice {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.to_hex(), f)
  }
}

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

impl<const S: usize> AsRef<Slice> for [u8; S] {
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

impl AsRef<Slice> for Bytes {
  #[inline]
  fn as_ref(&self) -> &Slice {
    self.borrow()
  }
}

impl AsRef<Slice> for str {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_bytes())
  }
}

impl AsRef<Slice> for String {
  #[inline]
  fn as_ref(&self) -> &Slice {
    Slice::new(self.as_bytes())
  }
}

impl AsRef<[u8]> for Slice {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self.as_slice()
  }
}

impl ToOwned for Slice {
  type Owned = Bytes;

  #[inline]
  fn to_owned(&self) -> Self::Owned {
    Bytes::new(self.inner.into())
  }
}

impl Serialize for Slice {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    base16::serialize::<Self, S, true>(self, serializer)
  }
}

#[cfg(test)]
mod tests {
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

    assert_eq!(slice.to_hex(), "0xdeadbeaf");
  }

  #[test]
  fn test_default() {
    let slice: &Slice = Default::default();

    assert_eq!(slice.len(), 0);
    assert_eq!(slice.to_hex(), "0x");
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
    let serde: String = serde_json::to_string(slice).unwrap();

    assert_eq!(serde, "\"0xdeadbeaf\"");
  }
}
