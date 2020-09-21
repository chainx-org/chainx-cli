mod app;
mod runtime;
mod utils;
mod xpallets;

use app::App;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let chainx_cli = App::from_args();
    chainx_cli.run().await?;

    Ok(())
}
