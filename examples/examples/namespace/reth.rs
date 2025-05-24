use std::collections::HashMap;
use ueb3::api::NsReth as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::eip::eip1898::BlockId;
use ueb3::types::eth::Address;
use ueb3::types::eth::Wei;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let input: BlockId = "0x4e3a3754410177e6937ef1f84bba68ea139e8d1a2258c5f85db9f1cd715a1bdd".parse()?;
  let value: HashMap<Address, Wei> = transport.balance_changes_in_block(input)?;
  println!("[reth_getBalanceChangesInBlock]: {value:#?}");

  Ok(())
}
