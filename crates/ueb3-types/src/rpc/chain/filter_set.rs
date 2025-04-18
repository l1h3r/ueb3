use serde::Serialize;
use serde::Serializer;
use std::collections::BTreeSet;
use std::collections::btree_set::IntoIter;
use std::collections::btree_set::Iter;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::iter;
use std::ops::Deref;
use std::ops::DerefMut;

// -----------------------------------------------------------------------------
// Ethereum Filter Property Set
// -----------------------------------------------------------------------------

/// A set of log filter properties.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FilterSet<T> {
  inner: BTreeSet<T>,
}

impl<T> FilterSet<T> {
  /// Creates a new, empty `FilterSet`.
  #[inline]
  pub const fn new() -> Self {
    Self::from_set(BTreeSet::new())
  }

  /// Creates a new `FilterSet` from an ordered set.
  #[inline]
  pub const fn from_set(inner: BTreeSet<T>) -> Self {
    Self { inner }
  }

  #[inline]
  fn value(&self) -> &T
  where
    T: Ord,
  {
    self.inner.first().expect("infallible")
  }
}

impl<T> Default for FilterSet<T> {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Debug for FilterSet<T>
where
  T: Debug + Ord,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self.inner.len() {
      0 => f.write_str("None"),
      1 => Debug::fmt(self.value(), f),
      _ => Debug::fmt(&self.inner, f),
    }
  }
}

impl<T> Deref for FilterSet<T> {
  type Target = BTreeSet<T>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> DerefMut for FilterSet<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl<T> Extend<T> for FilterSet<T>
where
  T: Ord,
{
  #[inline]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = T>,
  {
    self.inner.extend(iter);
  }
}

impl<'a, T> Extend<&'a T> for FilterSet<T>
where
  T: Copy + Ord,
{
  #[inline]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = &'a T>,
  {
    self.inner.extend(iter);
  }
}

impl<T> FromIterator<T> for FilterSet<T>
where
  T: Ord,
{
  #[inline]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = T>,
  {
    Self::from_set(iter.into_iter().collect())
  }
}

impl<T> IntoIterator for FilterSet<T> {
  type Item = T;
  type IntoIter = IntoIter<Self::Item>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.inner.into_iter()
  }
}

impl<'a, T> IntoIterator for &'a FilterSet<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.inner.iter()
  }
}

impl<T> From<T> for FilterSet<T>
where
  T: Ord,
{
  #[inline]
  fn from(other: T) -> Self {
    iter::once(other).collect()
  }
}

impl<T> From<BTreeSet<T>> for FilterSet<T> {
  #[inline]
  fn from(other: BTreeSet<T>) -> Self {
    Self::from_set(other)
  }
}

impl<'a, T> From<&'a [T]> for FilterSet<T>
where
  T: Copy + Ord,
{
  #[inline]
  fn from(other: &'a [T]) -> Self {
    other.iter().copied().collect()
  }
}

impl<T, const N: usize> From<[T; N]> for FilterSet<T>
where
  T: Ord,
{
  #[inline]
  fn from(other: [T; N]) -> Self {
    Self::from_set(other.into())
  }
}

impl<T> From<Vec<T>> for FilterSet<T>
where
  T: Ord,
{
  #[inline]
  fn from(other: Vec<T>) -> Self {
    other.into_iter().collect()
  }
}

impl<T> From<Box<[T]>> for FilterSet<T>
where
  T: Ord,
{
  #[inline]
  fn from(other: Box<[T]>) -> Self {
    other.into_iter().collect()
  }
}

impl<T> Serialize for FilterSet<T>
where
  T: Ord + Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self.inner.len() {
      0 => serializer.serialize_none(),
      1 => self.value().serialize(serializer),
      _ => self.inner.serialize(serializer),
    }
  }
}
