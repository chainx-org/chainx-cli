use anyhow::Result;
use chainx_cli::App;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = App::init();
    cli.run().await?;

    Ok(())
}
