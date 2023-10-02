use super::*;

#[derive(Args, Debug)]
#[command(about = "Print the config file")]
pub struct DebugCommand {}

pub fn command(config: Cfg, _args: DebugCommand) -> anyhow::Result<()> {
    println!("{:#?}", config);

    Ok(())
}
