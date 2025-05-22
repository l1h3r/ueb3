use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct as _;
use serde_json::value::RawValue;
use ueb3_serde::flatten::SerializeFlatten as _;
use ueb3_utils::traits::DecodeError;
use ueb3_utils::traits::DecodeOwned;
use ueb3_utils::traits::Encode;
use ueb3_utils::traits::EncodeError;
use ueb3_utils::traits::SizeOf as _;

use crate::request::BoxRequest;
use crate::request::RequestHeader;
use crate::request::RequestId;

// -----------------------------------------------------------------------------
// Request Object
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 request object (method/id/params).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Request<T> {
  pub(super) header: RequestHeader,
  pub(super) params: T,
}

impl<T> Request<T> {
  /// Creates a new `Request`.
  #[inline]
  pub const fn new(method: &'static str, id: RequestId, params: T) -> Self {
    Self::with_header(RequestHeader::new(method, id), params)
  }

  /// Creates a new `Request` with the given `header` and `params`.
  #[inline]
  pub const fn with_header(header: RequestHeader, params: T) -> Self {
    Self { header, params }
  }

  /// Returns a reference to the request header.
  #[inline]
  pub const fn header(&self) -> &RequestHeader {
    &self.header
  }

  /// Returns a mutable reference to the request header.
  #[inline]
  pub const fn header_mut(&mut self) -> &mut RequestHeader {
    &mut self.header
  }

  /// Returns a reference to the request params.
  #[inline]
  pub const fn params(&self) -> &T {
    &self.params
  }

  /// Returns a mutable reference to the request params.
  #[inline]
  pub const fn params_mut(&mut self) -> &mut T {
    &mut self.params
  }

  /// Converts `self` into a new `Request` with the params set to `f(T)`.
  #[inline]
  pub fn map<F, U>(self, f: F) -> Request<U>
  where
    F: Fn(T) -> U,
  {
    Request::with_header(self.header, f(self.params))
  }

  #[inline]
  pub(super) fn encode_params(&self) -> Result<Option<Box<RawValue>>, EncodeError>
  where
    T: Encode,
  {
    if T::IS_ZST {
      Ok(None)
    } else {
      self.params.encode_raw_value().map(Some)
    }
  }
}

impl<T> TryFrom<BoxRequest> for Request<T>
where
  T: DecodeOwned,
{
  type Error = DecodeError;

  #[inline]
  fn try_from(other: BoxRequest) -> Result<Self, Self::Error> {
    let params: T = T::decode_raw_value(other.params())?;
    let object: Self = Self::with_header(other.header, params);

    Ok(object)
  }
}

impl<T> Serialize for Request<T>
where
  T: Serialize,
{
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let fields: usize = self.header.fields() + usize::from(!T::IS_ZST);
    let mut state: S::SerializeStruct = serializer.serialize_struct("Request", fields)?;

    self.header.serialize_fields(&mut state)?;

    if T::IS_ZST {
      state.skip_field("params")?;
    } else {
      state.serialize_field("params", &self.params)?;
    }

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

  use crate::request::Request;
  use crate::request::RequestId;

  type Params = (&'static str, &'static str, &'static str);

  #[test]
  fn test_serde() {
    let params: Params = ("0x0", "0x00000000", "latest");

    let a: Request<()> = Request::new("rpc_modules", RequestId::new(1), ());
    let b: Request<Params> = Request::new("rpc_modules", RequestId::new(2), params);

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "Request",
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
      Token::Struct {
        name: "Request",
        len: 4,
      },
      Token::Str("jsonrpc"),
      Token::Str("2.0"),
      Token::Str("method"),
      Token::Str("rpc_modules"),
      Token::Str("id"),
      Token::U64(2),
      Token::Str("params"),
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
