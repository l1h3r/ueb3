use serde::Deserializer;
use serde::Serializer;
use serde::de::Error;
use serde::de::Visitor;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;

use crate::base16;
use crate::base16::FromHexError;

// -----------------------------------------------------------------------------
// Quantity
// -----------------------------------------------------------------------------

/// An integer type that can be serialized/deserialized as a quantity value.
pub trait Quantity: Copy {
  /// The representation of `Self` as byte array.
  type Bytes: AsRef<[u8]> + AsMut<[u8]>;

  /// The representation of `Self` as a hex-encoded byte array.
  type Hexed: AsRef<[u8]> + AsMut<[u8]>;

  /// A byte array filled with `0`.
  const BYTES: Self::Bytes;

  /// A byte array filled with the char '0'.
  const HEXED: Self::Hexed;

  /// Creates a native endian integer value from its
  /// representation as a byte array in big endian.
  fn from_be_bytes(bytes: Self::Bytes) -> Self;

  /// Returns the memory representation of this integer
  /// as a byte array in big-endian (network) byte order.
  fn to_be_bytes(self) -> Self::Bytes;

  /// Decodes `Self` from a slice of hex bytes.
  #[inline]
  fn from_hex(input: &[u8]) -> Result<Self, FromHexError> {
    if input == b"0x0" {
      return Ok(Self::from_be_bytes(Self::BYTES));
    }

    let Some(mut input) = input.strip_prefix(b"0x") else {
      return Err(FromHexError::MissingPrefix);
    };

    if input.is_empty() {
      return Err(FromHexError::MissingDigits);
    }

    if input[0] == b'0' {
      return Err(FromHexError::LeadingZeroes);
    }

    let hex_bytes: usize = size_of::<Self::Hexed>();
    let mut hexed: Self::Hexed;

    if input.len() > hex_bytes {
      return Err(FromHexError::InvalidSize);
    }

    if input.len() % 2 != 0 {
      hexed = Self::HEXED;
      hexed.as_mut()[hex_bytes - input.len()..].copy_from_slice(input);
      input = &hexed.as_ref()[hex_bytes - input.len() - 1..];
    }

    let mut output: Self::Bytes = Self::BYTES;

    let offset: usize = output.as_ref().len() - input.len() / 2;
    let buffer: &mut [u8] = &mut output.as_mut()[offset..];

    base16::decode_into_slice(input, buffer)?;

    Ok(Self::from_be_bytes(output))
  }

  /// Deserializes `Self` as a quantity value.
  #[inline]
  fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor<T>(PhantomData<T>);

    impl<T> Visitor<'_> for __Visitor<T>
    where
      T: Quantity,
    {
      type Value = T;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a hex-encoded quantity or bytes")
      }

      fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
      where
        E: Error,
      {
        #[cold]
        fn error<T, E>(length: usize) -> E
        where
          T: Quantity,
          E: Error,
        {
          E::invalid_length(
            length,
            &format!("0 to {} bytes", size_of::<T::Bytes>()).as_str(),
          )
        }

        let max_bytes: usize = size_of::<T::Bytes>();
        let mut bytes: T::Bytes = T::BYTES;

        if data.len() <= max_bytes {
          let offset: usize = max_bytes - data.len();
          let window: &mut [u8] = &mut bytes.as_mut()[offset..];

          window.copy_from_slice(data);

          Ok(T::from_be_bytes(bytes))
        } else {
          Err(error::<T, E>(data.len()))
        }
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: Error,
      {
        T::from_hex(value.as_bytes()).map_err(E::custom)
      }
    }

    if deserializer.is_human_readable() {
      deserializer.deserialize_str(__Visitor(PhantomData))
    } else {
      deserializer.deserialize_bytes(__Visitor(PhantomData))
    }
  }

  /// Serializes `self` as a quantity value.
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let bytes: Self::Bytes = self.to_be_bytes();
    let slice: &[u8] = bytes.as_ref();

    if serializer.is_human_readable() {
      let index: Option<usize> = slice.iter().position(|&byte| byte != 0);
      let input: &[u8] = &slice[index.unwrap_or(slice.len())..];

      if input.is_empty() {
        return serializer.serialize_str("0x0");
      }

      // Don't add prefix so we can strip leading `0` if present.
      let string: String = base16::encode_str::<[u8], false>(input);
      let output: &str = string.trim_start_matches('0');

      serializer.collect_str(&format_args!("0x{output}"))
    } else {
      serializer.serialize_bytes(slice)
    }
  }
}

// -----------------------------------------------------------------------------
// Default Quantity Implementations
// -----------------------------------------------------------------------------

macro_rules! implement {
  ($type:ty) => {
    const _: () = {
      const BYTES: usize = size_of::<$type>();
      const HEXED: usize = size_of::<$type>() * 2;

      impl Quantity for $type {
        type Bytes = [u8; BYTES];
        type Hexed = [u8; HEXED];

        const BYTES: Self::Bytes = [0; BYTES];
        const HEXED: Self::Hexed = [b'0'; HEXED];

        #[inline]
        fn from_be_bytes(bytes: [u8; BYTES]) -> Self {
          <$type>::from_be_bytes(bytes)
        }

        #[inline]
        fn to_be_bytes(self) -> [u8; BYTES] {
          <$type>::to_be_bytes(self)
        }
      }
    };
  };
}

implement!(u8);
implement!(u16);
implement!(u32);
implement!(u64);
implement!(u128);

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde::Serialize;
  use serde_json::Value;
  use serde_json::from_value;
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::traits::Quantity;

  #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
  #[serde(transparent)]
  struct Test<T: Quantity> {
    #[serde(
      deserialize_with = "<T as Quantity>::deserialize",
      serialize_with = "<T as Quantity>::serialize"
    )]
    inner: T,
  }

  macro_rules! test {
    (
      $serde:ident,
      $serde_missing_prefix:ident,
      $serde_missing_digits:ident,
      $serde_leading_zeroes:ident,
      $type:ty
    ) => {
      #[test]
      fn $serde() {
        const A: $type = 0;
        const B: $type = 65;
        const C: $type = 255;

        const A_BYTES: [u8; size_of::<$type>()] = A.to_be_bytes();
        const B_BYTES: [u8; size_of::<$type>()] = B.to_be_bytes();
        const C_BYTES: [u8; size_of::<$type>()] = C.to_be_bytes();

        let a: Test<$type> = Test { inner: A };
        let b: Test<$type> = Test { inner: B };
        let c: Test<$type> = Test { inner: C };

        assert_tokens(&a.readable(), &[Token::Str("0x0")]);
        assert_tokens(&b.readable(), &[Token::Str("0x41")]);
        assert_tokens(&c.readable(), &[Token::Str("0xff")]);

        assert_tokens(&a.compact(), &[Token::Bytes(A_BYTES.as_slice())]);
        assert_tokens(&b.compact(), &[Token::Bytes(B_BYTES.as_slice())]);
        assert_tokens(&c.compact(), &[Token::Bytes(C_BYTES.as_slice())]);
      }

      #[test]
      #[should_panic = "missing '0x' prefix"]
      fn $serde_missing_prefix() {
        let _: Test<$type> = from_value(Value::String("ff".to_owned())).unwrap();
      }

      #[test]
      #[should_panic = "missing digits"]
      fn $serde_missing_digits() {
        let _: Test<$type> = from_value(Value::String("0x".to_owned())).unwrap();
      }

      #[test]
      #[should_panic = "unexpected leading zero"]
      fn $serde_leading_zeroes() {
        let _: Test<$type> = from_value(Value::String("0x0400".to_owned())).unwrap();
      }
    };
  }

  test!(
    test_serde_u8,
    test_serde_missing_prefix_u8,
    test_serde_missing_digits_u8,
    test_serde_leading_zeroes_u8,
    u8
  );

  test!(
    test_serde_u16,
    test_serde_missing_prefix_u16,
    test_serde_missing_digits_u16,
    test_serde_leading_zeroes_u16,
    u16
  );

  test!(
    test_serde_u32,
    test_serde_missing_prefix_u32,
    test_serde_missing_digits_u32,
    test_serde_leading_zeroes_u32,
    u32
  );

  test!(
    test_serde_u64,
    test_serde_missing_prefix_u64,
    test_serde_missing_digits_u64,
    test_serde_leading_zeroes_u64,
    u64
  );

  test!(
    test_serde_u128,
    test_serde_missing_prefix_u128,
    test_serde_missing_digits_u128,
    test_serde_leading_zeroes_u128,
    u128
  );
}
