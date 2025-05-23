#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod future;
mod namespace;
mod provider;
mod stream;

pub use self::future::PendingTransaction;

pub use self::namespace::NsNet;
pub use self::namespace::NsOts;
pub use self::namespace::NsRpc;
pub use self::namespace::NsTxPool;
pub use self::namespace::NsWeb3;

pub use self::provider::Provider;

pub use self::stream::FilterConfig;
pub use self::stream::FilterId;
pub use self::stream::FilterStream;

mod temporary {
  use serde::Deserialize;
  use serde::Serialize;

  /// Marker for unimplemented types.
  #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
  pub enum Unimplemented {}
}
