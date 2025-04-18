use serde::Deserialize;
use serde::Serialize;
use serde_json::value::RawValue;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_utils::traits::Decode;
use ueb3_utils::traits::DecodeError;
use ueb3_utils::traits::OptionExt as _;

// -----------------------------------------------------------------------------
// JSON-RPC 2.0 Error Object
// -----------------------------------------------------------------------------

/// A JSON-RPC 2.0 error object.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseError<T> {
  code: i64,
  #[serde(rename = "message")]
  info: Cow<'static, str>,
  data: Option<T>,
}

impl<T> ResponseError<T> {
  /// A pre-defined JSON-RPC 2.0 error: `Invalid JSON was received by the server`.
  pub const PARSE_ERROR: Self = Self::new_const(-32700, "Parse error");

  /// A pre-defined JSON-RPC 2.0 error: `The JSON sent is not a valid Request object`.
  pub const INVALID_REQUEST: Self = Self::new_const(-32600, "Invalid request");

  /// A pre-defined JSON-RPC 2.0 error: `The method does not exist / is not available`.
  pub const METHOD_NOT_FOUND: Self = Self::new_const(-32601, "Method not found");

  /// A pre-defined JSON-RPC 2.0 error: `Invalid method parameter(s)`.
  pub const INVALID_PARAMS: Self = Self::new_const(-32602, "Invalid params");

  /// A pre-defined JSON-RPC 2.0 error: `Internal JSON-RPC error`.
  pub const INTERNAL_ERROR: Self = Self::new_const(-32603, "Internal error");

  /// A pre-defined Ethereum JSON-RPC error: `Generic client error while processing request.`
  pub const SERVER_ERROR: Self = Self::new_const(-32000, "Server error");

  /// A pre-defined Ethereum JSON-RPC error: `Payload does not exist / is not available`.
  pub const UNKNOWN_PAYLOAD: Self = Self::new_const(-38001, "Unknown payload");

  /// A pre-defined Ethereum JSON-RPC error: `Forkchoice state is invalid / inconsistent`.
  pub const INVALID_FORKCHOICE: Self = Self::new_const(-38002, "Invalid forkchoice state");

  /// A pre-defined Ethereum JSON-RPC error: `Payload attributes are invalid / inconsistent`.
  pub const INVALID_PAYLOAD: Self = Self::new_const(-38003, "Invalid payload attributes");

  /// A pre-defined Ethereum JSON-RPC error: `Number of requested entities is too large`.
  pub const REQUEST_TOO_LARGE: Self = Self::new_const(-38004, "Too large request");

  /// A pre-defined Ethereum JSON-RPC error: `Payload belongs to a fork that is not supported`.
  pub const UNSUPPORTED_FORK: Self = Self::new_const(-38005, "Unsupported fork");

  #[inline]
  const fn new_const(code: i64, info: &'static str) -> Self {
    Self {
      code,
      info: Cow::Borrowed(info),
      data: None,
    }
  }

  /// Returns the number which indicates the error type that occurred.
  #[inline]
  pub const fn code(&self) -> i64 {
    self.code
  }

  /// Returns a short description of the error.
  #[inline]
  pub fn info(&self) -> &str {
    self.info.as_ref()
  }

  /// Returns a reference to the server-defined error data.
  #[inline]
  pub const fn data(&self) -> Option<&T> {
    self.data.as_ref()
  }

  /// Returns a mutable reference to the server-defined error data.
  #[inline]
  pub const fn data_mut(&mut self) -> Option<&mut T> {
    self.data.as_mut()
  }

  /// Deserializes the server-defined error data as `U`.
  #[inline]
  pub fn decode<'de, U>(&'de self) -> Result<Option<U>, DecodeError>
  where
    T: Borrow<RawValue>,
    U: Decode<'de>,
  {
    self
      .data()
      .map(T::borrow)
      .map(U::decode_raw_value)
      .transpose()
  }
}

impl ResponseError<&RawValue> {
  /// Converts `self` into an equivalent type with boxed error data.
  #[inline]
  pub fn into_boxed(self) -> ResponseError<Box<RawValue>> {
    ResponseError {
      code: self.code,
      info: self.info,
      data: self.data.map(ToOwned::to_owned),
    }
  }
}

impl<T> Debug for ResponseError<T>
where
  T: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("ResponseError")
      .field("code", &self.code)
      .field("info", &self.info)
      .field("data", &self.data.format())
      .finish()
  }
}

impl<T> Display for ResponseError<T>
where
  T: Display,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let (code, info): (i64, &str) = (self.code, self.info());

    let Some(data) = self.data() else {
      return write!(f, "(code: {code}, info: {info})");
    };

    write!(f, "(code: {code}, info: {info}, data: {data})")
  }
}

impl<T> Error for ResponseError<T> where T: Debug + Display {}
