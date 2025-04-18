use ueb3::api::NsNet as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  println!("[net_version]:   {}", transport.version()?);
  println!("[net_listening]: {}", transport.listening()?);
  println!("[net_peerCount]: {}", transport.peer_count()?);

  if cfg!(feature = "node-besu") {
    println!("[net_enode]:    {}", transport.enode()?);
    println!("[net_services]: {:#?}", transport.services()?);
  }

  if cfg!(feature = "node-nethermind") {
    println!("[net_localAddress]: {}", transport.local_address()?);
    println!("[net_localEnode]:   {}", transport.local_enode()?);
  }

  Ok(())
}
