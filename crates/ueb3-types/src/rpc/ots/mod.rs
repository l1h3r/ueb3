//! [ots] JSON-RPC Types
//!
//! [ots]: https://docs.otterscan.io/api-docs/ots-api

mod block;
mod creator;
mod issuance;
mod operation;
mod receipt;
mod trace;

pub use self::block::BlockDetails;
pub use self::block::BlockTransactionSearch;
pub use self::block::BlockTransactions;
pub use self::creator::ContractCreator;
pub use self::issuance::Issuance;
pub use self::operation::Operation;
pub use self::operation::OperationType;
pub use self::receipt::TaggedReceipt;
pub use self::trace::TransactionTrace;
