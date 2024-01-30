use super::*;

#[derive(Args, Debug)]
#[command(about = "Print the config file")]
pub struct DebugCommand {}

pub fn command(config: Cfg, _args: DebugCommand) -> anyhow::Result<()> {
    config.user_data.initialize();
    config.repos.initialize();
    println!("{:#?}", *config.user_data);
    println!("{:#?}", *config.repos);

    Ok(())
}
