use serde::Deserialize;
use serde::Serialize;

// -----------------------------------------------------------------------------
// Parity Trace Category
// -----------------------------------------------------------------------------

/// A trace type identifier.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum TraceCategory {
  /// `call` trace type.
  Call,
  /// `create` trace type.
  Create,
  /// `reward` trace type.
  Reward,
  /// `suicide` trace type.
  Suicide,
}
