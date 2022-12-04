mod debug_config;
mod end;
mod ls_command;
mod start;
mod rm_cmd;
mod status;

use crate::config::*;
use clap::{command, Args, Parser, Subcommand};
use colored::Colorize;

pub type DateTime = chrono::DateTime<chrono::Local>;
type ConfigRef<'a> = &'a mut ConfigFile;

#[derive(Parser, Debug)]
#[command(
    author = "Domagoj",
    name = "Tempo",
    subcommand_required = true,
    bin_name = "tempo",
    about = "Terminal app to track time in tempo"
)]

pub enum Tempo {
    Start(start::StartCommand),
    End(end::EndCommand),
    Debug(debug_config::DebugCommand),
    Ls(ls_command::LsCommand),
    Rm(rm_cmd::RmCommand),
    Status,
}

impl Tempo {
    pub fn parse_wrap() -> Tempo {
        Tempo::parse()
    }

    pub fn run(self, config: ConfigRef) {
        println!();
        match self {
            Tempo::Start(args) => start::command(config, args),
            Tempo::End(args) => end::command(config, args),
            Tempo::Debug(args) => debug_config::command(config, args),
            Tempo::Ls(args) => ls_command::command(config, args),
            Tempo::Rm(args) => rm_cmd::command(config, args),
            Tempo::Status => status::command(config),
        }
    }
}
