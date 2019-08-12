use chainx_cli::*;

fn main() -> Result<()> {
    let cmd = cli::init();
    cmd.sub_cmd.dispatch("wss://w1.chainx.org.cn/ws")?;
    Ok(())
}
