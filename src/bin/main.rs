use chainx_cli::*;

fn main() -> Result<()> {
    let cmd = cli::init();
    let (_handle, chainx) = http_connect("http://47.99.192.159:8086")?;
    /*let (_handle, chainx) = ws_connect("ws://127.0.0.1:8087")?;*/
    cmd.dispatch(chainx)?;
    Ok(())
}
