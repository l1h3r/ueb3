use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::net::AddrParseError;
use std::net::IpAddr;
use std::num::ParseIntError;
use std::str::FromStr;

// -----------------------------------------------------------------------------
// Parse Error
// -----------------------------------------------------------------------------

/// The error type returned from a failed attempt to parse an enode URI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnodeParseError {
  /// The `enode://` scheme is missing.
  MissingScheme,
  /// The provided scheme is invalid.
  InvalidScheme,
  /// The node public key is missing.
  MissingPKey,
  /// The host address is missing.
  MissingHost,
  /// The provided host address is invalid.
  InvalidHost(AddrParseError),
  /// The provided port number is invalid.
  InvalidPort(ParseIntError),
}

impl Display for EnodeParseError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::MissingScheme => f.write_str("scheme missing"),
      Self::InvalidScheme => f.write_str("invalid scheme"),
      Self::MissingPKey => f.write_str("public key missing"),
      Self::MissingHost => f.write_str("host missing"),
      Self::InvalidHost(inner) => write!(f, "invalid host: {inner}"),
      Self::InvalidPort(inner) => write!(f, "invalid port: {inner}"),
    }
  }
}

impl StdError for EnodeParseError {}

impl From<AddrParseError> for EnodeParseError {
  #[inline]
  fn from(other: AddrParseError) -> Self {
    Self::InvalidHost(other)
  }
}

impl From<ParseIntError> for EnodeParseError {
  #[inline]
  fn from(other: ParseIntError) -> Self {
    Self::InvalidPort(other)
  }
}

// -----------------------------------------------------------------------------
// Ethereum Node Identifier
// -----------------------------------------------------------------------------

/// A URL-style Ethereum node identifier.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Enode {
  name: String,
  host: IpAddr,
  tcp: u16,
  udp: u16,
}

impl Enode {
  /// Returns the node public key.
  #[inline]
  pub fn pkey(&self) -> &str {
    self.pkey.as_str()
  }

  /// Returns the host IP address.
  #[inline]
  pub const fn host(&self) -> IpAddr {
    self.host
  }

  /// Returns the TCP port.
  #[inline]
  pub const fn tcp(&self) -> u16 {
    self.tcp
  }

  /// Returns the UDP port.
  #[inline]
  pub const fn udp(&self) -> u16 {
    self.udp
  }
}

impl Debug for Enode {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let Self {
      pkey,
      host,
      tcp,
      udp,
    } = self;

    if tcp == udp {
      write!(f, "enode://{pkey}@{host}:{tcp}")
    } else {
      write!(f, "enode://{pkey}@{host}:{tcp}?discport={udp}")
    }
  }
}

impl Display for Enode {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self, f)
  }
}

impl FromStr for Enode {
  type Err = EnodeParseError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let Some((scheme, rest)) = input.split_once("://") else {
      return Err(EnodeParseError::MissingScheme);
    };

    if scheme != "enode" {
      return Err(EnodeParseError::InvalidScheme);
    }

    let Some((pkey, rest)) = rest.split_once('@') else {
      return Err(EnodeParseError::MissingPKey);
    };

    let Some((host, rest)) = rest.split_once(':') else {
      return Err(EnodeParseError::MissingHost);
    };

    let mut this: Self = Self {
      pkey: pkey.to_owned(),
      host: host.parse()?,
      tcp: 0,
      udp: 0,
    };

    if let Some((tcp, udp)) = rest.split_once("?discport=") {
      this.tcp = tcp.parse()?;
      this.udp = udp.parse()?;
    } else {
      this.tcp = rest.parse()?;
      this.udp = this.tcp;
    }

    Ok(this)
  }
}

impl<'de> Deserialize<'de> for Enode {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let input: Cow<'_, str> = Cow::deserialize(deserializer)?;
    let value: Self = input.parse().map_err(D::Error::custom)?;

    Ok(value)
  }
}

impl Serialize for Enode {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&format!("{self}"))
  }
}

#[cfg(test)]
mod tests {
  use serde_json::from_str;
  use serde_json::to_string;
  use std::net::IpAddr;
  use std::net::Ipv4Addr;

  use crate::eth::Enode;

  static PKEY: &str = "6f8a80d14311c39f35f516fa664deaaaa13e85b2f7493f37f6144d86991ec012937307647bd3b9a82abe2974e1407241d54947bbb39763a4cac9f77166ad92a0";

  #[test]
  fn test_serde_no_udp() {
    let text: String = format!(r#""enode://{PKEY}@127.0.0.1:30303""#);
    let data: Enode = from_str(text.as_str()).unwrap();
    let json: String = to_string(&data).unwrap();

    assert_eq!(data.pkey(), PKEY);
    assert_eq!(data.host(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(data.tcp(), 30303);
    assert_eq!(data.udp(), 30303);

    assert_eq!(json, text);
  }

  #[test]
  fn test_serde_udp() {
    let text: String = format!(r#""enode://{PKEY}@127.0.0.1:30303?discport=30301""#);
    let data: Enode = from_str(text.as_str()).unwrap();
    let json: String = to_string(&data).unwrap();

    assert_eq!(data.pkey(), PKEY);
    assert_eq!(data.host(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(data.tcp(), 30303);
    assert_eq!(data.udp(), 30301);

    assert_eq!(json, text);
  }
}
