macro_rules! implement_binop {
  // Implementation - Binary Op
  (impl $trait:ident::$func:ident for $type:ty as $impl:ident) => {
    impl ::core::ops::$trait for $type {
      type Output = Self;

      #[inline]
      fn $func(self, other: $type) -> Self::Output {
        Self::$impl(self, other)
      }
    }

    implement_binop!(impl ref $trait::$func for $type);
  };
  // Implementation - Binary Assign Op
  (impl $trait:ident::$func:ident for $type:ty as $base:ident::$impl:ident) => {
    impl ::core::ops::$trait for $type {
      #[inline]
      fn $func(&mut self, other: $type) {
        *self = ::core::ops::$base::$impl(*self, other);
      }
    }

    implement_binop!(impl ref assign $trait::$func for $type);
  };
  // Implementation - Binary Op Reference
  (impl ref $trait:ident::$func:ident for $type:ty) => {
    impl<'a> ::core::ops::$trait<$type> for &'a $type {
      type Output = <$type as ::core::ops::$trait>::Output;

      #[inline]
      fn $func(self, other: $type) -> Self::Output {
        ::core::ops::$trait::$func(*self, other)
      }
    }

    impl ::core::ops::$trait<&'_ $type> for $type {
      type Output = <$type as ::core::ops::$trait>::Output;

      #[inline]
      fn $func(self, other: &'_ $type) -> Self::Output {
        ::core::ops::$trait::$func(self, *other)
      }
    }

    impl ::core::ops::$trait<&'_ $type> for &'_ $type {
      type Output = <$type as ::core::ops::$trait>::Output;

      #[inline]
      fn $func(self, other: &'_ $type) -> Self::Output {
        ::core::ops::$trait::$func(*self, *other)
      }
    }
  };
  // Implementation - Binary Assign Op Reference
  (impl ref assign $trait:ident::$func:ident for $type:ty) => {
    impl ::core::ops::$trait<&'_ $type> for $type {
      #[inline]
      fn $func(&mut self, other: &'_ $type) {
        ::core::ops::$trait::$func(self, *other);
      }
    }
  };
}

macro_rules! define_hash {
  ($name:ident($bytes:literal)) => {
    define_hash!(
      $name($bytes),
      concat!("A fixed-size hash of ", $bytes, " bytes.")
    );
  };
  ($name:ident($bytes:literal), $doc:expr) => {
    #[doc = $doc]
    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct $name([u8; $bytes]);

    impl $name {
      #[doc = concat!("A null `", stringify!($name), "` value (filled with zero bytes).")]
      pub const NULL: Self = Self::null();

      #[doc = concat!("Creates a new `", stringify!($name), "` filled with zero bytes.")]
      #[inline]
      pub const fn null() -> Self {
        Self::from_array([0; $bytes])
      }

      #[doc = concat!("Creates a new `", stringify!($name), "` from a fixed-size byte array.")]
      #[inline]
      pub const fn from_array(value: [u8; $bytes]) -> Self {
        Self(value)
      }

      /// Converts `self` into a fixed-size byte array.
      #[inline]
      pub const fn into_array(self) -> [u8; $bytes] {
        self.0
      }

      /// Returns a raw pointer to the underlying byte buffer.
      #[inline]
      pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
      }

      /// Returns an unsafe mutable pointer to the underlying byte buffer.
      #[inline]
      pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
      }

      /// Returns a reference to the underlying byte slice.
      #[inline]
      pub const fn as_slice(&self) -> &$crate::eth::Slice {
        $crate::eth::Slice::new(&self.0)
      }

      /// Returns a mutable reference to the underlying byte slice.
      #[inline]
      pub const fn as_mut_slice(&mut self) -> &mut $crate::eth::Slice {
        $crate::eth::Slice::new_mut(&mut self.0)
      }

      /// Returns a reference to the underlying byte array.
      #[inline]
      pub const fn as_array(&self) -> &[u8; $bytes] {
        &self.0
      }

      /// Returns a mutable reference to the underlying byte array.
      #[inline]
      pub const fn as_mut_array(&mut self) -> &mut [u8; $bytes] {
        &mut self.0
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::default::Default
    // -------------------------------------------------------------------------

    impl ::core::default::Default for $name {
      #[inline]
      fn default() -> Self {
        $name::NULL
      }
    }

    impl ::core::default::Default for &$name {
      #[inline]
      fn default() -> Self {
        &$name::NULL
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::fmt::{Debug/Display/LowerHex/UpperHex}
    // -------------------------------------------------------------------------

    impl ::core::fmt::Debug for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::LowerHex::fmt(self, f)
      }
    }

    impl ::core::fmt::Display for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::LowerHex::fmt(self, f)
      }
    }

    impl ::core::fmt::LowerHex for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::ueb3_utils::base16::FormatHex::fmt_lower(self, f)
      }
    }

    impl ::core::fmt::UpperHex for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::ueb3_utils::base16::FormatHex::fmt_upper(self, f)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::ops::Deref/DerefMut
    // -------------------------------------------------------------------------

    impl ::core::ops::Deref for $name {
      type Target = [u8; $bytes];

      #[inline]
      fn deref(&self) -> &Self::Target {
        self.as_array()
      }
    }

    impl ::core::ops::DerefMut for $name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_array()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::AsRef
    // -------------------------------------------------------------------------

    impl ::core::convert::AsRef<Self> for $name {
      #[inline]
      fn as_ref(&self) -> &Self {
        self
      }
    }

    impl ::core::convert::AsRef<$crate::eth::Slice> for $name {
      #[inline]
      fn as_ref(&self) -> &$crate::eth::Slice {
        self.as_slice()
      }
    }

    impl ::core::convert::AsRef<[u8]> for $name {
      #[inline]
      fn as_ref(&self) -> &[u8] {
        self.as_slice()
      }
    }

    impl ::core::convert::AsRef<[u8; $bytes]> for $name {
      #[inline]
      fn as_ref(&self) -> &[u8; $bytes] {
        self.as_array()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::AsMut
    // -------------------------------------------------------------------------

    impl ::core::convert::AsMut<Self> for $name {
      #[inline]
      fn as_mut(&mut self) -> &mut Self {
        self
      }
    }

    impl ::core::convert::AsMut<$crate::eth::Slice> for $name {
      #[inline]
      fn as_mut(&mut self) -> &mut $crate::eth::Slice {
        self.as_mut_slice()
      }
    }

    impl ::core::convert::AsMut<[u8]> for $name {
      #[inline]
      fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
      }
    }

    impl ::core::convert::AsMut<[u8; $bytes]> for $name {
      #[inline]
      fn as_mut(&mut self) -> &mut [u8; $bytes] {
        self.as_mut_array()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::borrow::Borrow
    // -------------------------------------------------------------------------

    impl ::core::borrow::Borrow<$crate::eth::Slice> for $name {
      #[inline]
      fn borrow(&self) -> &$crate::eth::Slice {
        self.as_slice()
      }
    }

    impl ::core::borrow::Borrow<[u8]> for $name {
      #[inline]
      fn borrow(&self) -> &[u8] {
        self.as_slice()
      }
    }

    impl ::core::borrow::Borrow<[u8; $bytes]> for $name {
      #[inline]
      fn borrow(&self) -> &[u8; $bytes] {
        self.as_array()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::borrow::BorrowMut
    // -------------------------------------------------------------------------

    impl ::core::borrow::BorrowMut<$crate::eth::Slice> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut $crate::eth::Slice {
        self.as_mut_slice()
      }
    }

    impl ::core::borrow::BorrowMut<[u8]> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
      }
    }

    impl ::core::borrow::BorrowMut<[u8; $bytes]> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut [u8; $bytes] {
        self.as_mut_array()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::From (array -> Self)
    // -------------------------------------------------------------------------

    impl ::core::convert::From<[u8; $bytes]> for $name {
      #[inline]
      fn from(other: [u8; $bytes]) -> Self {
        Self::from_array(other)
      }
    }

    impl ::core::convert::From<&[u8; $bytes]> for $name {
      #[inline]
      fn from(other: &[u8; $bytes]) -> Self {
        Self::from_array(*other)
      }
    }

    impl ::core::convert::From<&mut [u8; $bytes]> for $name {
      #[inline]
      fn from(other: &mut [u8; $bytes]) -> Self {
        Self::from_array(*other)
      }
    }

    impl ::core::convert::From<&[u8; $bytes]> for &$name {
      #[inline]
      fn from(other: &[u8; $bytes]) -> Self {
        // SAFETY: `$name` is the same representation as a fixed-size byte array.
        unsafe { ::core::mem::transmute(other) }
      }
    }

    impl ::core::convert::From<&mut [u8; $bytes]> for &$name {
      #[inline]
      fn from(other: &mut [u8; $bytes]) -> Self {
        // SAFETY: `$name` is the same representation as a fixed-size byte array.
        unsafe { ::core::mem::transmute(other) }
      }
    }

    impl ::core::convert::From<&mut [u8; $bytes]> for &mut $name {
      #[inline]
      fn from(other: &mut [u8; $bytes]) -> Self {
        // SAFETY: `$name` is the same representation as a fixed-size byte array.
        unsafe { ::core::mem::transmute(other) }
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::From (Self -> array)
    // -------------------------------------------------------------------------

    impl ::core::convert::From<$name> for [u8; $bytes] {
      #[inline]
      fn from(other: $name) -> Self {
        other.into_array()
      }
    }

    impl ::core::convert::From<&$name> for [u8; $bytes] {
      #[inline]
      fn from(other: &$name) -> Self {
        other.into_array()
      }
    }

    impl ::core::convert::From<&mut $name> for [u8; $bytes] {
      #[inline]
      fn from(other: &mut $name) -> Self {
        other.into_array()
      }
    }

    impl<'a> ::core::convert::From<&'a $name> for &'a [u8; $bytes] {
      #[inline]
      fn from(other: &'a $name) -> Self {
        other.as_array()
      }
    }

    impl<'a> ::core::convert::From<&'a mut $name> for &'a [u8; $bytes] {
      #[inline]
      fn from(other: &'a mut $name) -> Self {
        other.as_array()
      }
    }

    impl<'a> ::core::convert::From<&'a mut $name> for &'a mut [u8; $bytes] {
      #[inline]
      fn from(other: &'a mut $name) -> Self {
        other.as_mut_array()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::TryFrom (Slice -> Self)
    // -------------------------------------------------------------------------

    impl ::core::convert::TryFrom<&$crate::eth::Slice> for $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &$crate::eth::Slice) -> Result<Self, Self::Error> {
        <[u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&mut $crate::eth::Slice> for $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut $crate::eth::Slice) -> Result<Self, Self::Error> {
        <[u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&$crate::eth::Slice> for &$name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &$crate::eth::Slice) -> Result<Self, Self::Error> {
        <&[u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&mut $crate::eth::Slice> for &$name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut $crate::eth::Slice) -> Result<Self, Self::Error> {
        <&mut [u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&mut $crate::eth::Slice> for &mut $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut $crate::eth::Slice) -> Result<Self, Self::Error> {
        <&mut [u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::TryFrom (slice -> Self)
    // -------------------------------------------------------------------------

    impl ::core::convert::TryFrom<&[u8]> for $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &[u8]) -> Result<Self, Self::Error> {
        <[u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&mut [u8]> for $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut [u8]) -> Result<Self, Self::Error> {
        <[u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&[u8]> for &$name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &[u8]) -> Result<Self, Self::Error> {
        <&[u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&mut [u8]> for &$name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut [u8]) -> Result<Self, Self::Error> {
        <&mut [u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    impl ::core::convert::TryFrom<&mut [u8]> for &mut $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut [u8]) -> Result<Self, Self::Error> {
        <&mut [u8; $bytes]>::try_from(other).map(Into::into)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::ops::Index/IndexMut
    // -------------------------------------------------------------------------

    impl<I> ::core::ops::Index<I> for $name
    where
      [u8]: ::core::ops::Index<I>,
    {
      type Output = <[u8] as ::core::ops::Index<I>>::Output;

      #[inline]
      fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
      }
    }

    impl<I> ::core::ops::IndexMut<I> for $name
    where
      [u8]: ::core::ops::IndexMut<I>,
    {
      #[inline]
      fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.0.index_mut(index)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::iter::IntoIterator
    // -------------------------------------------------------------------------

    impl<'a> ::core::iter::IntoIterator for &'a $name {
      type Item = &'a u8;
      type IntoIter = ::core::slice::Iter<'a, u8>;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.iter()
      }
    }

    impl<'a> ::core::iter::IntoIterator for &'a mut $name {
      type Item = &'a mut u8;
      type IntoIter = ::core::slice::IterMut<'a, u8>;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
      }
    }

    impl ::core::iter::IntoIterator for $name {
      type Item = u8;
      type IntoIter = ::core::array::IntoIter<u8, $bytes>;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
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
    // Implementations - Bin Ops
    // -------------------------------------------------------------------------

    impl $name {
      #[inline]
      const fn const_band(self, other: Self) -> Self {
        let mut index: usize = 0;
        let mut value: [u8; $bytes] = [0; $bytes];

        let lhs: [u8; $bytes] = self.into_array();
        let rhs: [u8; $bytes] = other.into_array();

        while index < $bytes {
          value[index] = lhs[index] & rhs[index];
          index += 1;
        }

        Self(value)
      }

      #[inline]
      const fn const_bor(self, other: Self) -> Self {
        let mut index: usize = 0;
        let mut value: [u8; $bytes] = [0; $bytes];

        let lhs: [u8; $bytes] = self.into_array();
        let rhs: [u8; $bytes] = other.into_array();

        while index < $bytes {
          value[index] = lhs[index] | rhs[index];
          index += 1;
        }

        Self(value)
      }

      #[inline]
      const fn const_bxor(self, other: Self) -> Self {
        let mut index: usize = 0;
        let mut value: [u8; $bytes] = [0; $bytes];

        let lhs: [u8; $bytes] = self.into_array();
        let rhs: [u8; $bytes] = other.into_array();

        while index < $bytes {
          value[index] = lhs[index] ^ rhs[index];
          index += 1;
        }

        Self(value)
      }
    }

    implement_binop!(impl BitAnd::bitand for $name as const_band);
    implement_binop!(impl BitOr::bitor   for $name as const_bor);
    implement_binop!(impl BitXor::bitxor for $name as const_bxor);

    implement_binop!(impl BitAndAssign::bitand_assign for $name as BitAnd::bitand);
    implement_binop!(impl BitOrAssign::bitor_assign   for $name as BitOr::bitor);
    implement_binop!(impl BitXorAssign::bitxor_assign for $name as BitXor::bitxor);

    // -------------------------------------------------------------------------
    // Implementations - Base16 Encoding
    // -------------------------------------------------------------------------

    impl ::ueb3_utils::base16::FormatHex for $name {
      type Buffer = [u8; $bytes * 2];

      #[inline]
      fn buffer(_: usize) -> Self::Buffer {
        [b'0'; $bytes * 2]
      }
    }

    impl ::ueb3_utils::base16::FromHex for $name {
      type Error = ::ueb3_utils::base16::FromHexError;

      #[inline]
      fn from_slice(input: &[u8]) -> Result<Self, Self::Error> {
        ::ueb3_utils::base16::FromHex::from_slice(input).map(Self::from_array)
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
        struct __Visitor;

        impl ::serde::de::Visitor<'_> for __Visitor {
          type Value = $name;

          fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_str("a hex-encoded string or bytes")
          }

          fn visit_bytes<E>(self, data: &[u8]) -> Result<Self::Value, E>
          where
            E: ::serde::de::Error,
          {
            $name::try_from(data).map_err(E::custom)
          }

          fn visit_str<E>(self, data: &str) -> Result<Self::Value, E>
          where
            E: ::serde::de::Error,
          {
            ::core::str::FromStr::from_str(data).map_err(E::custom)
          }
        }

        if deserializer.is_human_readable() {
          deserializer.deserialize_str(__Visitor)
        } else {
          deserializer.deserialize_bytes(__Visitor)
        }
      }
    }

    impl ::serde::Serialize for $name {
      #[inline]
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        if serializer.is_human_readable() {
          serializer.collect_str(self)
        } else {
          serializer.serialize_bytes(self.as_slice())
        }
      }
    }
  };
}
