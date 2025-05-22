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

use crate::request::BoxRequest;
use crate::request::RequestHeader;

// -----------------------------------------------------------------------------
// Request Batch
// -----------------------------------------------------------------------------

/// A single value or batch of several JSON-RPC 2.0 request objects.
#[derive(Clone)]
pub enum RequestBatch {
  /// A batch of several requests.
  Batch(Vec<BoxRequest>),
  /// A single request object.
  Value(BoxRequest),
}

impl RequestBatch {
  /// Creates a new, empty `RequestBatch` with at least the specified capacity.
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    Self::Batch(Vec::with_capacity(capacity))
  }

  /// Returns `true` if `self` is a batch of multiple request objects.
  #[inline]
  pub const fn is_batch(&self) -> bool {
    matches!(self, Self::Batch(_))
  }

  /// Returns `true` if `self` is a single request object.
  #[inline]
  pub const fn is_value(&self) -> bool {
    matches!(self, Self::Value(_))
  }

  /// Returns a reference to the underlying slice of request objects.
  ///
  /// This may return `None` if the batch is a value variant.
  #[inline]
  pub const fn as_batch(&self) -> Option<&[BoxRequest]> {
    match self {
      Self::Batch(inner) => Some(inner.as_slice()),
      Self::Value(_) => None,
    }
  }

  /// Returns a reference to the underlying request object.
  ///
  /// This may return `None` if the batch is a batch variant.
  #[inline]
  pub const fn as_value(&self) -> Option<&BoxRequest> {
    match self {
      Self::Batch(_) => None,
      Self::Value(inner) => Some(inner),
    }
  }

  /// Returns `self` as a slice of request objects.
  #[inline]
  pub const fn as_slice(&self) -> &[BoxRequest] {
    match self {
      Self::Batch(inner) => inner.as_slice(),
      Self::Value(inner) => slice::from_ref(inner),
    }
  }

  /// Returns a mutable reference to the underlying slice of requests.
  #[inline]
  pub const fn as_mut_slice(&mut self) -> &mut [BoxRequest] {
    match self {
      Self::Batch(inner) => inner.as_mut_slice(),
      Self::Value(inner) => slice::from_mut(inner),
    }
  }

  /// Converts `self` into a vector of request objects.
  #[inline]
  pub fn into_vec(self) -> Vec<BoxRequest> {
    match self {
      Self::Batch(inner) => inner,
      Self::Value(inner) => vec![inner],
    }
  }

  /// Returns an iterator over the request methods.
  #[inline]
  pub fn methods(&self) -> impl Iterator<Item = &'static str> {
    self
      .as_slice()
      .iter()
      .map(BoxRequest::header)
      .map(RequestHeader::method)
  }
}

impl Default for RequestBatch {
  #[inline]
  fn default() -> Self {
    Self::Batch(Vec::new())
  }
}

impl Debug for RequestBatch {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Batch(inner) => Debug::fmt(inner, f),
      Self::Value(inner) => Debug::fmt(inner, f),
    }
  }
}

impl Deref for RequestBatch {
  type Target = [BoxRequest];

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl DerefMut for RequestBatch {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_slice()
  }
}

impl AsRef<[BoxRequest]> for RequestBatch {
  #[inline]
  fn as_ref(&self) -> &[BoxRequest] {
    self.as_slice()
  }
}

impl AsMut<[BoxRequest]> for RequestBatch {
  #[inline]
  fn as_mut(&mut self) -> &mut [BoxRequest] {
    self.as_mut_slice()
  }
}

impl From<BoxRequest> for RequestBatch {
  #[inline]
  fn from(other: BoxRequest) -> Self {
    Self::Value(other)
  }
}

impl<const N: usize> From<[BoxRequest; N]> for RequestBatch {
  #[inline]
  fn from(other: [BoxRequest; N]) -> Self {
    Self::Batch(other.into())
  }
}

impl From<Vec<BoxRequest>> for RequestBatch {
  #[inline]
  fn from(other: Vec<BoxRequest>) -> Self {
    Self::Batch(other)
  }
}

impl From<Box<[BoxRequest]>> for RequestBatch {
  #[inline]
  fn from(other: Box<[BoxRequest]>) -> Self {
    Self::Batch(other.into_vec())
  }
}

impl Extend<BoxRequest> for RequestBatch {
  #[inline]
  fn extend<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = BoxRequest>,
  {
    match self {
      Self::Batch(inner) => {
        inner.extend(iter);
      }
      Self::Value(_) => {
        let iter: I::IntoIter = iter.into_iter();
        let size: usize = iter.size_hint().1.unwrap_or(1);
        let data: Vec<BoxRequest> = Vec::with_capacity(size);
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

impl FromIterator<BoxRequest> for RequestBatch {
  #[inline]
  fn from_iter<I>(iter: I) -> Self
  where
    I: IntoIterator<Item = BoxRequest>,
  {
    Self::Batch(Vec::from_iter(iter))
  }
}

impl IntoIterator for RequestBatch {
  type Item = BoxRequest;
  type IntoIter = vec::IntoIter<BoxRequest>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.into_vec().into_iter()
  }
}

impl<'a> IntoIterator for &'a RequestBatch {
  type Item = &'a BoxRequest;
  type IntoIter = slice::Iter<'a, BoxRequest>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut RequestBatch {
  type Item = &'a mut BoxRequest;
  type IntoIter = slice::IterMut<'a, BoxRequest>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl Serialize for RequestBatch {
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_json::value::RawValue;
  use serde_test::Token;
  use serde_test::assert_ser_tokens;
  use ueb3_utils::traits::Encode as _;

  use crate::request::BoxRequest;
  use crate::request::RequestBatch;
  use crate::request::RequestId;

  type Params = (&'static str, &'static str, &'static str);

  #[test]
  fn test_serde() {
    let params: Params = ("0x0", "0x00000000", "latest");
    let params: Box<RawValue> = params.encode_raw_value().unwrap();

    let data_1: BoxRequest = BoxRequest::new("rpc_modules", RequestId::new(1), None);
    let data_2: BoxRequest = BoxRequest::new("rpc_modules", RequestId::new(2), Some(params));

    let a: RequestBatch = RequestBatch::from(data_1.clone());
    let b: RequestBatch = RequestBatch::from([data_1, data_2]);

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "BoxRequest",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
      Token::Str("method"),
      Token::Str("rpc_modules"),
      Token::Str("id"),
      Token::U64(1),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Seq { len: Some(2) },
      Token::Struct {
        name: "BoxRequest",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
      Token::Str("method"),
      Token::Str("rpc_modules"),
      Token::Str("id"),
      Token::U64(1),
      Token::StructEnd,
      Token::Struct {
        name: "BoxRequest",
        len: 4,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
      Token::Str("method"),
      Token::Str("rpc_modules"),
      Token::Str("id"),
      Token::U64(2),
      Token::Str("params"),
      Token::Struct {
        name: "$serde_json::private::RawValue",
        len: 1,
      },
      Token::Str("$serde_json::private::RawValue"),
      Token::Str(r#"["0x0","0x00000000","latest"]"#),
      Token::StructEnd,
      Token::StructEnd,
      Token::SeqEnd,
    ];

    assert_ser_tokens(&a, tokens_a);
    assert_ser_tokens(&b, tokens_b);
  }
}
