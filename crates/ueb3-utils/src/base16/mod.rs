//! Base-16 Encoding/Decoding

use const_hex::FromHexError as HexError;
use const_hex::decode_to_array;
use const_hex::decode_to_slice;
use const_hex::encode_to_slice;
use const_hex::encode_to_slice_upper;
use serde::Deserializer;
use serde::Serializer;
use serde::de::Error;
use serde::de::Visitor;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use std::rc::Rc;
use std::str;
use std::sync::Arc;

// -----------------------------------------------------------------------------
//  Hex Error
// -----------------------------------------------------------------------------

/// The error type returned when conversion from hex-encoded data fails.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum FromHexError {
  /// The provided input contained an invalid character.
  InvalidChar(usize, char),
  /// An improperly-sized buffer was provided to a fixed-size decoding function.
  InvalidSize,
  /// The provided input has an invalid pair of bytes.
  UnevenBytes,
  /// The provided input is missing the required '0x' prefix.
  MissingPrefix,
  /// The provided input has no content.
  MissingDigits,
  /// The provided input has unexpected leading zeroes.
  LeadingZeroes,
}

impl Display for FromHexError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::InvalidChar(index, ch) => write!(f, "invalid character {ch:?} at position {index}"),
      Self::InvalidSize => f.write_str("invalid string length"),
      Self::UnevenBytes => f.write_str("odd number of digits"),
      Self::MissingPrefix => f.write_str("missing '0x' prefix"),
      Self::MissingDigits => f.write_str("missing digits"),
      Self::LeadingZeroes => f.write_str("unexpected leading zero"),
    }
  }
}

impl StdError for FromHexError {}

impl From<HexError> for FromHexError {
  #[inline]
  fn from(other: HexError) -> Self {
    match other {
      HexError::InvalidHexCharacter { c, index } => Self::InvalidChar(index, c),
      HexError::OddLength => Self::UnevenBytes,
      HexError::InvalidStringLength => Self::InvalidSize,
    }
  }
}

// -----------------------------------------------------------------------------
//  Hex Conversion Trait
// -----------------------------------------------------------------------------

/// Provides conversion utilities from hex-encoded data to concrete types.
pub trait FromHex: Sized {
  /// The error type returned when conversion fails.
  type Error: StdError;

  /// Decodes `Self` from a slice of hex bytes.
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error>;

  /// Decodes `Self` from a hex string.
  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Error> {
    FromHex::from_slice(input.as_bytes())
  }
}

// -----------------------------------------------------------------------------
//  Hex Conversion Implementations
// -----------------------------------------------------------------------------

impl<T> FromHex for Cow<'_, T>
where
  T: ToOwned + ?Sized,
  T::Owned: FromHex,
{
  type Error = <T::Owned as FromHex>::Error;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::Owned)
  }
}

impl<T> FromHex for Box<T>
where
  T: FromHex,
{
  type Error = T::Error;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::new)
  }
}

impl<T> FromHex for Rc<T>
where
  T: FromHex,
{
  type Error = T::Error;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::new)
  }
}

impl<T> FromHex for Arc<T>
where
  T: FromHex,
{
  type Error = T::Error;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Self::new)
  }
}

impl FromHex for Vec<u8> {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    decode_vec(input)
  }
}

impl FromHex for Box<[u8]> {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    FromHex::from_slice(input).map(Vec::into_boxed_slice)
  }
}

impl<const N: usize> FromHex for [u8; N] {
  type Error = FromHexError;

  #[inline]
  fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
    decode_to_array(input).map_err(Into::into)
  }
}

// -----------------------------------------------------------------------------
//  Serde Helpers
// -----------------------------------------------------------------------------

/// Serializes `T` as a hex-encoded string.
///
/// Note: When the serializer is **not** [human-readable],
///       this will serialize `T` as a slice of raw bytes.
///
/// [human-readable]: Serializer::is_human_readable
#[inline]
pub fn serialize<T, S, const PREFIX: bool>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: AsRef<[u8]> + ?Sized,
{
  if serializer.is_human_readable() {
    serializer.serialize_str(encode_str::<T, PREFIX>(data).as_str())
  } else {
    serializer.serialize_bytes(data.as_ref())
  }
}

/// Deserializes `T` from a hex-encoded string or bytes.
///
/// Note: When the deserializer is **not** [human-readable], this will
///       hint that the deserializer should expect a slice of raw bytes.
///
/// [human-readable]: Deserializer::is_human_readable
#[inline]
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  for<'a> T: FromHex + TryFrom<&'a [u8]>,
  for<'a> <T as TryFrom<&'a [u8]>>::Error: Display,
{
  struct __Visitor<T>(PhantomData<T>);

  impl<T> Visitor<'_> for __Visitor<T>
  where
    for<'a> T: FromHex + TryFrom<&'a [u8]>,
    for<'a> <T as TryFrom<&'a [u8]>>::Error: Display,
  {
    type Value = T;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
      f.write_str("a hex-encoded string or bytes")
    }

    fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
    where
      E: Error,
    {
      T::try_from(data).map_err(E::custom)
    }

    fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
    where
      E: Error,
    {
      FromHex::from_str(data).map_err(E::custom)
    }
  }

  if deserializer.is_human_readable() {
    deserializer.deserialize_str(__Visitor(PhantomData))
  } else {
    deserializer.deserialize_bytes(__Visitor(PhantomData))
  }
}

// -----------------------------------------------------------------------------
//  Generic Functions
// -----------------------------------------------------------------------------

/// Decodes `input` into a vector of bytes.
#[inline]
pub fn decode_vec<T>(input: &T) -> Result<Vec<u8>, FromHexError>
where
  T: AsRef<[u8]> + ?Sized,
{
  fn inner(data: &[u8]) -> Result<Vec<u8>, FromHexError> {
    let data: &[u8] = remove_prefix(data);
    let size: usize = data.len() / 2;
    let mut buffer: Vec<u8> = vec![0; size];

    decode_to_slice(data, &mut buffer)?;

    Ok(buffer)
  }

  inner(input.as_ref())
}

/// Decodes `input` into the given `output` buffer.
#[inline]
pub fn decode_into_slice<T>(input: &T, output: &mut [u8]) -> Result<(), FromHexError>
where
  T: AsRef<[u8]> + ?Sized,
{
  decode_to_slice(input, output).map_err(Into::into)
}

/// Encodes `input` as a lowercase hex string.
#[inline]
pub fn encode_str<T, const PREFIX: bool>(data: &T) -> String
where
  T: AsRef<[u8]> + ?Sized,
{
  fn inner<const PREFIX: bool>(data: &[u8]) -> String {
    let head: usize = usize::from(PREFIX) * 2;
    let tail: usize = data.len() * 2;
    let mut buffer: Vec<u8> = vec![0; head + tail];

    if PREFIX {
      buffer[0] = b'0';
      buffer[1] = b'x';
    }

    // NO-PANIC: only returns Err(..) when `output.len() != 2 * input.len()`
    encode_to_slice(data, &mut buffer[head..]).expect("infallible");

    // SAFETY: A hex string is valid UTF-8.
    unsafe { String::from_utf8_unchecked(buffer) }
  }

  inner::<PREFIX>(data.as_ref())
}

/// Encodes `input` as a lowercase hex string written to `output`.
#[inline]
pub fn encode_into_lower<'a, T>(input: &T, output: &'a mut [u8]) -> Result<&'a str, FromHexError>
where
  T: AsRef<[u8]> + ?Sized,
{
  encode_to_slice(input, output)?;

  // SAFETY: A hex string is valid UTF-8.
  Ok(unsafe { str::from_utf8_unchecked(output) })
}

/// Encodes `input` as an uppercase hex string written to `output`.
#[inline]
pub fn encode_into_upper<'a, T>(input: &T, output: &'a mut [u8]) -> Result<&'a str, FromHexError>
where
  T: AsRef<[u8]> + ?Sized,
{
  encode_to_slice_upper(input, output)?;

  // SAFETY: A hex string is valid UTF-8.
  Ok(unsafe { str::from_utf8_unchecked(output) })
}

// -----------------------------------------------------------------------------
//  Utility Functions
// -----------------------------------------------------------------------------

#[inline]
const fn remove_prefix(data: &[u8]) -> &[u8] {
  let [b'0', b'x', tail @ ..] = data else {
    return data;
  };

  tail
}
