use super::*;

#[derive(Args, Debug)]
#[command(about = "Print the config file")]
pub struct DebugCommand {
    #[arg(long, help = "Only prints in progress tickets")]
    in_progress: bool
}
pub fn command(config: ConfigRef, args: DebugCommand) {
    if args.in_progress {
        println!("{:#?}", config.current);
        return;
    }

    println!("{:#?}", config);
}
