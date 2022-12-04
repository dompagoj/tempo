use super::*;
use chrono::{Datelike, Timelike};
use num_traits::FromPrimitive;
use std::str::FromStr;

#[derive(Args, Debug)]
#[command(about = "Print the config file")]
pub struct LsCommand {
    #[arg(short = 'a', long)]
    all: bool,
}

const NAME_COLUMN_SIZE: usize = 30;
const SEPARATOR: &str = "-----------";

pub fn command(cfg: ConfigRef, args: LsCommand) {
    println!("{}", "Current tickets:".bright_green());
    println!("{}", SEPARATOR);
    println!(
        "{:<SIZE$} | {1: <10}",
        "Name",
        "Start date",
        SIZE = NAME_COLUMN_SIZE
    );

    if cfg.current.is_empty() {
        println!("No current tickets");
    }

    for entry in cfg.current.iter() {
        print_entry(entry);
    }

    if args.all {
        println!();
        println!("{}", "Archived".bright_green());
        println!("{}", SEPARATOR);
        println!(
            "{:<SIZE$} | {1: <10}",
            "Name",
            "Start date",
            SIZE = NAME_COLUMN_SIZE
        );

        if cfg.entries.len() == 0 {
            println!("None in archive");
        } else {
            for entry in cfg.entries.iter() {
                print_entry(entry);
            }
        }
    }
}

fn print_entry(entry: &TimeEntry) {
    let date = DateTime::from_str(&entry.start_time).unwrap();
    let month = chrono::Month::from_u32(date.month()).unwrap();

    let mut name = entry.ticket_name.clone();
    if name.len() > NAME_COLUMN_SIZE {
        name.truncate(NAME_COLUMN_SIZE);
        name.replace_range(NAME_COLUMN_SIZE - 3.., "...");
    }

    println!(
        "{:<SIZE$} | {1: <10}",
        name,
        format!(
            "{}/{}/{} ({}) | {}",
            date.day(),
            date.month(),
            date.year(),
            month.name(),
            format!("{}:{}:{}", date.hour(), date.minute(), date.second())
        ),
        SIZE = NAME_COLUMN_SIZE,
    );
}
