#![allow(clippy::needless_return)]

extern crate core;

mod commands;
mod data;
mod git_helpers;
mod jira;
mod pretty_print;
mod step;
mod time;

use commands::Tempo;

fn main() {
    let args = Tempo::parse_wrap();
    let mut config = data::ConfigFile::new();

    args.run(&mut config);

    config.save();
}
