use chainx_cli::*;

fn main() -> Result<()> {
    let cmd = cli::init();
    println!("{:?}", cmd);
    Ok(())
}
