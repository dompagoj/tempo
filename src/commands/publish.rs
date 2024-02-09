use crate::{jira, pretty_print, step::Step, time};

use super::*;
use anyhow::bail;
use chrono::Duration;

#[derive(Args, Debug)]
#[command(about = "Publish hours to jira time tracking")]
pub struct PublishCommand {
    #[arg(short = 's', long)]
    skip_pull: bool,
}

pub fn command(config: Cfg, args: PublishCommand) -> anyhow::Result<()> {
    let (start_date, end_date) = time::get_date_range_from_user()?;

    let vacation_days = time::get_day_range(
        "Did you have any vacation days?",
        "Pick vacation day",
        start_date.date(),
        end_date.date(),
    )?;
    let skip_days = time::get_day_range(
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
        user_data.get_user_aliases(),
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

    let pb = pretty_print::get_progress_bar(jira_payload.len() as u64);
    println!("{} Publishing to jira...", step.get_str().bold());

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
