mod app;
mod primitives;
mod runtime;
mod utils;
mod xpallet;

use self::app::App;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = App::init();
    cli.run().await?;

    Ok(())
}
