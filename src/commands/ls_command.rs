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

const NAME_COLUMN_SIZE: usize = 35;
const SEPARATOR: &str = "-----------";

pub fn command(cfg: ConfigRef, args: LsCommand) {
    cfg.user_data.initialize();

    println!("{}", "Current tickets:".bright_green());
    println!("{}", SEPARATOR);
    println!(
        "{0: <SIZE$} | {1: <35} | {2: <35}",
        "Name",
        "Start date",
        "End date",
        SIZE = NAME_COLUMN_SIZE,
    );

    if cfg.current.is_empty() {
        println!("No current tickets");
    }

    for entry in cfg.current.iter().rev() {
        print_entry(entry);
    }

    if args.all {
        println!();
        println!("{}", "Archived".bright_green());
        println!("{}", SEPARATOR);
        println!(
            "{:<SIZE$} | {1: <SIZE$} | {2: <SIZE$}",
            "Name",
            "Start date",
            "End date",
            SIZE = NAME_COLUMN_SIZE
        );

        if cfg.entries.len() == 0 {
            println!("None in archive");
        } else {
            for entry in cfg.entries.iter().rev() {
                print_entry(entry);
            }
        }
    }
}

fn print_entry(entry: &TimeEntry) {
    let (date, month) = get_date_and_month(&entry.start_time);
    let end_date = if let Some(ref end_date_str) = entry.end_time {
        let (date, month) = get_date_and_month(end_date_str);

        format!(
            "{}/{}/{} ({}) | {}",
            date.day(),
            date.month(),
            date.year(),
            month.name(),
            format!("{}:{}:{}", date.hour(), date.minute(), date.second())
        )
    } else {
        String::from("In progress")
    };

    let mut temp_string = String::new();

    let name = if entry.ticket_name.len() < NAME_COLUMN_SIZE {
        &entry.ticket_name
    } else {
        temp_string.reserve_exact(NAME_COLUMN_SIZE + 1);
        temp_string.push_str(&entry.ticket_name[..NAME_COLUMN_SIZE - 3]);
        temp_string.push_str("...");

        &temp_string
    };

    println!(
        "{:<SIZE$} | {1: <SIZE$} | {2: <SIZE$}",
        name,
        format!(
            "{}/{}/{} ({}) | {}",
            date.day(),
            date.month(),
            date.year(),
            month.name(),
            format!("{}:{}:{}", date.hour(), date.minute(), date.second())
        ),
        end_date,
        SIZE = NAME_COLUMN_SIZE,
    );
}

fn get_date_and_month(date: &str) -> (chrono::DateTime<chrono::Local>, chrono::Month) {
    let date = DateTime::from_str(date).unwrap();
    let month = chrono::Month::from_u32(date.month()).unwrap();

    (date, month)
}
