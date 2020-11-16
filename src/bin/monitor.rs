#![feature(async_closure)]
use chainx_cli::monitor::types::ReportedRoundStates;
use chainx_cli::monitor::EventType;
use chainx_cli::monitor::Monitor;
use jsonrpsee::common::Params;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut monitor = Monitor::new();

    monitor.chain(|client, monitor| {
        let client_handler = client.clone();
        let monitor_handler = monitor.clone();
        async_std::task::spawn(async move {
            let params = Params::Array(vec![]);
            let result: Result<ReportedRoundStates, jsonrpsee::client::RequestError> =
                client_handler.request("grandpa_roundState", params).await;
            match result {
                Ok(v) => {
                    monitor_handler.emit(EventType::TestEvent);
                    println!("current round: {}, signer: {}", v.best.round, v.best.prevotes.current_weight);
                }
                Err(_) => println!("not ok"),
            }
        });
    });

    monitor.on(EventType::TestEvent, || println!("event handled"));
    monitor.run().await?;

    Ok(())
}
