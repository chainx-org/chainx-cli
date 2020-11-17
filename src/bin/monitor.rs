#![feature(async_closure)]
use chainx_cli::monitor::EventType;
use chainx_cli::monitor::Monitor;

#[macro_use]
extern crate log;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut monitor = Monitor::new();

    monitor.chain(|client, monitor, header| {
        async_std::task::spawn(async move {
            let hash = client.finalized_head().await.unwrap();
            let finalized_header = client.header(Some(hash)).await.unwrap().unwrap();
            let unfinalized_header_height = header.number - finalized_header.number;
            debug!("#{}: {} blocks need to be finalized.", header.number, unfinalized_header_height); 
            if unfinalized_header_height > 3 {
                monitor.emit(EventType::SignerLack);
            }
        });
    });

    monitor.on(EventType::SignerLack, || {
        info!("SignerLack event handled.");
    });
    monitor.run().await?;

    Ok(())
}
