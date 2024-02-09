use crate::{commands::unwrap_or_continue, time};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone, Utc, Weekday};
use git2::{BranchType, Commit, Repository, Sort};
use std::{ops::Add, path::PathBuf};

const DAILY_STANDUP_ID: &str = "ART-1777";
#[allow(unused)]
const SPRINT_PLANNING_ID: &str = "ART-1778";
const PTO_ID: &str = "ART-1790";

#[derive(Debug)]
struct GitCommitOccurance {
    ticket_id: String,
    comments: Vec<String>,
    started: DateTime<Local>,
}

#[derive(Debug)]
struct GitCommitTimeEntry {
    occurance: GitCommitOccurance,
    remaining_time: Duration,
}

#[derive(Debug)]
pub enum JiraTicketId {
    Regular(String),
    DailyStandup,
    Pto,
    Skipped,
}

impl JiraTicketId {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Pto => PTO_ID,
            Self::DailyStandup => DAILY_STANDUP_ID,
            Self::Regular(str) => str,
            Self::Skipped => "NULL",
        }
    }
}

#[derive(Debug)]
pub struct JiraTimeEntry {
    pub ticket_id: JiraTicketId,
    pub comment: String,
    pub started: DateTime<Local>,
    pub time_spent: Duration,
}

pub fn construct_jira_payload(
    aliases: &[String],
    repos: &[PathBuf],
    start_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
    vacation_days: Vec<NaiveDate>,
    skip_days: Vec<NaiveDate>,
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
        add_commits_from_repo(&mut commits, start_date, end_date, aliases, repo)?;
    }
    commits.sort_unstable_by_key(|c| c.started);

    let parsed = parse_ticket_map(commits, start_date.year(), start_date.month(), vacation_days, skip_days);

    return Ok(parsed);
}

fn add_commits_from_repo(
    commits: &mut Vec<GitCommitOccurance>,
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime,
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

    let email_entry = cfg.get_entry("user.email")?;
    let email = email_entry
        .value()
        .ok_or(anyhow::format_err!("Couldnt parse email from global git config"))?;

    let mut rev = repo.revwalk()?;
    let dev_branch = repo.find_branch("develop", BranchType::Local)?;
    rev.push(dev_branch.get().target().expect("Weird, branch has no target"))?;
    rev.set_sorting(Sort::TIME)?;

    for oid in rev.filter_map(|item| item.ok()) {
        let commit = repo.find_commit(oid)?;
        let timestamp_utc = chrono::DateTime::from_timestamp(commit.time().seconds(), 0).unwrap();

        let timestamp = DateTime::from(timestamp_utc);
        let timestamp_naive = timestamp.naive_local();

        if timestamp_naive > end {
            continue;
        }

        if timestamp_naive < start {
            break;
        }

        if !filter_commit(email, &commit) {
            continue;
        }

        if commit.message().is_none() {
            continue;
        }

        let msg_ref = unwrap_or_continue!(commit.message());
        let semi_idx = unwrap_or_continue!(msg_ref.find(':'));

        let ticket_id = msg_ref[..semi_idx].trim().to_string();
        let comment = msg_ref[(semi_idx + 1)..].trim().to_string();

        match commits.iter_mut().find(|item| item.ticket_id == ticket_id) {
            Some(val) => {
                val.comments.push(comment);
            }
            None => {
                commits.push(GitCommitOccurance {
                    started: timestamp,
                    ticket_id,
                    comments: vec![comment],
                });
            }
        }
    }

    return Ok(());
}

fn parse_ticket_map(
    commits: Vec<GitCommitOccurance>,
    year: i32,
    month: u32,
    vacation_days: Vec<NaiveDate>,
    skip_days: Vec<NaiveDate>,
) -> Vec<JiraTimeEntry> {
    let last_day = time::last_day_of_month(year, month);
    let mut res = vec![];

    let days = (0..last_day.day())
        .rev()
        .map(|i| last_day - Duration::days(i as i64))
        .filter(|day| !matches!(day.weekday(), Weekday::Sun | Weekday::Sat))
        .collect::<Vec<_>>();

    let total_required_duration = Duration::minutes((days.len() * 8 * 60) as i64);
    let each_ticket_duration = total_required_duration / commits.len() as i32;

    res.reserve(days.len() + commits.len());

    let mut commit_entries = commits
        .into_iter()
        .map(|c| GitCommitTimeEntry {
            occurance: c,
            remaining_time: each_ticket_duration,
        })
        .peekable();

    let daily_standup_duration = Duration::minutes(30);

    let mut total_logged = Duration::zero();

    for day in days.iter() {
        if skip_days.iter().any(|d| *d == *day) {
            res.push(JiraTimeEntry {
                ticket_id: JiraTicketId::Skipped,
                started: Local::now(),
                comment: String::from("empty"),
                time_spent: Duration::zero(),
            });

            continue;
        }

        if vacation_days.iter().any(|d| *d == *day) {
            let vacation_date_utc = Utc
                .from_utc_datetime(&day.and_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap()))
                .with_timezone(&Local);

            res.push(JiraTimeEntry {
                ticket_id: JiraTicketId::Pto,
                comment: "(Auto generated) PTO".to_string(),
                started: vacation_date_utc,
                time_spent: Duration::hours(8),
            });

            continue;
        }

        let daily_date_utc = Utc
            .from_utc_datetime(&day.and_time(NaiveTime::from_hms_opt(15, 0, 0).unwrap()))
            .with_timezone(&Local);

        res.push(JiraTimeEntry {
            ticket_id: JiraTicketId::DailyStandup,
            time_spent: daily_standup_duration,
            comment: "(Auto generated) Daily standup".to_string(),
            started: daily_date_utc,
        });
        total_logged = total_logged + daily_standup_duration;

        if total_logged >= total_required_duration {
            continue;
        }

        let next_commit = match commit_entries.peek_mut() {
            Some(c) => {
                if c.remaining_time > Duration::zero() {
                    c
                } else {
                    commit_entries.next();
                    match commit_entries.peek_mut() {
                        Some(c) => c,
                        None => continue,
                    }
                }
            }
            None => continue,
        };

        let spent = Duration::hours(7).add(Duration::minutes(30));
        res.push(JiraTimeEntry {
            ticket_id: JiraTicketId::Regular(next_commit.occurance.ticket_id.clone()),
            time_spent: spent,
            comment: format!("(Auto generated) \n{}", next_commit.occurance.comments.join("\n")),
            started: daily_date_utc + Duration::minutes(30),
        });
        next_commit.remaining_time = next_commit.remaining_time - spent;
        total_logged = total_logged + spent;
    }

    res
}
