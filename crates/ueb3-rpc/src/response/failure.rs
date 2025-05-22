use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;
use serde_json::value::RawValue;
use std::borrow::Borrow;
use ueb3_utils::traits::Decode;
use ueb3_utils::traits::DecodeError;

use crate::request::RequestId;
use crate::response::ResponseError;
use crate::version::VERSION;

// -----------------------------------------------------------------------------
// Response (failure)
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 error response.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ResponseFailure<T> {
  pub(super) request: RequestId,
  pub(super) content: ResponseError<T>,
}

impl<T> ResponseFailure<T> {
  /// Creates a new `ResponseFailure`.
  #[inline]
  pub const fn new(request: RequestId, content: ResponseError<T>) -> Self {
    Self { request, content }
  }

  /// Returns the corresponding request id.
  #[inline]
  pub const fn request(&self) -> RequestId {
    self.request
  }

  /// Returns a reference to the response error object.
  #[inline]
  pub const fn content(&self) -> &ResponseError<T> {
    &self.content
  }

  /// Returns a mutable reference to the response error object.
  #[inline]
  pub const fn content_mut(&mut self) -> &mut ResponseError<T> {
    &mut self.content
  }

  /// Converts `self` into the response error object.
  #[inline]
  pub fn into_content(self) -> ResponseError<T> {
    self.content
  }

  /// Converts `self` into a new `ResponseFailure`
  /// with the response error content set to `f(T)`.
  #[inline]
  pub fn map<F, U>(self, f: F) -> ResponseFailure<U>
  where
    F: Fn(T) -> U,
  {
    ResponseFailure {
      request: self.request,
      content: self.content.map(f),
    }
  }

  /// Deserializes the server-defined error data as `U`.
  #[inline]
  pub fn decode<'de, U>(&'de self) -> Result<Option<U>, DecodeError>
  where
    T: Borrow<RawValue>,
    U: Decode<'de>,
  {
    self.content.decode()
  }
}

impl ResponseFailure<&RawValue> {
  /// Converts `self` into an equivalent type with boxed error data.
  #[inline]
  pub fn into_boxed(self) -> ResponseFailure<Box<RawValue>> {
    ResponseFailure {
      request: self.request,
      content: self.content.into_boxed(),
    }
  }
}

impl<T> Serialize for ResponseFailure<T>
where
  T: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state: S::SerializeStruct = serializer.serialize_struct("ResponseFailure", 3)?;
    state.serialize_field("jsonrpc", VERSION)?;
    state.serialize_field("id", &self.request)?;
    state.serialize_field("error", &self.content)?;
    state.end()
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Token;
  use serde_test::assert_ser_tokens;

  use crate::request::RequestId;
  use crate::response::ResponseError;
  use crate::response::ResponseFailure;

  type Params = (&'static str, &'static str, &'static str);

  #[test]
  fn test_serde() {
    let params: Params = ("0x0", "0x00000000", "latest");

    let error_a: ResponseError<()> = ResponseError::new_const(-666, "foo", None);
    let error_b: ResponseError<Params> = ResponseError::new_const(-666, "foo", Some(params));

    let a: ResponseFailure<()> = ResponseFailure::new(RequestId::new(1), error_a);
    let b: ResponseFailure<Params> = ResponseFailure::new(RequestId::new(1), error_b);

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "ResponseFailure",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
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

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "ResponseFailure",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
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
      Token::Tuple { len: 3 },
      Token::Str("0x0"),
      Token::Str("0x00000000"),
      Token::Str("latest"),
      Token::TupleEnd,
      Token::StructEnd,
      Token::StructEnd,
    ];

    assert_ser_tokens(&a, tokens_a);
    assert_ser_tokens(&b, tokens_b);
  }
}
