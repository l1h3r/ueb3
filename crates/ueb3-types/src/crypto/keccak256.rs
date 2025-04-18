use tiny_keccak::Hasher as _;
use tiny_keccak::Keccak;

use crate::eth::H256;
use crate::eth::Slice;

/// Returns the `Keccak-256` hash of the given data.
#[inline]
pub fn keccak256<T>(input: &T) -> H256
where
  T: AsRef<Slice> + ?Sized,
{
  fn inner(input: &Slice) -> H256 {
    let mut digest: Keccak = Keccak::v256();
    let mut output: H256 = H256::NULL;
    digest.update(input);
    digest.finalize(&mut output);
    output
  }

  inner(input.as_ref())
}
