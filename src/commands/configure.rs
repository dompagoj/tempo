use super::*;

#[derive(Args, Debug)]
#[command(about = "Sets user configuration/secrets")]
pub struct ConfigureCommand {
    #[command(subcommand)]
    action: Option<ConfigureSubCommands>,

    #[arg(long)]
    tempo_api_key: Option<String>,

    #[arg(long)]
    name: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigureSubCommands {
    Ls,
}

#[derive(Args, Debug)]
pub struct CommandArgs {
    val: String,
}

pub fn command(config: ConfigRef, args: ConfigureCommand) {
    use ConfigureSubCommands::*;
    config.user_data.initialize();

    if args.action.is_some() {
        match args.action.unwrap() {
            Ls => print_user_data(&config.user_data),
        }
    }

    args.tempo_api_key.map(|v| config.user_data.set_tempo_api_key(v));
    args.name.map(|v| config.user_data.set_name(v));
}

fn print_user_data(user_data: &UserDataInner) {
    dbg!(&user_data);
}
