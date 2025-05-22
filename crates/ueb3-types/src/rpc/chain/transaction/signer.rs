use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use ueb3_serde::flatten::SerializeFlatten;
use ueb3_utils::traits::OptionExt as _;

use crate::eth::U64;
use crate::eth::U256;
use crate::eth::YParity;

// -----------------------------------------------------------------------------
// Ethereum Transaction Signer
// -----------------------------------------------------------------------------

/// A description of the creator of a transaction signature.
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct TransactionSigner {
  /// ECDSA recovery id.
  pub v: u64,
  /// ECDSA signature r.
  pub r: U256,
  /// ECDSA signature s.
  pub s: U256,
  /// The parity (0 for even, 1 for odd) of the y-value of the secp256k1 signature.
  pub y: Option<YParity>,
}

impl Debug for TransactionSigner {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("TransactionSigner")
      .field("v", &self.v)
      .field("r", &self.r)
      .field("s", &self.s)
      .field("y", &self.y.format())
      .finish()
  }
}

impl SerializeFlatten for TransactionSigner {
  const NAME: &'static str = "TransactionSigner";

  #[inline]
  fn fields(&self) -> usize {
    3 + usize::from(self.y.is_some())
  }

  #[inline]
  fn serialize_fields<S>(&self, state: &mut S) -> Result<(), S::Error>
  where
    S: SerializeStruct,
  {
    state.serialize_field("v", &U64::from_u64(self.v))?;
    state.serialize_field("r", &self.r)?;
    state.serialize_field("s", &self.s)?;

    if let Some(ref value) = self.y {
      state.serialize_field("yParity", value)?;
    } else {
      state.skip_field("yParity")?;
    }

    Ok(())
  }
}

impl Serialize for TransactionSigner {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.serialize_struct(serializer)
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_ser_tokens;

  use crate::rpc::TransactionSigner;

  #[test]
  fn test_serde() {
    const SIG_R: &str = "0x88ff6cf0fefd94db46111149ae4bfc179e9b94721fffd821d38d16464b3f71d0";
    const SIG_S: &str = "0x45e0aff800961cfce805daef7016b9b675c137a6a41a548f7b60a3484c06a33a";

    let a: TransactionSigner = TransactionSigner::default();

    let b: TransactionSigner = TransactionSigner {
      v: 0x1c,
      r: SIG_R.parse().unwrap(),
      s: SIG_S.parse().unwrap(),
      y: None,
    };

    let tokens_a: &[Token] = &[
      Token::Struct {
        name: "TransactionSigner",
        len: 3,
      },
      Token::Str("v"),
      Token::Str("0x0"),
      Token::Str("r"),
      Token::Str("0x0"),
      Token::Str("s"),
      Token::Str("0x0"),
      Token::StructEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Struct {
        name: "TransactionSigner",
        len: 3,
      },
      Token::Str("v"),
      Token::Str("0x1c"),
      Token::Str("r"),
      Token::Str(SIG_R),
      Token::Str("s"),
      Token::Str(SIG_S),
      Token::StructEnd,
    ];

    assert_ser_tokens(&a.readable(), tokens_a);
    assert_ser_tokens(&b.readable(), tokens_b);
  }
}
