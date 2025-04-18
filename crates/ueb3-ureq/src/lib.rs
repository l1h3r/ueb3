#![doc = include_str!("../README.md")]

use std::error::Error as StdError;
use std::time::Duration;
use ueb3_rpc::BoxRequest;
use ueb3_rpc::BoxResponse;
use ueb3_rpc::Error;
use ueb3_rpc::ErrorKind;
use ueb3_rpc::Result;
use ueb3_rpc::RpcTransport;
use ueb3_utils::traits::DecodeOwned as _;
use ueb3_utils::traits::Encode as _;
use ureq::Agent;
use ureq::config::Config;
use ureq::config::ConfigBuilder;
use ureq::config::RedirectAuthHeaders;
use ureq::http::header;
use ureq::http::uri::Uri;
use ureq::typestate::AgentScope;

// Re-export to allow easy custom configuration.
pub use ureq;

// -----------------------------------------------------------------------------
// Ureq Transport
// -----------------------------------------------------------------------------

/// An [`RpcTransport`] implementation based on [`ureq`].
///
/// [`ureq`]: <https://docs.rs/ureq/latest/ureq>
#[derive(Clone, Debug)]
pub struct Ureq {
  http: Agent,
  node: Uri,
}

impl Ureq {
  /// The default value of the `user-agent` header.
  pub const UAGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

  /// Creates a new `Ureq` transport.
  #[inline]
  pub fn new<T>(node: T) -> Result<Self>
  where
    T: TryInto<Uri>,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
  {
    let node: Uri = Self::parse_uri(node)?;
    let http: bool = Self::is_localhost(&node);

    let config: ConfigBuilder<AgentScope> = if http {
      Self::config().https_only(false)
    } else {
      Self::config()
    };

    Self::with_config(node, config.build())
  }

  /// Creates a new `Ureq` transport with the given `config`.
  #[inline]
  pub fn with_config<T>(node: T, config: Config) -> Result<Self>
  where
    T: TryInto<Uri>,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
  {
    Ok(Self {
      http: Agent::new_with_config(config),
      node: Self::parse_uri(node)?,
    })
  }

  /// Returns a reference to the underlying `Agent`.
  #[inline]
  pub const fn http(&self) -> &Agent {
    &self.http
  }

  /// Returns a reference to the node `Uri`.
  #[inline]
  pub const fn node(&self) -> &Uri {
    &self.node
  }

  #[inline]
  fn parse_uri<T>(input: T) -> Result<Uri>
  where
    T: TryInto<Uri>,
    T::Error: Into<Box<dyn StdError + Send + Sync>>,
  {
    input
      .try_into()
      .map_err(|error| Error::new(ErrorKind::InvalidUrl, error))
  }

  #[inline]
  fn is_localhost(uri: &Uri) -> bool {
    let Some(host) = uri.host() else {
      return false;
    };

    host == "127.0.0.1" || host == "localhost"
  }

  #[inline]
  fn config() -> ConfigBuilder<AgentScope> {
    Config::builder()
      .allow_non_standard_methods(false)
      .http_status_as_error(true)
      .https_only(true)
      .input_buffer_size(1024 * 128)
      .max_idle_age(Duration::from_secs(15))
      .max_idle_connections(100)
      .max_idle_connections_per_host(1)
      .max_redirects(0)
      .max_redirects_will_error(true)
      .max_response_header_size(1024 * 64)
      .no_delay(true)
      .output_buffer_size(1024 * 128)
      .redirect_auth_headers(RedirectAuthHeaders::Never)
      .save_redirect_history(false)
      .user_agent(Self::UAGENT)
  }
}

impl RpcTransport for Ureq {
  #[inline]
  fn call(&self, value: BoxRequest) -> Result<BoxResponse> {
    self
      .http
      .post(&self.node)
      .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
      .send(value.encode_vec()?)
      .map_err(|error| Error::new(ErrorKind::TransportSend, error))?
      .into_body()
      .read_to_vec()
      .map_err(|error| Error::new(ErrorKind::TransportRecv, error))
      .and_then(|body| BoxResponse::decode_vec(body).map_err(Into::into))
  }
}
