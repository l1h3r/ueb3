//! Ueb3

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod api {
  #[doc(inline)]
  pub use ueb3_api::*;
}

pub mod rpc {
  #[doc(inline)]
  pub use ueb3_rpc::*;
}

pub mod serde {
  #[doc(inline)]
  pub use ueb3_serde::*;
}

pub mod types {
  #[doc(inline)]
  pub use ueb3_types::*;
}

pub mod utils {
  #[doc(inline)]
  pub use ueb3_utils::*;
}

#[cfg(feature = "sled")]
pub mod sled {
  #[doc(inline)]
  pub use ueb3_sled::*;
}

#[cfg(feature = "ureq")]
pub mod ureq {
  #[doc(inline)]
  pub use ueb3_ureq::*;
}
