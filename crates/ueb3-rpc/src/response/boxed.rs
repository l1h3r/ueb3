use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_json::value::RawValue;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::Decode;
use ueb3_utils::traits::DecodeError;

use crate::request::RequestId;
use crate::response::Response;
use crate::response::ResponseFailure;
use crate::response::ResponseSuccess;

// -----------------------------------------------------------------------------
// Response Object (boxed)
// -----------------------------------------------------------------------------

/// A lifetime-erased JSON-RPC 2.0 response object.
#[derive(Clone)]
pub enum BoxResponse {
  /// A success response.
  Success(ResponseSuccess<Box<RawValue>>),
  /// An error response.
  Failure(ResponseFailure<Box<RawValue>>),
}

impl BoxResponse {
  /// Returns the corresponding request id.
  #[inline]
  pub const fn request(&self) -> RequestId {
    match self {
      Self::Success(inner) => inner.request(),
      Self::Failure(inner) => inner.request(),
    }
  }

  /// Returns `true` if `self` is a success response.
  #[inline]
  pub const fn is_success(&self) -> bool {
    matches!(self, Self::Success(_))
  }

  /// Returns `true` if `self` is an error response.
  #[inline]
  pub const fn is_failure(&self) -> bool {
    matches!(self, Self::Failure(_))
  }

  /// Deserializes the success response result as `U`.
  #[inline]
  pub fn decode_success<'de, T>(&'de self) -> Result<Option<T>, DecodeError>
  where
    T: Decode<'de>,
  {
    match self {
      Self::Success(inner) => inner.decode().map(Some),
      Self::Failure(_) => Ok(None),
    }
  }

  /// Deserializes the server-defined error data as `U`.
  #[inline]
  pub fn decode_failure<'de, T>(&'de self) -> Result<Option<T>, DecodeError>
  where
    T: Decode<'de>,
  {
    match self {
      Self::Success(_) => Ok(None),
      Self::Failure(inner) => inner.decode(),
    }
  }
}

impl Debug for BoxResponse {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Success(inner) => Debug::fmt(inner, f),
      Self::Failure(inner) => Debug::fmt(inner, f),
    }
  }
}

impl From<ResponseSuccess<Box<RawValue>>> for BoxResponse {
  #[inline]
  fn from(other: ResponseSuccess<Box<RawValue>>) -> Self {
    Self::Success(other)
  }
}

impl From<ResponseFailure<Box<RawValue>>> for BoxResponse {
  #[inline]
  fn from(other: ResponseFailure<Box<RawValue>>) -> Self {
    Self::Failure(other)
  }
}

impl<'de> Deserialize<'de> for BoxResponse {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    Deserialize::deserialize(deserializer).map(Response::into_boxed)
  }
}

impl Serialize for BoxResponse {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::Success(inner) => inner.serialize(serializer),
      Self::Failure(inner) => inner.serialize(serializer),
    }
  }
}
