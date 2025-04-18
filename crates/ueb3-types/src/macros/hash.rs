macro_rules! implement_bitop {
  (
    type: $type:ty;
    expr: $expr:tt;
    op_1: $op_1_trait:ident::$op_1_func:ident;
    op_2: $op_2_trait:ident::$op_2_func:ident;
  ) => {
    impl ::core::ops::$op_2_trait for $type {
      #[inline]
      fn $op_2_func(&mut self, other: Self) {
        let lhs: &mut [u8] = self.as_mut_slice();
        let rhs: &[u8] = other.as_slice();

        for (a, b) in lhs.iter_mut().zip(rhs.iter()) {
          *a $expr b;
        }
      }
    }

    impl ::core::ops::$op_2_trait<&$type> for $type {
      #[inline]
      fn $op_2_func(&mut self, other: &$type) {
        ::core::ops::$op_2_trait::$op_2_func(self, *other);
      }
    }

    impl ::core::ops::$op_1_trait for $type {
      type Output = Self;

      #[inline]
      fn $op_1_func(mut self, other: Self) -> Self::Output {
        self $expr other;
        self
      }
    }

    impl ::core::ops::$op_1_trait<$type> for &$type {
      type Output = <$type as ::core::ops::$op_1_trait<$type>>::Output;

      #[inline]
      fn $op_1_func(self, other: $type) -> Self::Output {
        ::core::ops::$op_1_trait::$op_1_func(*self, other)
      }
    }

    impl ::core::ops::$op_1_trait<&$type> for $type {
      type Output = <$type as ::core::ops::$op_1_trait<$type>>::Output;

      #[inline]
      fn $op_1_func(self, other: &$type) -> Self::Output {
        ::core::ops::$op_1_trait::$op_1_func(self, *other)
      }
    }

    impl ::core::ops::$op_1_trait<&$type> for &$type {
      type Output = <$type as ::core::ops::$op_1_trait<$type>>::Output;

      #[inline]
      fn $op_1_func(self, other: &$type) -> Self::Output {
        ::core::ops::$op_1_trait::$op_1_func(*self, *other)
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
    pub struct $name {
      inner: [u8; $bytes],
    }

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
      pub const fn from_array(inner: [u8; $bytes]) -> Self {
        Self { inner }
      }

      /// Converts `self` into a fixed-size byte array.
      #[inline]
      pub const fn into_array(self) -> [u8; $bytes] {
        self.inner
      }

      /// Returns a raw pointer to the underlying byte buffer.
      #[inline]
      pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
      }

      /// Returns an unsafe mutable pointer to the underlying byte buffer.
      #[inline]
      pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
      }

      /// Returns a reference to the underlying byte slice.
      #[inline]
      pub const fn as_slice(&self) -> &$crate::eth::Slice {
        $crate::eth::Slice::new(&self.inner)
      }

      /// Returns a mutable reference to the underlying byte slice.
      #[inline]
      pub const fn as_mut_slice(&mut self) -> &mut $crate::eth::Slice {
        $crate::eth::Slice::new_mut(&mut self.inner)
      }

      /// Returns a reference to the underlying byte array.
      #[inline]
      pub const fn as_array(&self) -> &[u8; $bytes] {
        &self.inner
      }

      /// Returns a mutable reference to the underlying byte array.
      #[inline]
      pub const fn as_mut_array(&mut self) -> &mut [u8; $bytes] {
        &mut self.inner
      }
    }

    impl ::core::default::Default for $name {
      #[inline]
      fn default() -> Self {
        Self::null()
      }
    }

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
        let mut buffer: [u8; $bytes * 2] = [0; $bytes * 2];

        if let Ok(value) = ::ueb3_utils::base16::encode_into_lower(self, &mut buffer) {
          f.write_str("0x")?;
          f.write_str(value)
        } else {
          Err(::core::fmt::Error)
        }
      }
    }

    impl ::core::fmt::UpperHex for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        let mut buffer: [u8; $bytes * 2] = [0; $bytes * 2];

        if let Ok(value) = ::ueb3_utils::base16::encode_into_upper(self, &mut buffer) {
          f.write_str("0x")?;
          f.write_str(value)
        } else {
          Err(::core::fmt::Error)
        }
      }
    }

    impl ::core::ops::Deref for $name {
      type Target = [u8];

      #[inline]
      fn deref(&self) -> &Self::Target {
        self.as_slice()
      }
    }

    impl ::core::ops::DerefMut for $name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
      }
    }

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

    impl ::core::borrow::Borrow<$crate::eth::Slice> for $name {
      #[inline]
      fn borrow(&self) -> &$crate::eth::Slice {
        self.as_slice()
      }
    }

    impl ::core::borrow::BorrowMut<$crate::eth::Slice> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut $crate::eth::Slice {
        self.as_mut_slice()
      }
    }

    impl ::core::borrow::Borrow<[u8]> for $name {
      #[inline]
      fn borrow(&self) -> &[u8] {
        self.as_slice()
      }
    }

    impl ::core::borrow::BorrowMut<[u8]> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
      }
    }

    impl ::core::borrow::Borrow<[u8; $bytes]> for $name {
      #[inline]
      fn borrow(&self) -> &[u8; $bytes] {
        self.as_array()
      }
    }

    impl ::core::borrow::BorrowMut<[u8; $bytes]> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut [u8; $bytes] {
        self.as_mut_array()
      }
    }

    impl<I> ::core::ops::Index<I> for $name
    where
      I: ::core::slice::SliceIndex<[u8]>,
    {
      type Output = <I as ::core::slice::SliceIndex<[u8]>>::Output;

      #[inline]
      fn index(&self, index: I) -> &Self::Output {
        self.inner.index(index)
      }
    }

    impl<I> ::core::ops::IndexMut<I> for $name
    where
      I: ::core::slice::SliceIndex<[u8]>,
    {
      #[inline]
      fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.inner.index_mut(index)
      }
    }

    implement_bitop! {
      type: $name;
      expr: &=;
      op_1: BitAnd::bitand;
      op_2: BitAndAssign::bitand_assign;
    }

    implement_bitop! {
      type: $name;
      expr: |=;
      op_1: BitOr::bitor;
      op_2: BitOrAssign::bitor_assign;
    }

    implement_bitop! {
      type: $name;
      expr: ^=;
      op_1: BitXor::bitxor;
      op_2: BitXorAssign::bitxor_assign;
    }

    impl From<[u8; $bytes]> for $name {
      #[inline]
      fn from(other: [u8; $bytes]) -> Self {
        Self::from_array(other)
      }
    }

    impl From<&[u8; $bytes]> for $name {
      #[inline]
      fn from(other: &[u8; $bytes]) -> Self {
        Self::from_array(*other)
      }
    }

    impl From<&mut [u8; $bytes]> for $name {
      #[inline]
      fn from(other: &mut [u8; $bytes]) -> Self {
        Self::from_array(*other)
      }
    }

    impl From<$name> for [u8; $bytes] {
      #[inline]
      fn from(other: $name) -> Self {
        other.into_array()
      }
    }

    impl TryFrom<&[u8]> for $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &[u8]) -> Result<Self, Self::Error> {
        other.try_into().map(Self::from_array)
      }
    }

    impl TryFrom<&mut [u8]> for $name {
      type Error = ::core::array::TryFromSliceError;

      #[inline]
      fn try_from(other: &mut [u8]) -> Result<Self, Self::Error> {
        other.try_into().map(Self::from_array)
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
        ::ueb3_utils::base16::FromHex::from_slice(input).map(Self::from_array)
      }
    }

    impl<'de> ::serde::Deserialize<'de> for $name {
      #[inline]
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        ::ueb3_utils::base16::deserialize(deserializer)
      }
    }

    impl ::serde::Serialize for $name {
      #[inline]
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: ::serde::Serializer,
      {
        ::ueb3_utils::base16::serialize::<Self, S, true>(self, serializer)
      }
    }
  };
}
