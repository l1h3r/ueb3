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
