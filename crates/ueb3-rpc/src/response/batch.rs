use serde::Serialize;
use serde::Serializer;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice;
use std::vec;

use crate::response::BoxResponse;

// -----------------------------------------------------------------------------
// Response Batch
// -----------------------------------------------------------------------------

/// A single value or batch of several JSON-RPC 2.0 response objects.
#[derive(Clone)]
pub enum ResponseBatch {
  /// A batch of several responses.
  Batch(Vec<BoxResponse>),
  /// A single response object.
  Value(BoxResponse),
}

impl ResponseBatch {
  /// Creates a new, empty `ResponseBatch` with at least the specified capacity.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self::Batch(Vec::with_capacity(capacity))
  }

  /// Returns `true` if `self` is a batch of multiple response objects.
  #[inline]
  pub const fn is_batch(&self) -> bool {
    matches!(self, Self::Batch(_))
  }

  /// Returns `true` if `self` is a single response object.
  #[inline]
  pub const fn is_value(&self) -> bool {
    matches!(self, Self::Value(_))
  }

  /// Returns a reference to the underlying slice of response objects.
  ///
  /// This may return `None` if the batch is a value variant.
  #[inline]
  pub fn as_batch(&self) -> Option<&[BoxResponse]> {
    match self {
      Self::Batch(inner) => Some(inner.as_slice()),
      Self::Value(_) => None,
    }
  }

  /// Returns a reference to the underlying response object.
  ///
  /// This may return `None` if the batch is a batch variant.
  #[inline]
  pub const fn as_value(&self) -> Option<&BoxResponse> {
    match self {
      Self::Batch(_) => None,
      Self::Value(inner) => Some(inner),
    }
  }

  /// Returns `self` as a slice of response objects.
  #[inline]
  pub fn as_slice(&self) -> &[BoxResponse] {
    match self {
      Self::Batch(inner) => inner.as_slice(),
      Self::Value(inner) => slice::from_ref(inner),
    }
  }

  /// Returns a mutable reference to the underlying slice of responses.
  #[inline]
  pub fn as_mut_slice(&mut self) -> &mut [BoxResponse] {
    match self {
      Self::Batch(inner) => inner.as_mut_slice(),
      Self::Value(inner) => slice::from_mut(inner),
    }
  }

  /// Converts `self` into a vector of response objects.
  #[inline]
  pub fn into_vec(self) -> Vec<BoxResponse> {
    match self {
      Self::Batch(inner) => inner,
      Self::Value(inner) => vec![inner],
    }
  }
}

impl Default for ResponseBatch {
  #[inline]
  fn default() -> Self {
    Self::Batch(Vec::new())
  }
}

impl Debug for ResponseBatch {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Batch(inner) => Debug::fmt(inner, f),
      Self::Value(inner) => Debug::fmt(inner, f),
    }
  }
}

impl Deref for ResponseBatch {
  type Target = [BoxResponse];

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl DerefMut for ResponseBatch {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_slice()
  }
}

impl AsRef<[BoxResponse]> for ResponseBatch {
  #[inline]
  fn as_ref(&self) -> &[BoxResponse] {
    self.as_slice()
  }
}

impl AsMut<[BoxResponse]> for ResponseBatch {
  #[inline]
  fn as_mut(&mut self) -> &mut [BoxResponse] {
    self.as_mut_slice()
  }
}

impl From<BoxResponse> for ResponseBatch {
  #[inline]
  fn from(other: BoxResponse) -> Self {
    Self::Value(other)
  }
}

impl From<Vec<BoxResponse>> for ResponseBatch {
  #[inline]
  fn from(other: Vec<BoxResponse>) -> Self {
    Self::Batch(other)
  }
}

impl From<Box<[BoxResponse]>> for ResponseBatch {
  #[inline]
  fn from(other: Box<[BoxResponse]>) -> Self {
    Self::Batch(other.into_vec())
  }
}

impl Extend<BoxResponse> for ResponseBatch {
  #[inline]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = BoxResponse>,
  {
    match self {
      Self::Batch(inner) => {
        inner.extend(iter);
      }
      Self::Value(_) => {
        let iter: I::IntoIter = iter.into_iter();
        let size: usize = iter.size_hint().1.unwrap_or(1);
        let data: Vec<BoxResponse> = Vec::with_capacity(size);
        let prev: Self = mem::replace(self, Self::Batch(data));

        let Self::Value(value) = prev else {
          unreachable!()
        };

        let Self::Batch(batch) = self else {
          unreachable!()
        };

        batch.extend(Some(value));
        batch.extend(iter);
      }
    }
  }
}

impl FromIterator<BoxResponse> for ResponseBatch {
  #[inline]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = BoxResponse>,
  {
    Self::Batch(Vec::from_iter(iter))
  }
}

impl IntoIterator for ResponseBatch {
  type Item = BoxResponse;
  type IntoIter = vec::IntoIter<BoxResponse>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.into_vec().into_iter()
  }
}

impl<'a> IntoIterator for &'a ResponseBatch {
  type Item = &'a BoxResponse;
  type IntoIter = slice::Iter<'a, BoxResponse>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut ResponseBatch {
  type Item = &'a mut BoxResponse;
  type IntoIter = slice::IterMut<'a, BoxResponse>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl Serialize for ResponseBatch {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Batch(inner) => inner.serialize(serializer),
      Self::Value(inner) => inner.serialize(serializer),
    }
  }
}
