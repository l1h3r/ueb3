use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use serde::de::MapAccess;
use serde::de::Unexpected;
use serde::de::Visitor;
use serde_json::value::RawValue;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::marker::PhantomData;
use ueb3_serde::maybe::Maybe;
use ueb3_utils::traits::Decode;
use ueb3_utils::traits::DecodeError;

use crate::request::RequestId;
use crate::response::BoxResponse;
use crate::response::ResponseError;
use crate::response::ResponseFailure;
use crate::response::ResponseSuccess;
use crate::version::VERSION;

// -----------------------------------------------------------------------------
// Response Object
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 response object.
#[derive(Clone)]
pub enum Response<'de> {
  /// A success response.
  Success(ResponseSuccess<&'de RawValue>),
  /// An error response.
  Failure(ResponseFailure<&'de RawValue>),
}

impl Response<'_> {
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

  /// Converts `self` into a `BoxResponse`.
  #[inline]
  pub fn into_boxed(self) -> BoxResponse {
    match self {
      Self::Success(inner) => BoxResponse::Success(inner.into_boxed()),
      Self::Failure(inner) => BoxResponse::Failure(inner.into_boxed()),
    }
  }
}

impl Debug for Response<'_> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Success(inner) => Debug::fmt(inner, f),
      Self::Failure(inner) => Debug::fmt(inner, f),
    }
  }
}

impl<'de> From<ResponseSuccess<&'de RawValue>> for Response<'de> {
  #[inline]
  fn from(other: ResponseSuccess<&'de RawValue>) -> Self {
    Self::Success(other)
  }
}

impl<'de> From<ResponseFailure<&'de RawValue>> for Response<'de> {
  #[inline]
  fn from(other: ResponseFailure<&'de RawValue>) -> Self {
    Self::Failure(other)
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for Response<'a> {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    static ERR_VARIANT_BOTH: &str = "expected one of `result` or `error` (found both)";
    static ERR_VARIANT_NONE: &str = "expected one of `result` or `error` (found none)";

    struct __Visitor<'a>(PhantomData<&'a ()>);

    impl<'de: 'a, 'a> Visitor<'de> for __Visitor<'a> {
      type Value = Response<'a>;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a JSON-RPC 2.0 response object")
      }

      fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
      where
        A: MapAccess<'de>,
      {
        #[derive(Deserialize)]
        #[serde(variant_identifier)]
        enum Field {
          #[serde(rename = "jsonrpc")]
          Version,
          #[serde(rename = "id")]
          Request,
          #[serde(rename = "result")]
          Result,
          #[serde(rename = "error")]
          Error,
        }

        let mut jsonrpc: Maybe<A, &'de str> = Maybe::NONE;
        let mut request: Maybe<A, RequestId> = Maybe::NONE;
        let mut result: Maybe<A, &'a RawValue> = Maybe::NONE;
        let mut error: Maybe<A, ResponseError<&'a RawValue>> = Maybe::NONE;

        while let Some(field) = access.next_key()? {
          match field {
            Field::Version => jsonrpc.set(&mut access, "jsonrpc")?,
            Field::Request => request.set(&mut access, "id")?,
            Field::Result => result.set(&mut access, "result")?,
            Field::Error => error.set(&mut access, "error")?,
          }
        }

        let jsonrpc: &str = jsonrpc.get_required("jsonrpc")?;

        if jsonrpc != VERSION {
          return Err(A::Error::invalid_value(Unexpected::Str(jsonrpc), &VERSION));
        }

        let request: RequestId = request.get_required("id")?;
        let result: Option<&RawValue> = result.get_optional();
        let error: Option<ResponseError<&RawValue>> = error.get_optional();

        match (result, error) {
          (Some(content), None) => Ok(Response::Success(ResponseSuccess { request, content })),
          (None, Some(content)) => Ok(Response::Failure(ResponseFailure { request, content })),
          (Some(_), Some(_)) => Err(A::Error::custom(ERR_VARIANT_BOTH)),
          (None, None) => Err(A::Error::custom(ERR_VARIANT_NONE)),
        }
      }
    }

    deserializer.deserialize_map(__Visitor(PhantomData))
  }
}

impl Serialize for Response<'_> {
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
