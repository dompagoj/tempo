use crate::jira;

use super::*;
use anyhow::bail;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveTime};
use core::fmt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fmt::Formatter;

const MAX_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(23, 59, 59);

#[derive(Debug, Clone)]
struct MonthDisplay {
    month: u32,
    name: &'static str,
}

#[derive(Args, Debug)]
#[command(about = "Print the config file")]
pub struct PublishCommand {
    #[arg(short = 's', long)]
    skip_pull: bool,
}

impl fmt::Display for MonthDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.month, self.name)
    }
}

struct Step {
    pos: u32,
    len: u32,
}

impl Step {
    fn new(len: u32) -> Self {
        Self { pos: 1, len }
    }

    fn get_pos(&self) -> u32 {
        self.pos
    }

    fn inc_pos(&mut self) {
        self.pos += 1;
    }

    fn get_str(&mut self) -> String {
        assert!(self.get_pos() <= self.len);
        let str = format!("[{}/{}]", self.get_pos(), self.len);
        self.inc_pos();

        return str;
    }
}

pub fn command(config: Cfg, args: PublishCommand) -> anyhow::Result<()> {
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

    let vacation_days = get_day_range(
        "Did you have any vacation days?",
        "Pick vacation day",
        start_date.date(),
        end_date.date(),
    )?;
    let skip_days = get_day_range(
        "Do you want to skip any days?",
        "Pick skip day",
        start_date.date(),
        end_date.date(),
    )?;

    let mut step = Step::new(3);
    println!("{} Resolving repositories...", step.get_str().bold());
    let user_data = config.user_data.initialize();
    let repos = &config.repos.initialize().list;

    println!("{} Parsing commits...", step.get_str().bold());
    let jira_payload = construct_jira_payload(
        user_data.user_aliases.as_ref().unwrap_or(&vec![]),
        repos,
        start_date,
        end_date,
        vacation_days,
        skip_days,
        !args.skip_pull,
    )?;

    let sum = Duration::seconds(jira_payload.iter().map(|p| p.time_spent.num_seconds()).sum());

    for jira_entry in &jira_payload {
        println!(
            "       Date: {} Hours: {} Ticket: {} ({})",
            jira_entry.started,
            format!(
                "{}.{}",
                jira_entry.time_spent.num_hours(),
                ((jira_entry.time_spent.num_minutes() - jira_entry.time_spent.num_hours() * 60) as f32 / 60_f32)
                    .to_string()
                    .split('.')
                    .last()
                    .unwrap()
            )
            .yellow(),
            jira_entry.ticket_id.to_str().green(),
            match jira_entry.ticket_id {
                JiraTicketId::Pto => "PTO".red(),
                JiraTicketId::DailyStandup => "Daily standup".yellow(),
                JiraTicketId::Regular(_) => "Regular".bright_green(),
                JiraTicketId::Skipped => "Skipped".red(),
            }
        );
    }
    println!(
        "                     Total hours to be logged: {}",
        sum.num_hours().to_string().green()
    );

    if !inquire::Confirm::new("Publish to jira?").prompt()? {
        bail!("Canceled");
    }

    let pb = ProgressBar::new(jira_payload.len() as u64);
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {bar} {pos}/{len}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    pb.set_style(spinner_style);

    println!("{} Publishing to jira...", step.get_str().bold());
    pb.set_position(0);

    let mut published_hours = Duration::zero();

    for jira_entry in jira_payload
        .into_iter()
        .filter(|entry| !matches!(entry.ticket_id, JiraTicketId::Skipped))
    {
        match jira::publish_entry(config, &jira_entry) {
            Ok(res) => match res.status().as_u16() {
                200..=299 => {
                    published_hours = published_hours + jira_entry.time_spent;
                }
                _ => {
                    println!("Failed {} {}", res.status(), res.text()?);
                    println!("Published hours so far: {}", published_hours.num_hours());
                    if !inquire::Confirm::new("You have a failed requiest, continue?").prompt()? {
                        break;
                    }
                }
            },
            Err(err) => {
                println!("{} {:?}", "Request failed".bright_red(), err);
                break;
            }
        }

        pb.inc(1);
    }
    pb.finish_and_clear();
    println!("{}", "Done".green());
    println!(
        "Total hours published: {}",
        published_hours.num_hours().to_string().green()
    );

    Ok(())
}

fn get_years(curr: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in ((curr - 1)..curr + 1).rev() {
        res.push(i);
    }

    return res;
}

fn get_day_range(
    msg: &str,
    picker_msg: &str,
    min_date: NaiveDate,
    max_date: NaiveDate,
) -> anyhow::Result<Vec<NaiveDate>> {
    let mut buf = vec![];

    if inquire::Confirm::new(msg).prompt()? {
        let mut done = false;

        while !done {
            let res = inquire::DateSelect::new(picker_msg)
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

    return Ok(buf);
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
