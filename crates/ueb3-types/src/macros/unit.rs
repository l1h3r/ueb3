macro_rules! define_unit {
  ($name:ident($exp:literal)) => {
    #[doc = concat!("A value of ETH denominated in ", stringify!($name), ".")]
    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct $name($crate::eth::U256);

    impl $name {
      #[doc = concat!("Creates a new `", stringify!($name), "` value from a 256-bit integer.")]
      #[inline]
      pub const fn from_u256(value: $crate::eth::U256) -> Self {
        Self(value)
      }

      /// Converts `self` into a 256-bit integer.
      #[inline]
      pub const fn into_u256(self) -> $crate::eth::U256 {
        self.0
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::fmt::{Debug/Display}
    // -------------------------------------------------------------------------

    impl ::core::fmt::Debug for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, concat!(stringify!($name), "({})"), self.0)
      }
    }

    impl ::core::fmt::Display for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{} {}", self.0, stringify!($name).to_lowercase())
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::ops::Deref/DerefMut
    // -------------------------------------------------------------------------

    impl ::core::ops::Deref for $name {
      type Target = $crate::eth::U256;

      #[inline]
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl ::core::ops::DerefMut for $name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::From (Self <-> U256)
    // -------------------------------------------------------------------------

    impl From<$crate::eth::U256> for $name {
      #[inline]
      fn from(other: $crate::eth::U256) -> Self {
        Self::from_u256(other)
      }
    }

    impl From<$name> for $crate::eth::U256 {
      #[inline]
      fn from(other: $name) -> Self {
        other.into_u256()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::str::FromStr
    // -------------------------------------------------------------------------

    impl ::core::str::FromStr for $name {
      type Err = ::ueb3_utils::base16::FromHexError;

      #[inline]
      fn from_str(input: &str) -> Result<Self, Self::Err> {
        ::ueb3_utils::base16::FromHex::from_str(input)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - Quantity
    // -------------------------------------------------------------------------

    impl ::ueb3_utils::traits::Quantity for $name {
      type Bytes = <$crate::eth::U256 as ::ueb3_utils::traits::Quantity>::Bytes;
      type Hexed = <$crate::eth::U256 as ::ueb3_utils::traits::Quantity>::Hexed;

      const BYTES: Self::Bytes = <$crate::eth::U256 as ::ueb3_utils::traits::Quantity>::BYTES;
      const HEXED: Self::Hexed = <$crate::eth::U256 as ::ueb3_utils::traits::Quantity>::HEXED;

      #[inline]
      fn from_be_bytes(bytes: Self::Bytes) -> Self {
        Self(::ueb3_utils::traits::Quantity::from_be_bytes(bytes))
      }

      #[inline]
      fn to_be_bytes(self) -> Self::Bytes {
        ::ueb3_utils::traits::Quantity::to_be_bytes(self.0)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - Base16 Encoding
    // -------------------------------------------------------------------------

    impl ::ueb3_utils::base16::FromHex for $name {
      type Error = ::ueb3_utils::base16::FromHexError;

      #[inline]
      fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
        ::ueb3_utils::traits::Quantity::from_hex(input)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - Serde
    // -------------------------------------------------------------------------

    impl<'de> ::serde::Deserialize<'de> for $name {
      #[inline]
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        ::ueb3_utils::traits::Quantity::deserialize(deserializer)
      }
    }

    impl ::serde::Serialize for $name {
      #[inline]
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        ::ueb3_utils::traits::Quantity::serialize(self, serializer)
      }
    }
  };
}
