use ueb3::api::NsRpc as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  println!("[rpc_modules]: {:#?}", transport.modules()?);

  Ok(())
}
