use super::*;

#[derive(Args, Debug)]
#[command(about = "Sets user configuration/secrets")]
pub struct ConfigureCommand {
    #[command(subcommand)]
    action: Option<ConfigureSubCommands>,

    #[arg(long)]
    jira_token: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigureSubCommands {
    Ls,
    Rm,
    #[command(subcommand)]
    Alias(UserAliasArgs),
}

#[derive(Subcommand, Debug)]
pub enum UserAliasArgs {
    Add { val: String },
    Rm,
    Ls,
}

#[derive(Args, Debug)]
pub struct CommandArgs {
    val: String,
}

pub fn command(config: Cfg, args: ConfigureCommand) -> anyhow::Result<()> {
    use ConfigureSubCommands::*;
    config.user_data.initialize();

    if args.action.is_some() {
        match args.action.unwrap() {
            Ls => {
                print_user_data(config.user_data.inner());
                bail_ok!();
            }
            Rm => {
                config.user_data.delete();
                bail_ok!("Deleted all user data");
            }
            Alias(args) => handle_alias(config, args)?,
        };
    }
    let user_data = config.user_data.inner_mut();

    if let Some(v) = args.jira_token {
        print_confirm("jira-token", &v);
        user_data.set_jira_token(v);
    }

    Ok(())
}

fn print_confirm(field: &str, value: &str) {
    println!("Succesfully saved: {} to: {}", field.bright_green(), value.green())
}

fn print_user_data(user_data: &UserData) {
    println!("{:#?}", &user_data);
}

fn handle_alias(cfg: Cfg, args: UserAliasArgs) -> anyhow::Result<()> {
    match args {
        UserAliasArgs::Add { val } => {
            let aliases = cfg.user_data.inner_mut().user_aliases.as_mut().unwrap();
            if aliases.contains(&val) {
                bail_ok!("{} Already added", val.bright_green());
            }

            aliases.push(val);
            bail_ok!("Added {}", aliases[aliases.len() - 1].bright_green());
        }
        UserAliasArgs::Rm => {
            let list = cfg.user_data.inner_mut().user_aliases.as_mut().unwrap();

            if list.is_empty() {
                bail_ok!("No aliases configured");
            }

            let res = match inquire::MultiSelect::new("Delete alias(s)", list.clone()).prompt() {
                Ok(val) => val,
                Err(_) => bail_ok!(),
            };

            if res.len() == list.len() {
                list.clear();
                bail_ok!("{}", "Removed aliases".red());
            }

            cfg.user_data.inner_mut().user_aliases = Some(
                list.iter()
                    .filter(|alias| !res.contains(*alias))
                    .cloned()
                    .collect::<Vec<_>>(),
            );
        }
        UserAliasArgs::Ls => {
            println!("LS");
        }
    };

    return Ok(());
}
