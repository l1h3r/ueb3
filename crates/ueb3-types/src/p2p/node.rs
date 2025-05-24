use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::net::IpAddr;
use std::net::SocketAddr;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::Enode;
use crate::eth::H256;
use crate::p2p::ChainConfig;

// -----------------------------------------------------------------------------
// Node Information
// -----------------------------------------------------------------------------

/// Information known about a host node.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct NodeInfo {
  /// The unique node identifier.
  pub id: String,
  /// The node client identifier (client type, version, OS, custom data).
  pub name: String,
  /// The enode URL of the node.
  pub enode: Enode,
  /// The node ENR (Ethereum Node Record).
  ///
  /// Reference: <https://eips.ethereum.org/EIPS/eip-778>
  pub enr: String,
  /// The IP address of the node.
  pub ip: NodeIp,
  /// The ports of the node.
  pub ports: NodePorts,
  /// The listening address of the node.
  pub listen_addr: SocketAddr,
  /// Additional protocol-specific information.
  pub protocols: NodeProtocols,
}

// -----------------------------------------------------------------------------
// Node IP Address
// -----------------------------------------------------------------------------

/// Node IP information.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NodeIp {
  /// IP address.
  Addr(IpAddr),
  /// Enode URL.
  Node(Enode),
}

impl Debug for NodeIp {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Addr(inner) => Debug::fmt(inner, f),
      Self::Node(inner) => Debug::fmt(inner, f),
    }
  }
}

impl Display for NodeIp {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Addr(inner) => Display::fmt(inner, f),
      Self::Node(inner) => Display::fmt(inner, f),
    }
  }
}

// -----------------------------------------------------------------------------
// Node Ports
// -----------------------------------------------------------------------------

/// Node port information.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodePorts {
  /// The UDP listening port for discovery protocol.
  pub discovery: u64,
  /// The TCP listening port for `RLPx`.
  pub listener: u64,
}

// -----------------------------------------------------------------------------
// Node Protocols
// -----------------------------------------------------------------------------

/// Node protocol-specific information.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeProtocols {
  /// The `eth` sub-protocol metadata.
  pub eth: Option<NodeInfoEth>,
  /// The `snap` sub-protocol metadata.
  pub snap: Option<NodeInfoSnap>,
}

impl Debug for NodeProtocols {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("NodeProtocols")
      .field("eth", &self.eth.format())
      .field("snap", &self.snap.format())
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Node Information (ETH Protocol)
// -----------------------------------------------------------------------------

/// The `eth` sub-protocol metadata known about the host node.
#[derive(Clone, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeInfoEth {
  /// The Ethereum network id.
  pub network: u64,
  /// The total difficulty of the node's blockchain.
  pub difficulty: Option<u128>,
  /// The SHA3 hash of the node's genesis block.
  pub genesis: H256,
  /// The chain configuration for the fork rules.
  pub config: ChainConfig,
  /// The hash of the node's best owned block.
  pub head: H256,
}

impl Debug for NodeInfoEth {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("NodeInfoEth")
      .field("network", &self.network)
      .field("difficulty", &self.difficulty.format())
      .field("genesis", &self.genesis)
      .field("config", &self.config)
      .field("head", &self.head)
      .finish()
  }
}

// -----------------------------------------------------------------------------
// Node Information (SNAP Protocol)
// -----------------------------------------------------------------------------

/// The `snap` sub-protocol metadata known about the host node.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeInfoSnap {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_ser_tokens;

  use crate::p2p::NodeInfo;
  use crate::p2p::NodeInfoEth;
  use crate::p2p::NodeInfoSnap;
  use crate::p2p::NodeIp;
  use crate::p2p::NodePorts;
  use crate::p2p::NodeProtocols;

  #[test]
  #[ignore]
  fn test_serde() {
    const ID: &str = "b7b61ea54ad081258a13a6d82920ce6719301f4670c458f64f0035e3463ec2df";
    const ENR: &str = "enr:-K64QL0-CI9BofDkirpulbV1OOOgqf5HLRMHr9iaziZInqI9HmeGOGZv2hs6J7olLu32LUMeYHTCjNBu3De_zlkI1fSGAY_h5ivyg2V0aMrJhPxk7ASDEYwwgmlkgnY0gmlwhJySOIOJc2VjcDI1NmsxoQM9h2JSiA4yEW_bUupWp47iuXieVbRBPekQ22lwLOk6f4RzbmFwwIN0Y3CCdl-DdWRwgpldhHVkcDaCdl8";
    const ENODE: &str = "enode://3d876252880e32116fdb52ea56a78ee2b9789e55b4413de910db69702ce93a7ff9a0b7c647a010a5e1e079c0aca146331083009644e12dc03510b9de9f50b9ef@156.146.56.131:30303?discport=39261";

    const NAME: &str = "Geth/v1.14.4-unstable-51327686/linux-arm64/go1.22.3";
    const HASH: &str = "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3";

    let a: NodeInfo = NodeInfo {
      id: String::new(),
      name: String::new(),
      enode: ENODE.parse().unwrap(),
      enr: String::new(),
      ip: NodeIp::Addr("127.0.0.1".parse().unwrap()),
      ports: NodePorts {
        discovery: 0,
        listener: 0,
      },
      listen_addr: "127.0.0.1:30303".parse().unwrap(),
      protocols: NodeProtocols {
        eth: None,
        snap: None,
      },
    };

    let b: NodeInfo = NodeInfo {
      id: ID.to_owned(),
      name: NAME.to_owned(),
      enode: ENODE.parse().unwrap(),
      enr: ENR.to_owned(),
      ip: NodeIp::Addr("156.146.56.131".parse().unwrap()),
      ports: NodePorts {
        discovery: 39261,
        listener: 30303,
      },
      listen_addr: "[::]:30303".parse().unwrap(),
      protocols: NodeProtocols {
        eth: Some(NodeInfoEth {
          network: 1,
          difficulty: Some(u128::from(u64::MAX)),
          genesis: HASH.parse().unwrap(),
          config: Default::default(),
          head: HASH.parse().unwrap(),
        }),
        snap: Some(NodeInfoSnap {}),
      },
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "NodeInfo",
        len: 8,
      },
      Token::Str("id"),
      Token::Str(""),
      Token::Str("name"),
      Token::Str(""),
      Token::Str("enode"),
      Token::Str(ENODE),
      Token::Str("enr"),
      Token::Str(""),
      Token::Str("ip"),
      Token::Str("127.0.0.1"),
      Token::Str("ports"),
      Token::Struct {
        name: "NodePorts",
        len: 2,
      },
      Token::Str("discovery"),
      Token::U64(0),
      Token::Str("listener"),
      Token::U64(0),
      Token::StructEnd,
      Token::Str("listenAddr"),
      Token::Str("127.0.0.1:30303"),
      Token::Str("protocols"),
      Token::Struct {
        name: "NodeProtocols",
        len: 2,
      },
      Token::Str("eth"),
      Token::None,
      Token::Str("snap"),
      Token::None,
      Token::StructEnd,
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "NodeInfo",
        len: 8,
      },
      Token::Str("id"),
      Token::Str(ID),
      Token::Str("name"),
      Token::Str(NAME),
      Token::Str("enode"),
      Token::Str(ENODE),
      Token::Str("enr"),
      Token::Str(ENR),
      Token::Str("ip"),
      Token::Str("156.146.56.131"),
      Token::Str("ports"),
      Token::Struct {
        name: "NodePorts",
        len: 2,
      },
      Token::Str("discovery"),
      Token::U64(39261),
      Token::Str("listener"),
      Token::U64(30303),
      Token::StructEnd,
      Token::Str("listenAddr"),
      Token::Str("[::]:30303"),
      Token::Str("protocols"),
      Token::Struct {
        name: "NodeProtocols",
        len: 2,
      },
      Token::Str("eth"),
      Token::Some,
      Token::Struct {
        name: "NodeInfoEth",
        len: 5,
      },
      Token::Str("network"),
      Token::U64(1),
      Token::Str("difficulty"),
      Token::Some,
      Token::U64(u64::MAX),
      Token::Str("genesis"),
      Token::Str(HASH),
      Token::Str("config"),
      Token::Struct {
        name: "ChainConfig",
        len: 1,
      },
      Token::Str("chainId"),
      Token::U64(0),
      Token::StructEnd,
      Token::Str("head"),
      Token::Str(HASH),
      Token::StructEnd,
      Token::Str("snap"),
      Token::Some,
      Token::Struct {
        name: "NodeInfoSnap",
        len: 0,
      },
      Token::StructEnd,
      Token::StructEnd,
      Token::StructEnd,
    ];

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
  }
}
