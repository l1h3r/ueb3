use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use ueb3_utils::base16::FromHexError;
use ueb3_utils::traits::DecodeError;
use ueb3_utils::traits::EncodeError;

use crate::response::ResponseError;

/// Alias for a `Result` with the error type `ueb3_rpc::Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

/// The error type returned when a ueb3 JSON-RPC operation fails.
///
/// See [`ErrorKind`] for more info.
pub struct Error {
  // Use `Box` to reduce the actual error size (8 bytes vs 24 bytes).
  repr: Box<ErrorRepr>,
}

impl Error {
  /// Creates a new `Error`.
  #[inline]
  pub fn new<T>(kind: ErrorKind, data: T) -> Self
  where
    T: Into<Box<dyn StdError + Send + Sync>>,
  {
    Self {
      repr: Box::new(ErrorRepr::new(kind, data.into())),
    }
  }

  /// Creates a new `Error` with the type [`Other`][ErrorKind::Other].
  #[inline]
  pub fn other<T>(data: T) -> Self
  where
    T: Into<Box<dyn StdError + Send + Sync>>,
  {
    Self::new(ErrorKind::Other, data)
  }

  /// Returns the error type.
  #[inline]
  pub const fn kind(&self) -> ErrorKind {
    self.repr.kind
  }

  #[inline]
  const fn data(&self) -> &(dyn StdError + Send + Sync + 'static) {
    &*self.repr.data
  }
}

impl Debug for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.data(), f)
  }
}

impl Display for Error {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.data(), f)
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    Some(self.data())
  }
}

impl From<Infallible> for Error {
  #[inline]
  fn from(other: Infallible) -> Self {
    match other {}
  }
}

impl From<IoError> for Error {
  #[inline]
  fn from(other: IoError) -> Self {
    Self::new(ErrorKind::Other, other)
  }
}

impl<T> From<ResponseError<T>> for Error
where
  T: Debug + Display + Send + Sync + 'static,
{
  #[inline]
  fn from(other: ResponseError<T>) -> Self {
    Self::new(ErrorKind::JsonRpcErr, other)
  }
}

impl From<FromHexError> for Error {
  #[inline]
  fn from(other: FromHexError) -> Self {
    Self::new(ErrorKind::Other, other)
  }
}

impl From<DecodeError> for Error {
  #[inline]
  fn from(other: DecodeError) -> Self {
    Self::new(ErrorKind::DecodeJSON, other)
  }
}

impl From<EncodeError> for Error {
  #[inline]
  fn from(other: EncodeError) -> Self {
    Self::new(ErrorKind::EncodeJSON, other)
  }
}

// -----------------------------------------------------------------------------
// Error Kind
// -----------------------------------------------------------------------------

/// The general types of errors that may occur.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ErrorKind {
  /// Failed to decode a value from JSON.
  DecodeJSON,
  /// Failed to encode a value as JSON.
  EncodeJSON,
  /// A JSON-RPC 2.0 error response was received.
  JsonRpcErr,
  /// Failed attempt to parse a URL.
  InvalidUrl,
  /// Failed to send a JSON-RPC 2.0 request.
  TransportSend,
  /// Failed to receive a JSON-RPC 2.0 request.
  TransportRecv,
  /// An error that does not fall under any known category.
  Other,
}

// -----------------------------------------------------------------------------
// Error Representation
// -----------------------------------------------------------------------------

struct ErrorRepr {
  kind: ErrorKind,
  data: Box<dyn StdError + Send + Sync>,
}

impl ErrorRepr {
  #[inline]
  const fn new(kind: ErrorKind, data: Box<dyn StdError + Send + Sync>) -> Self {
    Self { kind, data }
  }
}
