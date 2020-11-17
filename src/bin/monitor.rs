#![feature(async_closure)]
use chainx_cli::monitor::EventType;
use chainx_cli::monitor::Monitor;
use once_cell::sync::OnceCell;
use pickledb::{PickleDb, PickleDbDumpPolicy};
use std::sync::RwLock;

#[macro_use]
extern crate log;

fn store() -> &'static RwLock<PickleDb> {
    static STORE: OnceCell<RwLock<PickleDb>> = OnceCell::new();
    STORE.get_or_init(|| {
        RwLock::new(PickleDb::new_yaml(
            "db.yml",
            PickleDbDumpPolicy::PeriodicDump(std::time::Duration::from_secs(10)),
        ))
    })
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let mut monitor = Monitor::new();

    monitor
        .chain(|client, monitor| {
            async_std::task::spawn(async move {
                let hash = client.finalized_head().await.unwrap();
                let finalized_header = client.header(Some(hash)).await.unwrap().unwrap();
                let hash = client.head().await.unwrap();
                let header = client.header(Some(hash)).await.unwrap().unwrap();
                let unfinalized_header_height = header.number - finalized_header.number;
                debug!(
                    "#{}: {} blocks need to be finalized.",
                    header.number, unfinalized_header_height
                );
                if unfinalized_header_height > 4 {
                    monitor.emit(EventType::SignerLack);
                }
            });
        })
        .chain(|client, monitor| {
            async_std::task::spawn(async move {
                let hash = client.head().await.unwrap().to_string();
                debug!("Head block hash: {}", hash);
                match store().write() {
                    Ok(mut store) => {
                        if let Some(v) = store.get::<String>("head_hash") {
                            if v == hash {
                                let tick = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;
                                let old_tick = store.get::<u64>("last_receive_tick").unwrap();
                                if tick - old_tick > 10_000 {
                                    monitor.emit(EventType::NewBlockReceiveTimeout);
                                }
                                return;
                            }
                        }
                        let _ = store.set("head_hash", &hash);
                        let tick = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;
                        let _ = store.set("last_receive_tick", &tick);
                        info!("initialized head hash with {}, tick with {}", hash, tick);
                    }
                    Err(_) => {}
                }
            });
        });

    monitor
        .on(EventType::SignerLack, || {
            info!("SignerLack event handled.");
        })
        .on(EventType::RpcTimeout, || {
            info!("RpcTimeout event handled.");
        })
        .on(EventType::NewBlockReceiveTimeout, || {
            info!("NewBlockReceiveTimeout event handled.");
        });

    monitor.run().await;
    Ok(())
}
