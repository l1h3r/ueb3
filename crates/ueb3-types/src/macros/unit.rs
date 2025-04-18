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

    impl ::core::str::FromStr for $name {
      type Err = ::ueb3_utils::base16::FromHexError;

      #[inline]
      fn from_str(input: &str) -> Result<Self, Self::Err> {
        ::ueb3_utils::base16::FromHex::from_str(input)
      }
    }

    impl ::ueb3_utils::base16::FromHex for $name {
      type Error = ::ueb3_utils::base16::FromHexError;

      #[inline]
      fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
        ::ueb3_utils::base16::FromHex::from_slice(input).map(Self::from_u256)
      }
    }

    impl<'de> ::serde::Deserialize<'de> for $name {
      #[inline]
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        ::serde::Deserialize::deserialize(deserializer).map(Self::from_u256)
      }
    }

    impl ::serde::Serialize for $name {
      #[inline]
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        ::serde::Serialize::serialize(&self.0, serializer)
      }
    }
  };
}
