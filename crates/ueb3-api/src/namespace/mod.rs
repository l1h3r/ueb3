//! References:
//!
//!  * [erigon](https://docs.erigon.tech/advanced/JSONRPC-daemon)
//!  * [ots](https://docs.otterscan.io/api-docs/ots-api)
//!  * [reth](https://github.com/paradigmxyz/reth/blob/main/crates/rpc/rpc-api/src/reth.rs)
//!  * [trace](https://openethereum.github.io/JSONRPC-trace-module)
//!  * [txpool](https://geth.ethereum.org/docs/interacting-with-geth/rpc/ns-txpool)

mod erigon;
mod net;
mod ots;
mod reth;
mod rpc;
mod trace;
mod txpool;
mod web3;

pub use self::erigon::NsErigon;
pub use self::net::NsNet;
pub use self::ots::NsOts;
pub use self::reth::NsReth;
pub use self::rpc::NsRpc;
pub use self::trace::NsTrace;
pub use self::txpool::NsTxPool;
pub use self::web3::NsWeb3;
