use anyhow::Result;
use structopt::StructOpt;

use crate::utils::build_client;

/// Monitor
#[derive(Debug, StructOpt)]
pub enum Monitor {}

impl Monitor {
    pub async fn run(self, url: String) -> Result<()> {
        match build_client(url).await {
            Ok(client) => {
                let mut new_header = client.subscribe_blocks().await.unwrap();
                let head = new_header.next().await;
                println!("{}", head.parent_hash);
            },
            _ => println!("not ok")
        }
        Ok(())
    }
}
