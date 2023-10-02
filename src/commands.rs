mod configure;
mod debug_config;
mod publish;
mod repo;

use crate::data::*;
use clap::{command, Args, Parser, Subcommand};
use colored::Colorize;

type Cfg<'a> = &'a mut ConfigFile;

#[derive(Parser, Debug)]
#[command(
    author = "Domagoj",
    name = "Tempo",
    subcommand_required = true,
    bin_name = "tempo",
    about = "Terminal app to log time in jira"
)]
pub enum Tempo {
    Debug(debug_config::DebugCommand),
    Publish(publish::PublishCommand),
    Configure(configure::ConfigureCommand),
    Repo(repo::RepoCommand),
}

macro_rules! bail_ok {
    () => {
        return Ok(())
    };
    ($msg: tt) => {
        println!("{}", $msg.green());
        return Ok(());
    };

    ($format: tt, $($str: expr),+) => {
        println!($format, $($str),+);
        return Ok(());
    };
    ($format: tt, $($str: tt),+) => {
        println!($format, $($str),+);
        return Ok(());
    };

}

pub(crate) use bail_ok;

impl Tempo {
    pub fn parse_wrap() -> Self { Tempo::parse() }

    pub fn run(self, config: Cfg) {
        println!();
        let res = match self {
            Tempo::Debug(args) => debug_config::command(config, args),
            Tempo::Publish(args) => publish::command(config, args),
            Tempo::Configure(args) => configure::command(config, args),
            Tempo::Repo(args) => repo::command(config, args.action),
        };

        match res {
            Ok(_) => {
                println!();
            }
            Err(err) => {
                println!("{}: {}", "Err".bright_red(), err.root_cause().to_string().bright_red());
            }
        }
    }
}
