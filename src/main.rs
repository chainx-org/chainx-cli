mod app;
mod primitives;
mod rpc;
mod runtime;
mod types;
mod utils;
mod xpallets;

use self::app::App;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = App::init();
    cli.run().await?;

    Ok(())
}
