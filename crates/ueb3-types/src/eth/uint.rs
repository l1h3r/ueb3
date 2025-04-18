#![expect(missing_docs, reason = "macro code")]
#![expect(clippy::manual_div_ceil, reason = "macro code")]
#![expect(clippy::reversed_empty_ranges, reason = "macro code")]

define_uint!(U64(1), "The 64-bit unsigned integer type.");
define_uint!(U128(2), "The 128-bit unsigned integer type.");
define_uint!(U256(4), "The 256-bit unsigned integer type.");
define_uint!(U512(8), "The 512-bit unsigned integer type.");

// Re-export for convenience when parsing integers.
pub use uint::FromDecStrErr;
pub use uint::FromHexError as FromHexStrErr;
pub use uint::FromStrRadixErr;
pub use uint::FromStrRadixErrKind;

impl U64 {
  /// Creates a new `U64` from a native 64-bit integer.
  #[inline]
  pub const fn from_u64(value: u64) -> Self {
    Self([value])
  }

  /// Converts `self` into a native 64-bit integer.
  #[inline]
  pub const fn into_u64(self) -> u64 {
    self.0[0]
  }
}

#[cfg(test)]
mod tests {
  use serde_json::Value;
  use serde_json::from_value;
  use serde_json::to_value;

  use crate::eth::U64;
  use crate::eth::U128;
  use crate::eth::U256;
  use crate::eth::U512;

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
        let a: Value = Value::String("0x0".to_owned()); // 0
        let b: Value = Value::String("0x41".to_owned()); // 65
        let c: Value = Value::String("0x400".to_owned()); // 1024

        assert_eq!(<$type>::from(0), from_value(a.clone()).unwrap());
        assert_eq!(<$type>::from(65), from_value(b.clone()).unwrap());
        assert_eq!(<$type>::from(1024), from_value(c.clone()).unwrap());

        assert_eq!(to_value(<$type>::from(0)).unwrap(), a);
        assert_eq!(to_value(<$type>::from(65)).unwrap(), b);
        assert_eq!(to_value(<$type>::from(1024)).unwrap(), c);
      }

      #[test]
      #[should_panic = "missing '0x' prefix"]
      fn $serde_missing_prefix() {
        let _: $type = from_value(Value::String("ff".to_owned())).unwrap();
      }

      #[test]
      #[should_panic = "missing digits"]
      fn $serde_missing_digits() {
        let _: $type = from_value(Value::String("0x".to_owned())).unwrap();
      }

      #[test]
      #[should_panic = "unexpected leading zero"]
      fn $serde_leading_zeroes() {
        let _: $type = from_value(Value::String("0x0400".to_owned())).unwrap();
      }
    };
  }

  test!(
    test_serde_u64,
    test_serde_missing_prefix_u64,
    test_serde_missing_digits_u64,
    test_serde_leading_zeroes_u64,
    U64
  );

  test!(
    test_serde_u128,
    test_serde_missing_prefix_u128,
    test_serde_missing_digits_u128,
    test_serde_leading_zeroes_u128,
    U128
  );

  test!(
    test_serde_u256,
    test_serde_missing_prefix_u256,
    test_serde_missing_digits_u256,
    test_serde_leading_zeroes_u256,
    U256
  );

  test!(
    test_serde_u512,
    test_serde_missing_prefix_u512,
    test_serde_missing_digits_u512,
    test_serde_leading_zeroes_u512,
    U512
  );
}
