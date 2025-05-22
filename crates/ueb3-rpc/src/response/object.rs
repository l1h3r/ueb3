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
  use crate::response::Response;
  use crate::response::ResponseError;
  use crate::response::ResponseFailure;
  use crate::response::ResponseSuccess;

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
  #[serde(from = "Response<'a>", into = "Response<'a>")]
  enum ResponseEq<'a> {
    Success(#[serde(borrow)] ResponseSuccess<Wrapper<&'a RawValue>>),
    Failure(#[serde(borrow)] ResponseFailure<Wrapper<&'a RawValue>>),
  }

  impl<'a> From<ResponseEq<'a>> for Response<'a> {
    #[inline]
    fn from(other: ResponseEq<'a>) -> Self {
      match other {
        ResponseEq::Success(inner) => Self::Success(inner.map(|item| item.0)),
        ResponseEq::Failure(inner) => Self::Failure(inner.map(|item| item.0)),
      }
    }
  }

  impl<'a> From<Response<'a>> for ResponseEq<'a> {
    #[inline]
    fn from(other: Response<'a>) -> Self {
      match other {
        Response::Success(inner) => Self::Success(inner.map(Wrapper)),
        Response::Failure(inner) => Self::Failure(inner.map(Wrapper)),
      }
    }
  }

  #[test]
  fn test_serde() {
    let params: Params = ("0x0", "0x00000000", "latest");
    let params: Box<RawValue> = params.encode_raw_value().unwrap();

    let request: RequestId = RequestId::new(1);

    let error_c: ResponseError<&RawValue> = ResponseError::new_const(-666, "foo", None);
    let error_d: ResponseError<&RawValue> = ResponseError::new_const(-666, "foo", Some(&params));

    let content_a: ResponseSuccess<&RawValue> = ResponseSuccess::new(request, RawValue::NULL);
    let content_b: ResponseSuccess<&RawValue> = ResponseSuccess::new(request, &params);
    let content_c: ResponseFailure<&RawValue> = ResponseFailure::new(request, error_c);
    let content_d: ResponseFailure<&RawValue> = ResponseFailure::new(request, error_d);

    let a: Response<'_> = Response::Success(content_a);
    let b: Response<'_> = Response::Success(content_b);
    let c: Response<'_> = Response::Failure(content_c);
    let d: Response<'_> = Response::Failure(content_d);

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
