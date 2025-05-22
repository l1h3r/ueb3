macro_rules! define_list {
  (#[copy] $name:ident = [$data:ty]) => {
    define_list! {
      $name = [$data]
    }

    impl<'a> ::core::iter::Extend<&'a $data> for $name {
      #[inline]
      fn extend<I>(&mut self, iter: I)
      where
        I: ::core::iter::IntoIterator<Item = &'a $data>,
      {
        self.inner.extend(iter);
      }
    }
  };
  ($name:ident = [$data:ty]) => {
    #[doc = concat!("A dynamic list of `", stringify!($data), "` objects.")]
    #[derive(Clone, Default, Hash, PartialEq, Eq, ::serde::Deserialize, ::serde::Serialize)]
    #[repr(transparent)]
    #[serde(transparent)]
    pub struct $name {
      inner: Vec<$data>,
    }

    impl $name {
      #[doc = concat!("Creates a new `", stringify!($name), "`.")]
      #[inline]
      pub const fn new() -> Self {
        Self::from_vec(Vec::new())
      }

      #[doc = concat!("Creates a new `", stringify!($name), "` from a vector of values.")]
      #[inline]
      pub const fn from_vec(inner: Vec<$data>) -> Self {
        Self { inner }
      }

      /// Converts `self` into a vector of values.
      #[inline]
      pub fn into_vec(self) -> Vec<$data> {
        self.inner
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::fmt::Debug
    // -------------------------------------------------------------------------

    impl ::core::fmt::Debug for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.inner, f)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::ops::Deref/DerefMut
    // -------------------------------------------------------------------------

    impl ::core::ops::Deref for $name {
      type Target = Vec<$data>;

      #[inline]
      fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
      }
    }

    impl ::core::ops::DerefMut for $name {
      #[inline]
      fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
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

    impl ::core::convert::AsRef<[$data]> for $name {
      #[inline]
      fn as_ref(&self) -> &[$data] {
        self.inner.as_slice()
      }
    }

    impl ::core::convert::AsRef<Vec<$data>> for $name {
      #[inline]
      fn as_ref(&self) -> &Vec<$data> {
        self.inner.as_ref()
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

    impl ::core::convert::AsMut<[$data]> for $name {
      #[inline]
      fn as_mut(&mut self) -> &mut [$data] {
        self.inner.as_mut_slice()
      }
    }

    impl ::core::convert::AsMut<Vec<$data>> for $name {
      #[inline]
      fn as_mut(&mut self) -> &mut Vec<$data> {
        self.inner.as_mut()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::borrow::Borrow
    // -------------------------------------------------------------------------

    impl ::core::borrow::Borrow<[$data]> for $name {
      #[inline]
      fn borrow(&self) -> &[$data] {
        self.inner.borrow()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::borrow::BorrowMut
    // -------------------------------------------------------------------------

    impl ::core::borrow::BorrowMut<[$data]> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut [$data] {
        self.inner.borrow_mut()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::convert::From (<container> -> Self)
    // -------------------------------------------------------------------------

    impl<const N: usize> ::core::convert::From<[$data; N]> for $name {
      #[inline]
      fn from(other: [$data; N]) -> Self {
        Self::from_vec(other.into())
      }
    }

    impl ::core::convert::From<Vec<$data>> for $name {
      #[inline]
      fn from(other: Vec<$data>) -> Self {
        Self::from_vec(other)
      }
    }

    impl ::core::convert::From<Box<[$data]>> for $name {
      #[inline]
      fn from(other: Box<[$data]>) -> Self {
        Self::from_vec(other.into())
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::ops::Index/IndexMut
    // -------------------------------------------------------------------------

    impl<I> ::core::ops::Index<I> for $name
    where
      I: ::core::slice::SliceIndex<[$data]>,
    {
      type Output = <I as ::core::slice::SliceIndex<[$data]>>::Output;

      #[inline]
      fn index(&self, index: I) -> &Self::Output {
        self.inner.index(index)
      }
    }

    impl<I> ::core::ops::IndexMut<I> for $name
    where
      I: ::core::slice::SliceIndex<[$data]>,
    {
      #[inline]
      fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.inner.index_mut(index)
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::iter::IntoIterator
    // -------------------------------------------------------------------------

    impl ::core::iter::IntoIterator for $name {
      type Item = $data;
      type IntoIter = ::std::vec::IntoIter<Self::Item>;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
      }
    }

    impl<'a> ::core::iter::IntoIterator for &'a $name {
      type Item = &'a $data;
      type IntoIter = ::core::slice::Iter<'a, $data>;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
      }
    }

    impl<'a> ::core::iter::IntoIterator for &'a mut $name {
      type Item = &'a mut $data;
      type IntoIter = ::core::slice::IterMut<'a, $data>;

      #[inline]
      fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::iter::FromIterator
    // -------------------------------------------------------------------------

    impl ::core::iter::FromIterator<$data> for $name {
      #[inline]
      fn from_iter<I>(iter: I) -> Self
      where
        I: ::core::iter::IntoIterator<Item = $data>,
      {
        Self::from_vec(iter.into_iter().collect())
      }
    }

    // -------------------------------------------------------------------------
    // Implementations - ::core::iter::Extend
    // -------------------------------------------------------------------------

    impl ::core::iter::Extend<$data> for $name {
      #[inline]
      fn extend<I>(&mut self, iter: I)
      where
        I: ::core::iter::IntoIterator<Item = $data>,
      {
        self.inner.extend(iter);
      }
    }
  };
}
