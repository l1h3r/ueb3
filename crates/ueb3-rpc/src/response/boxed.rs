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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde::Deserialize;
  use serde::Serialize;
  use serde_json::value::RawValue;
  use serde_test::Token;
  use serde_test::assert_tokens;
  use std::borrow::Borrow;
  use ueb3_utils::traits::Encode as _;

  use crate::request::RequestId;
  use crate::response::BoxResponse;
  use crate::response::ResponseError as Error;
  use crate::response::ResponseFailure as Failure;
  use crate::response::ResponseSuccess as Success;

  type Params = (&'static str, &'static str, &'static str);

  #[derive(Clone, Debug, Deserialize, Serialize)]
  #[repr(transparent)]
  #[serde(transparent)]
  struct Wrapper<T>(T);

  impl<T> PartialEq for Wrapper<T>
  where
    T: Borrow<RawValue>,
  {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
      let lhs: &RawValue = self.0.borrow();
      let rhs: &RawValue = other.0.borrow();
      lhs.get() == rhs.get()
    }
  }

  #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
  #[serde(from = "BoxResponse", into = "BoxResponse")]
  enum ResponseEq {
    Success(Success<Wrapper<Box<RawValue>>>),
    Failure(Failure<Wrapper<Box<RawValue>>>),
  }

  impl From<ResponseEq> for BoxResponse {
    #[inline]
    fn from(other: ResponseEq) -> Self {
      match other {
        ResponseEq::Success(inner) => Self::Success(inner.map(|item| item.0)),
        ResponseEq::Failure(inner) => Self::Failure(inner.map(|item| item.0)),
      }
    }
  }

  impl From<BoxResponse> for ResponseEq {
    #[inline]
    fn from(other: BoxResponse) -> Self {
      match other {
        BoxResponse::Success(inner) => Self::Success(inner.map(Wrapper)),
        BoxResponse::Failure(inner) => Self::Failure(inner.map(Wrapper)),
      }
    }
  }

  #[test]
  fn test_serde() {
    let params: Params = ("0x0", "0x00000000", "latest");
    let params: Box<RawValue> = params.encode_raw_value().unwrap();

    let request: RequestId = RequestId::new(1);

    let error_c: Error<Box<RawValue>> = Error::new_const(-666, "foo", None);
    let error_d: Error<Box<RawValue>> = Error::new_const(-666, "foo", Some(params.clone()));

    let content_a: Success<Box<RawValue>> = Success::new(request, RawValue::NULL.to_owned());
    let content_b: Success<Box<RawValue>> = Success::new(request, params);
    let content_c: Failure<Box<RawValue>> = Failure::new(request, error_c);
    let content_d: Failure<Box<RawValue>> = Failure::new(request, error_d);

    let a: BoxResponse = BoxResponse::Success(content_a);
    let b: BoxResponse = BoxResponse::Success(content_b);
    let c: BoxResponse = BoxResponse::Failure(content_c);
    let d: BoxResponse = BoxResponse::Failure(content_d);

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "ResponseSuccess",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::BorrowedStr("2.0"),
      Token::Str("id"),
      Token::U64(1),
      Token::Str("result"),
      Token::Struct {
        name: "$serde_json::private::RawValue",
        len: 1,
      },
      Token::Str("$serde_json::private::RawValue"),
      Token::BorrowedStr("null"),
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "ResponseSuccess",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::BorrowedStr("2.0"),
      Token::Str("id"),
      Token::U64(1),
      Token::Str("result"),
      Token::Struct {
        name: "$serde_json::private::RawValue",
        len: 1,
      },
      Token::Str("$serde_json::private::RawValue"),
      Token::BorrowedStr(r#"["0x0","0x00000000","latest"]"#),
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_c: &[Token] = &[
      Token::Struct {
        name: "ResponseFailure",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::BorrowedStr("2.0"),
      Token::Str("id"),
      Token::U64(1),
      Token::Str("error"),
      Token::Struct {
        name: "ResponseError",
        len: 3,
      },
      Token::Str("code"),
      Token::I64(-666),
      Token::Str("message"),
      Token::Str("foo"),
      Token::Str("data"),
      Token::None,
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_d: &[Token] = &[
      Token::Struct {
        name: "ResponseFailure",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::BorrowedStr("2.0"),
      Token::Str("id"),
      Token::U64(1),
      Token::Str("error"),
      Token::Struct {
        name: "ResponseError",
        len: 3,
      },
      Token::Str("code"),
      Token::I64(-666),
      Token::Str("message"),
      Token::Str("foo"),
      Token::Str("data"),
      Token::Some,
      Token::Struct {
        name: "$serde_json::private::RawValue",
        len: 1,
      },
      Token::Str("$serde_json::private::RawValue"),
      Token::BorrowedStr(r#"["0x0","0x00000000","latest"]"#),
      Token::StructEnd,
      Token::StructEnd,
      Token::StructEnd,
    ];

    assert_tokens(&ResponseEq::from(a), tokens_a);
    assert_tokens(&ResponseEq::from(b), tokens_b);
    assert_tokens(&ResponseEq::from(c), tokens_c);
    assert_tokens(&ResponseEq::from(d), tokens_d);
  }
}
