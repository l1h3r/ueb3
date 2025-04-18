use serde::Deserialize;
use serde::Serialize;

// -----------------------------------------------------------------------------
// Assorted Types
// -----------------------------------------------------------------------------

define_hash!(
  Address(20),
  "An Ethereum address as a fixed-size hash of 20 bytes."
);

define_hash!(
  Public(64),
  "An Ethereum public key as a fixed-size hash of 64 bytes."
);

define_hash!(
  Secret(32),
  "An Ethereum secret key as a fixed-size hash of 32 bytes."
);

define_hash!(
  Signature(65),
  "An Ethereum signature as a fixed-size hash of 65 bytes."
);

// -----------------------------------------------------------------------------
// Signature Y Parity
// -----------------------------------------------------------------------------

/// The parity of the y-value of a `secp256k1` signature.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[repr(u8)]
pub enum YParity {
  /// Even parity.
  #[serde(rename = "0x0")]
  Even = 0x0,
  /// Odd parity.
  #[serde(rename = "0x1")]
  Odd = 0x1,
}
