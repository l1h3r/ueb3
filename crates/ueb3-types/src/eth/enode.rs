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
use ueb3_utils::base16::FromHexError;

use crate::eth::H512;

// -----------------------------------------------------------------------------
// Parse Error
// -----------------------------------------------------------------------------

/// The error type returned from a failed attempt to parse an enode URI.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnodeParseError {
  /// The provided format is invalid.
  InvalidFormat,
  /// The provided scheme is invalid.
  InvalidScheme,
  /// The provided node identifier is invalid.
  InvalidName(FromHexError),
  /// The provided host address is invalid.
  InvalidHost(AddrParseError),
  /// The provided port number is invalid.
  InvalidPort(ParseIntError),
}

impl Display for EnodeParseError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::InvalidFormat => f.write_str("invalid URI format"),
      Self::InvalidScheme => f.write_str("invalid scheme"),
      Self::InvalidName(inner) => write!(f, "invalid node identifier: {inner}"),
      Self::InvalidHost(inner) => write!(f, "invalid host address: {inner}"),
      Self::InvalidPort(inner) => write!(f, "invalid port number: {inner}"),
    }
  }
}

impl StdError for EnodeParseError {}

// -----------------------------------------------------------------------------
// Ethereum Node Identifier
// -----------------------------------------------------------------------------

/// A URL-style Ethereum node identifier.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Enode {
  node: H512,
  host: IpAddr,
  tcp: u16,
  udp: u16,
}

impl Enode {
  /// Returns the node public key.
  #[inline]
  pub const fn node(&self) -> &H512 {
    &self.node
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

  #[inline]
  fn split_str(mut input: &str) -> Option<[&str; 5]> {
    let mut parts: [&str; 5] = [""; 5];

    (parts[0], input) = input.split_once("://")?;
    (parts[1], input) = input.split_once('@')?;
    (parts[2], input) = input.split_once(':')?;

    if let Some((head, tail)) = input.split_once("?discport=") {
      parts[3] = head;
      parts[4] = tail;
    } else {
      parts[3] = input;
      parts[4] = input;
    }

    Some(parts)
  }

  #[inline]
  fn parse_str(input: &str) -> Result<Self, EnodeParseError> {
    let Some([scheme, node, host, tcp, udp]) = Self::split_str(input) else {
      return Err(EnodeParseError::InvalidFormat);
    };

    if scheme != "enode" {
      return Err(EnodeParseError::InvalidScheme);
    }

    Ok(Self {
      node: node.parse().map_err(EnodeParseError::InvalidName)?,
      host: host.parse().map_err(EnodeParseError::InvalidHost)?,
      tcp: tcp.parse().map_err(EnodeParseError::InvalidPort)?,
      udp: udp.parse().map_err(EnodeParseError::InvalidPort)?,
    })
  }
}

// -----------------------------------------------------------------------------
// Implementations - Debug/Display
// -----------------------------------------------------------------------------

impl Debug for Enode {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self, f)
  }
}

impl Display for Enode {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let Self {
      node,
      host,
      tcp,
      udp,
    } = self;

    let node: String = format!("{node}");
    let node: &str = node.trim_start_matches("0x");

    if tcp == udp {
      write!(f, "enode://{node}@{host}:{tcp}")
    } else {
      write!(f, "enode://{node}@{host}:{tcp}?discport={udp}")
    }
  }
}

// -----------------------------------------------------------------------------
// Implementations - FromStr
// -----------------------------------------------------------------------------

impl FromStr for Enode {
  type Err = EnodeParseError;

  #[inline]
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Self::parse_str(input)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Serde
// -----------------------------------------------------------------------------

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
    serializer.collect_str(self)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use std::net::IpAddr;
  use std::net::Ipv4Addr;

  use crate::eth::Enode;
  use crate::eth::H512;

  static PKEY: &str = "6f8a80d14311c39f35f516fa664deaaaa13e85b2f7493f37f6144d86991ec012937307647bd3b9a82abe2974e1407241d54947bbb39763a4cac9f77166ad92a0";

  #[test]
  fn test_parse_no_udp() {
    let node: H512 = PKEY.parse().unwrap();
    let text: String = format!("enode://{PKEY}@127.0.0.1:30303");
    let data: Enode = text.parse().unwrap();

    assert_eq!(data.node(), &node);
    assert_eq!(data.host(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(data.tcp(), 30303);
    assert_eq!(data.udp(), 30303);
    assert_eq!(text, format!("{data}"));
  }

  #[test]
  fn test_parse_udp() {
    let node: H512 = PKEY.parse().unwrap();
    let text: String = format!("enode://{PKEY}@127.0.0.1:30303?discport=30301");
    let data: Enode = text.parse().unwrap();

    assert_eq!(data.node(), &node);
    assert_eq!(data.host(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(data.tcp(), 30303);
    assert_eq!(data.udp(), 30301);
    assert_eq!(text, format!("{data}"));
  }
}
