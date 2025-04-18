macro_rules! define_uint {
  ($name:ident($words:tt), $doc:literal) => {
    ::uint::construct_uint! {
      #[doc = $doc]
      pub struct $name($words);
    }

    impl $name {
      const BYTES: usize = $words * size_of::<u64>();
      const HEXED: usize = Self::BYTES * 2;
    }

    impl ::ueb3_utils::traits::Quantity for $name {
      type Bytes = [u8; Self::BYTES];
      type Hexed = [u8; Self::HEXED];

      const BYTES: Self::Bytes = [0; Self::BYTES];
      const HEXED: Self::Hexed = [b'0'; Self::HEXED];

      #[inline]
      fn from_be_bytes(bytes: [u8; Self::BYTES]) -> Self {
        $name::from_big_endian(bytes.as_slice())
      }

      #[inline]
      fn to_be_bytes(self) -> [u8; Self::BYTES] {
        $name::to_big_endian(&self)
      }
    }

    impl ::ueb3_utils::base16::FromHex for $name {
      type Error = ::ueb3_utils::base16::FromHexError;

      #[inline]
      fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
        ::ueb3_utils::traits::Quantity::from_hex(input)
      }
    }

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
