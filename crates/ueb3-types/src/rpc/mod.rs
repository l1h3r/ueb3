//! Core JSON-RPC Types

mod chain;

// Re-export for convenience when handling arbitrary JSON data.
pub use serde_json::Value;
pub use serde_json::value::RawValue;

pub use self::chain::Block;
pub use self::chain::BlockNumber;
pub use self::chain::FeeHistory;
pub use self::chain::Filter;
pub use self::chain::FilterBlock;
pub use self::chain::FilterRange;
pub use self::chain::FilterSet;
pub use self::chain::SyncResult;
pub use self::chain::SyncStatus;
pub use self::chain::Transaction;
pub use self::chain::TransactionEip1559;
pub use self::chain::TransactionEip2930;
pub use self::chain::TransactionEip4844;
pub use self::chain::TransactionEip7702;
pub use self::chain::TransactionHeader;
pub use self::chain::TransactionId;
pub use self::chain::TransactionLegacy;
pub use self::chain::TransactionLog;
pub use self::chain::TransactionObject;
pub use self::chain::TransactionReceipt;
pub use self::chain::TransactionRequest;
pub use self::chain::TransactionSigner;
pub use self::chain::WorkBundle;
