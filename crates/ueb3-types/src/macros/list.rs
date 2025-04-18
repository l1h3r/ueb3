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

    impl ::core::fmt::Debug for $name {
      #[inline]
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.inner, f)
      }
    }

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

    impl ::core::convert::AsRef<$name> for $name {
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

    impl ::core::convert::AsMut<$name> for $name {
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

    impl ::core::borrow::Borrow<[$data]> for $name {
      #[inline]
      fn borrow(&self) -> &[$data] {
        self.inner.borrow()
      }
    }

    impl ::core::borrow::BorrowMut<[$data]> for $name {
      #[inline]
      fn borrow_mut(&mut self) -> &mut [$data] {
        self.inner.borrow_mut()
      }
    }

    impl ::core::iter::Extend<$data> for $name {
      #[inline]
      fn extend<I>(&mut self, iter: I)
      where
        I: ::core::iter::IntoIterator<Item = $data>,
      {
        self.inner.extend(iter);
      }
    }

    impl ::core::iter::FromIterator<$data> for $name {
      #[inline]
      fn from_iter<I>(iter: I) -> Self
      where
        I: ::core::iter::IntoIterator<Item = $data>,
      {
        Self::from_vec(iter.into_iter().collect())
      }
    }

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
  };
}
