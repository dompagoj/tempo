use super::*;
use chrono::{Datelike, Local, NaiveTime, Utc};
use core::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
struct MonthDisplay {
    month: u32,
    name: &'static str,
}

#[derive(Args, Debug)]
#[command(about = "Print the config file")]
pub struct PublishCommand {
    #[arg(short, long)]
    skip_pull: bool,
}

impl fmt::Display for MonthDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{} {}", self.month, self.name) }
}

pub fn command(config: Cfg, args: PublishCommand) -> anyhow::Result<()> {
    let user_data = config.user_data.initialize();
    let repos = &config.repos.initialize().list;

    // jira::authenticate(config);
    let now = Local::now();

    let selected_year = inquire::Select::new("Select year", get_years(now.year())).prompt()?;
    let selected_month = inquire::Select::new("Select month", get_months())
        .with_starting_cursor(now.month() as usize - 2)
        .prompt()?;

    let start_date = chrono::NaiveDate::from_ymd_opt(selected_year, selected_month.month, 1)
        .ok_or(anyhow::anyhow!("Failed to parse date"))?
        .and_time(NaiveTime::MIN);

    let res = construct_jira_payload(
        user_data.user_aliases.as_ref().unwrap_or(&vec![]),
        repos,
        start_date,
        !args.skip_pull,
    )?;

    dbg!(&res);

    Ok(())
}

fn get_years(curr: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in ((curr - 1)..curr + 1).rev() {
        res.push(i);
    }

    return res;
}

fn get_months() -> Vec<MonthDisplay> {
    vec![
        MonthDisplay {
            month: 1,
            name: chrono::Month::January.name(),
        },
        MonthDisplay {
            month: 2,
            name: chrono::Month::February.name(),
        },
        MonthDisplay {
            month: 3,
            name: chrono::Month::March.name(),
        },
        MonthDisplay {
            month: 4,
            name: chrono::Month::April.name(),
        },
        MonthDisplay {
            month: 5,
            name: chrono::Month::May.name(),
        },
        MonthDisplay {
            month: 6,
            name: chrono::Month::June.name(),
        },
        MonthDisplay {
            month: 7,
            name: chrono::Month::July.name(),
        },
        MonthDisplay {
            month: 8,
            name: chrono::Month::August.name(),
        },
        MonthDisplay {
            month: 9,
            name: chrono::Month::September.name(),
        },
        MonthDisplay {
            month: 10,
            name: chrono::Month::October.name(),
        },
        MonthDisplay {
            month: 11,
            name: chrono::Month::November.name(),
        },
        MonthDisplay {
            month: 12,
            name: chrono::Month::December.name(),
        },
    ]
}
