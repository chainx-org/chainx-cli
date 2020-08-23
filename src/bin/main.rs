use chainx_cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = cli::init();
    cmd.dispatch("wss://w1.chainx.org.cn/ws").await?;
    Ok(())
}
