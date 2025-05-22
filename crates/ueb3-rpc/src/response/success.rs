use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;
use serde_json::value::RawValue;
use std::borrow::Borrow;
use ueb3_utils::traits::Decode;
use ueb3_utils::traits::DecodeError;

use crate::request::RequestId;
use crate::version::VERSION;

// -----------------------------------------------------------------------------
// Response (success)
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 success response.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ResponseSuccess<T> {
  pub(super) request: RequestId,
  pub(super) content: T,
}

impl<T> ResponseSuccess<T> {
  /// Creates a new `ResponseSuccess`.
  #[inline]
  pub const fn new(request: RequestId, content: T) -> Self {
    Self { request, content }
  }

  /// Returns the corresponding request id.
  #[inline]
  pub const fn request(&self) -> RequestId {
    self.request
  }

  /// Returns a reference to the response result.
  #[inline]
  pub const fn content(&self) -> &T {
    &self.content
  }

  /// Returns a mutable reference to the response result.
  #[inline]
  pub const fn content_mut(&mut self) -> &mut T {
    &mut self.content
  }

  /// Converts `self` into the response result.
  #[inline]
  pub fn into_content(self) -> T {
    self.content
  }

  /// Converts `self` into a new `ResponseFailure`
  /// with the response result set to `f(T)`.
  #[inline]
  pub fn map<F, U>(self, f: F) -> ResponseSuccess<U>
  where
    F: Fn(T) -> U,
  {
    ResponseSuccess {
      request: self.request,
      content: f(self.content),
    }
  }

  /// Deserializes the response result as `U`.
  #[inline]
  pub fn decode<'de, U>(&'de self) -> Result<U, DecodeError>
  where
    T: Borrow<RawValue>,
    U: Decode<'de>,
  {
    U::decode_raw_value(self.content.borrow())
  }
}

impl ResponseSuccess<&RawValue> {
  /// Converts `self` into an equivalent type with a boxed result.
  #[inline]
  pub fn into_boxed(self) -> ResponseSuccess<Box<RawValue>> {
    ResponseSuccess {
      request: self.request,
      content: self.content.to_owned(),
    }
  }
}

impl<T> Serialize for ResponseSuccess<T>
where
  T: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state: S::SerializeStruct = serializer.serialize_struct("ResponseSuccess", 3)?;
    state.serialize_field("jsonrpc", VERSION)?;
    state.serialize_field("id", &self.request)?;
    state.serialize_field("result", &self.content)?;
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
  use crate::response::ResponseSuccess;

  type Params = (&'static str, &'static str, &'static str);

  #[test]
  fn test_serde() {
    let params: Params = ("0x0", "0x00000000", "latest");

    let a: ResponseSuccess<()> = ResponseSuccess::new(RequestId::new(1), ());
    let b: ResponseSuccess<Params> = ResponseSuccess::new(RequestId::new(1), params);

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "ResponseSuccess",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
      Token::Str("id"),
      Token::U64(1),
      Token::Str("result"),
      Token::Unit,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "ResponseSuccess",
        len: 3,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
      Token::Str("id"),
      Token::U64(1),
      Token::Str("result"),
      Token::Tuple { len: 3 },
      Token::Str("0x0"),
      Token::Str("0x00000000"),
      Token::Str("latest"),
      Token::TupleEnd,
      Token::StructEnd,
    ];

    assert_ser_tokens(&a, tokens_a);
    assert_ser_tokens(&b, tokens_b);
  }
}
