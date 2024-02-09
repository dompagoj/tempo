mod configure;
mod debug_config;
mod delete;
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
    Delete(delete::DeleteCommand),
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

macro_rules! unwrap_or_continue {
    ($option: expr) => {
        match $option {
            Some(val) => val,
            None => continue,
        }
    };
}

pub(crate) use bail_ok;
pub(crate) use unwrap_or_continue;

impl Tempo {
    pub fn parse_wrap() -> Self {
        Tempo::parse()
    }

    pub fn run(self, config: Cfg) {
        println!();
        let res = match self {
            Tempo::Debug(args) => debug_config::command(config, args),
            Tempo::Publish(args) => publish::command(config, args),
            Tempo::Configure(args) => configure::command(config, args),
            Tempo::Repo(args) => repo::command(config, args.action),
            Tempo::Delete(args) => delete::command(config, args),
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
