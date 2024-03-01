use std::fmt::Formatter;

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
use colored::Colorize;

pub const MAX_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(23, 59, 59);

#[derive(Debug, Clone)]
pub struct MonthDisplay {
    month: u32,
    name: &'static str,
}

impl std::fmt::Display for MonthDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.month, self.name)
    }
}

pub fn get_date_range_from_user() -> anyhow::Result<(NaiveDateTime, NaiveDateTime)> {
    let now = Local::now();
    let starting_cursor = if now.month() <= 2 { 0 } else { now.month() as usize - 2 };

    let selected_year = inquire::Select::new("Select year", get_years(now.year())).prompt()?;
    let selected_month = inquire::Select::new("Select month", get_months())
        .with_starting_cursor(starting_cursor)
        .prompt()?;

    let start_date = chrono::NaiveDate::from_ymd_opt(selected_year, selected_month.month, 1)
        .ok_or(anyhow::anyhow!("Failed to parse date"))?
        .and_time(NaiveTime::MIN);

    let end_date = last_day_of_month(start_date.year(), start_date.month()).and_time(MAX_TIME.unwrap());

    Ok((start_date, end_date))
}

pub fn get_years(curr: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in ((curr - 1)..curr + 1).rev() {
        res.push(i);
    }

    return res;
}

pub fn get_day_range(
    msg: &str,
    picker_msg: &str,
    min_date: NaiveDate,
    max_date: NaiveDate,
) -> anyhow::Result<Vec<NaiveDate>> {
    let mut buf = vec![];

    if inquire::Confirm::new(msg).prompt()? {
        let mut done = false;

        while !done {
            dbg!(min_date, max_date);
            let res = inquire::DateSelect::new(picker_msg)
                .with_default(min_date)
                .with_min_date(min_date)
                .with_max_date(max_date)
                .prompt()?;

            if buf.iter().any(|d| *d == res) {
                println!("{}", "Already added".red());
            } else {
                buf.push(res);
            }

            done = !inquire::Confirm::new("Add more?").prompt()?;
        }
    }

    Ok(buf)
}

pub fn get_months() -> Vec<MonthDisplay> {
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

pub fn last_day_of_month(year: i32, month: u32) -> chrono::NaiveDate {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    return chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap() - chrono::Duration::days(1);
}
