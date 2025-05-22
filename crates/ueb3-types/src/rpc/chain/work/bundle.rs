use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error as _;
use serde::de::SeqAccess;
use serde::de::Visitor;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use crate::eth::H256;
use crate::eth::U64;

// -----------------------------------------------------------------------------
// Ethereum Proof-of-Work
// -----------------------------------------------------------------------------

/// A proof-of-work bundle.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub struct WorkBundle {
  /// The proof-of-work hash.
  pub work_hash: H256,
  /// The seed hash.
  pub seed_hash: H256,
  /// The target (difficulty).
  pub target: H256,
  /// The block number.
  pub number: Option<u64>,
}

impl<'de> Deserialize<'de> for WorkBundle {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = WorkBundle;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a proof-of-work sequence")
      }

      fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
      where
        A: SeqAccess<'de>,
      {
        let work_hash: Option<H256> = access.next_element()?;
        let seed_hash: Option<H256> = access.next_element()?;
        let target: Option<H256> = access.next_element()?;
        let number: Option<U64> = access.next_element()?;

        let Some(work_hash) = work_hash else {
          return Err(A::Error::invalid_length(0, &self));
        };

        let Some(seed_hash) = seed_hash else {
          return Err(A::Error::invalid_length(1, &self));
        };

        let Some(target) = target else {
          return Err(A::Error::invalid_length(2, &self));
        };

        Ok(WorkBundle {
          work_hash,
          seed_hash,
          target,
          number: number.map(U64::into_u64),
        })
      }
    }

    deserializer.deserialize_seq(__Visitor)
  }
}

impl Serialize for WorkBundle {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let (work_hash, seed_hash, target): (H256, H256, H256) =
      (self.work_hash, self.seed_hash, self.target);

    if let Some(value) = self.number {
      (work_hash, seed_hash, target, U64::from_u64(value)).serialize(serializer)
    } else {
      (work_hash, seed_hash, target).serialize(serializer)
    }
  }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use serde_test::Configure as _;
  use serde_test::Token;
  use serde_test::assert_tokens;

  use crate::crypto::keccak256;
  use crate::rpc::WorkBundle;

  #[test]
  fn test_serde() {
    let a: WorkBundle = WorkBundle {
      work_hash: keccak256("foo"),
      seed_hash: keccak256("bar"),
      target: keccak256("baz"),
      number: None,
    };

    let b: WorkBundle = WorkBundle {
      work_hash: keccak256("foo"),
      seed_hash: keccak256("bar"),
      target: keccak256("baz"),
      number: Some(255),
    };

    let tokens_a: &[Token] = &[
      Token::Tuple { len: 3 },
      Token::Str("0x41b1a0649752af1b28b3dc29a1556eee781e4a4c3a1f7f53f90fa834de098c4d"),
      Token::Str("0x435cd288e3694b535549c3af56ad805c149f92961bf84a1c647f7d86fc2431b4"),
      Token::Str("0xf2d05ec5c5729fb559780c70a93ca7b4ee2ca37f64e62fa31046b324f60d9447"),
      Token::TupleEnd,
    ];

    let tokens_b: &[Token] = &[
      Token::Tuple { len: 4 },
      Token::Str("0x41b1a0649752af1b28b3dc29a1556eee781e4a4c3a1f7f53f90fa834de098c4d"),
      Token::Str("0x435cd288e3694b535549c3af56ad805c149f92961bf84a1c647f7d86fc2431b4"),
      Token::Str("0xf2d05ec5c5729fb559780c70a93ca7b4ee2ca37f64e62fa31046b324f60d9447"),
      Token::Str("0xff"),
      Token::TupleEnd,
    ];

    assert_tokens(&a.readable(), tokens_a);
    assert_tokens(&b.readable(), tokens_b);
  }
}
