use std::path::PathBuf;

use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveTime, TimeZone, Utc, Weekday};
use git2::{BranchType, Commit, Repository, Sort};

const DAILY_STANDUP_ID: &str = "ART-1427";
#[allow(unused)]
const SPRINT_PLANNING_ID: &str = "ART-1427";
const MAX_TIME: Option<NaiveTime> = NaiveTime::from_hms_opt(23, 59, 59);

#[derive(Debug)]
struct GitCommitOccurance {
    ticket_id: String,
    comment: String,
    started: DateTime<Local>,
}

struct GitCommitTimeEntry {
    occurance: GitCommitOccurance,
    remaining_time: Duration,
}

#[derive(Debug)]
pub struct JiraTimeEntry {
    ticket_id: String,
    comment: String,
    started: DateTime<Local>,
    time_spent_seconds: i64,
}

pub fn construct_jira_payload(
    aliases: &[String],
    repos: &[PathBuf],
    start_date: chrono::NaiveDateTime,
    should_pull: bool,
) -> anyhow::Result<Vec<JiraTimeEntry>> {
    let opened_repos = repos
        .iter()
        .map(|path| -> anyhow::Result<Repository> {
            let full_path = std::fs::canonicalize(path)?;
            return Ok(Repository::open(full_path.clone())?);
        })
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();

    if should_pull {
        for opened_repo in opened_repos.iter() {
            crate::git_helpers::pull(opened_repo)?;
        }
    }

    let mut commits = Vec::new();
    for repo in opened_repos {
        add_commits_from_repo(&mut commits, start_date, aliases, repo)?;
    }
    commits.sort_unstable_by_key(|c| c.started);

    // dbg!(&commits);

    let parsed = parse_ticket_map(commits, start_date.year(), start_date.month());

    dbg!(&parsed);

    let sum = Duration::seconds(parsed.iter().map(|p| p.time_spent_seconds).sum());

    dbg!(sum.num_hours());

    return Ok(vec![]);
}

fn add_commits_from_repo(
    commits: &mut Vec<GitCommitOccurance>,
    start: chrono::NaiveDateTime,
    aliases: &[String],
    repo: Repository,
) -> anyhow::Result<()> {
    let filter_commit = |config_email: &str, commit: &Commit| {
        let author = commit.author();
        let a_email = author.email().unwrap();

        if a_email.contains(config_email) {
            return true;
        }

        if aliases.iter().any(|a| a.contains(a_email)) {
            return true;
        }

        return false;
    };

    let global_cfg = git2::Config::find_global()?;
    let cfg = git2::Config::open(global_cfg.as_path())?;

    dbg!(repo.head().unwrap().shorthand().unwrap());

    let email_entry = cfg.get_entry("user.email")?;
    let email = email_entry
        .value()
        .ok_or(anyhow::format_err!("Couldnt parse email from global git config"))?;

    let mut rev = repo.revwalk()?;
    let dev_branch = repo.find_branch("develop", BranchType::Local)?;
    rev.push(dev_branch.get().target().expect("Weird, branch has no target"))?;
    rev.set_sorting(Sort::TIME)?;

    let end = last_day_of_month(start.year(), start.month()).and_time(MAX_TIME.unwrap());

    for oid in rev {
        if oid.is_err() {
            continue;
        }
        let commit = repo.find_commit(oid.unwrap())?;
        let timestamp_utc = chrono::DateTime::from_timestamp(commit.time().seconds(), 0).unwrap();

        let timestamp = DateTime::from(timestamp_utc);
        let timestamp_naive = timestamp.naive_local();

        if timestamp_naive > end {
            continue;
        }

        if timestamp_naive < start {
            dbg!("Got to end");
            break;
        }

        if !filter_commit(email, &commit) {
            continue;
        }

        if commit.message().is_none() {
            continue;
        }

        let msg_ref = commit.message().unwrap();
        let semi_idx = match msg_ref.find(':') {
            Some(v) => v,
            None => continue,
        };

        commits.push(GitCommitOccurance {
            started: timestamp,
            comment: msg_ref[(semi_idx + 1)..].trim().to_string(),
            ticket_id: msg_ref[..semi_idx].trim().to_string(),
        });
    }

    return Ok(());
}

fn last_day_of_month(year: i32, month: u32) -> chrono::NaiveDate {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    return chrono::NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap() - Duration::days(1);
}

fn parse_ticket_map(commits: Vec<GitCommitOccurance>, year: i32, month: u32) -> Vec<JiraTimeEntry> {
    let last_day = last_day_of_month(year, month);
    let mut res = vec![];

    let days = (0..last_day.day())
        .rev()
        .map(|i| last_day - Duration::days(i as i64))
        .filter(|day| !matches!(day.weekday(), Weekday::Sun | Weekday::Sat))
        .collect::<Vec<_>>();

    let total_required_duration = Duration::minutes((days.len() * 8 * 60) as i64);
    let each_ticket_duration = total_required_duration / commits.len() as i32;

    dbg!(
        days.len(),
        total_required_duration.num_minutes(),
        total_required_duration.num_hours(),
        each_ticket_duration.num_hours(),
        commits.len(),
    );

    let mut commit_entries =
        commits
            .into_iter()
            .map(|c| GitCommitTimeEntry {
                occurance: c,
                remaining_time: each_ticket_duration,
            })
            .peekable();

    let max_per_day: Duration = Duration::hours(8);

    let daily_standup_duration = Duration::minutes(30);

    for day in days.iter() {
        let mut clocked_time = Duration::zero();
        let daily_date_utc = Utc
            .from_utc_datetime(&day.and_time(NaiveTime::from_hms_opt(15, 0, 0).unwrap()))
            .with_timezone(&Local);

        res.push(JiraTimeEntry {
            ticket_id: DAILY_STANDUP_ID.to_string(),
            time_spent_seconds: daily_standup_duration.num_seconds(),
            comment: "(Auto generated) Daily standup".to_string(),
            started: daily_date_utc,
        });
        clocked_time = clocked_time + daily_standup_duration;

        match commit_entries.peek_mut() {
            Some(commit) => {
                while commit.remaining_time > Duration::zero() {
                    // commit.remaining_time.max(max_per_day);

                    res.push(JiraTimeEntry {
                        ticket_id: commit.occurance.ticket_id.clone(),
                        time_spent_seconds: each_ticket_duration.num_seconds(),
                        comment: "(Auto generated) Daily standup".to_string(),
                        started: daily_date_utc,
                    });
                }
            }
            None => break,
        };

        // while let Some(val) = commits_iter.peek_mut() {
        //     res.push(JiraTimeEntry {
        //         ticket_id: val.ticket_id.clone(),
        //         time_spent_seconds: ticket_duration.num_seconds(),
        //         comment: format!("(Auto generated) {}", val.comment),
        //         started: daily_date_utc + clocked_time,
        //     });
        //     dbg!("Logged for ticket: {}", ticket_duration.num_hours());
        //     clocked_time = clocked_time + ticket_duration;
        //
        //     commits_iter.next();
        //     if clocked_time >= max_per_day {
        //         break;
        //     }
        // }
    }

    res
}
