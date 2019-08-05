use chainx_cli::*;

fn main() -> Result<()> {
    let cmd = cli::init();
    let (_handle, chainx) = ws_connect("wss://w1.chainx.org.cn/ws")?;
    /*let (_handle, chainx) = http_connect("http://127.0.0.1:8086")?;*/
    cmd.sub_cmd.dispatch(chainx)?;
    Ok(())
}
