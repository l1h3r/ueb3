use smol::stream::StreamExt as _;
use ueb3::api::FilterId;
use ueb3::api::FilterStream;
use ueb3::api::Provider as _;
use ueb3::rpc::Result;
use ueb3::rpc::RpcClient;
use ueb3::types::rpc::Filter;
use ueb3::ureq::Ureq;
use ueb3_examples::NODE;

const COUNT: usize = 3;

fn main() -> Result<()> {
  let transport: RpcClient = RpcClient::new(Ureq::new(NODE)?);

  let stream: FilterStream<'_, _, _> = transport.watch_logs(&Filter::new())?;
  let filter: FilterId = stream.filter();

  smol::block_on(async {
    let stream: _ = stream.into_stream().take(COUNT);

    smol::pin!(stream);

    while let Some(item) = stream.next().await {
      println!("[log]: {:#?}", item);
    }

    transport.uninstall_filter(filter)?;

    Ok(())
  })
}
