use crate::{
    data::{ConfigFile, JiraTimeEntry},
    pretty_print,
};
use anyhow::Context;
use chrono::{DateTime, Duration, Local, NaiveDateTime};
use reqwest::blocking::Response;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddWorkLogPayload {
    started: String,
    time_spent_seconds: i64,
}

pub fn publish_entry(config: &ConfigFile, entry: &JiraTimeEntry) -> anyhow::Result<Response> {
    let jira_token = config.user_data.inner().get_jira_token();
    let client = reqwest::blocking::Client::new();

    let json = json!({
      "comment": entry.comment,
      "started": format_date(&entry.started),
      "timeSpentSeconds": entry.time_spent.num_seconds(),
    })
    .to_string();

    return Ok(client
        .post(format!(
            "https://jira.internetbrands.com/rest/api/2/issue/{}/worklog?adjustEstimate=leave",
            entry.ticket_id.to_str(),
        ))
        .body(json)
        .header("Authorization", format!("Bearer {}", jira_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()?);
}
// TODO: Remove all this logic from this file, seperate all calls to jira into a seperate functions and call them from the delete command
pub fn delete_worklogs(
    config: &mut ConfigFile,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> anyhow::Result<()> {
    config.user_data.initialize();
    let jira_token = config.user_data.inner().get_jira_token();
    let client = reqwest::blocking::Client::new();

    let user = client
        .get("https://jira.internetbrands.com/rest/api/2/myself?fields=username")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", jira_token))
        .send()?
        .json::<serde_json::Value>()?;
    let user = user.as_object().unwrap();

    let me_key = user.get("key").unwrap().as_str().unwrap();
    let me_username = user
        .get("self")
        .unwrap()
        .as_str()
        .unwrap()
        .split("username=")
        .nth(1)
        .unwrap();

    let search_url = format!(
        r#"https://jira.internetbrands.com/rest/api/2/search?fields=worklog,maxResults=1000&jql=worklogDate >= "{}" and worklogDate < "{}" and (worklogAuthor in ("{}"))&startAt=0"#,
        start_time.date().format("%Y-%m-%d"),
        end_time.date().format("%Y-%m-%d"),
        me_username,
    );

    let mut total_time_spent = Duration::zero();

    let response = client
        .get(search_url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", jira_token))
        .send()?
        .json::<serde_json::Value>()?;

    let issues = response
        .as_object()
        .context("Invalid json")?
        .get("issues")
        .context("No issues key")?
        .as_array()
        .context("Not an array")?;

    let mut worklog_ids = vec![];

    for issue in issues {
        let issue = issue.as_object().unwrap();
        let worklogs = issue
            .get("fields")
            .unwrap()
            .as_object()
            .unwrap()
            .get("worklog")
            .unwrap()
            .as_object()
            .unwrap()
            .get("worklogs")
            .unwrap()
            .as_array()
            .unwrap();

        for worklog in worklogs {
            let worklog = worklog.as_object().unwrap();
            let worklog_author_key = worklog
                .get("author")
                .unwrap()
                .as_object()
                .unwrap()
                .get("key")
                .unwrap()
                .as_str()
                .unwrap();

            if worklog_author_key != me_key {
                continue;
            }

            total_time_spent =
                total_time_spent + Duration::seconds(worklog.get("timeSpentSeconds").unwrap().as_i64().unwrap());

            let worklog_id = worklog.get("id").unwrap().as_str().unwrap();
            let worklog_issue_id = worklog.get("issueId").unwrap().as_str().unwrap();

            worklog_ids.push((worklog_id, worklog_issue_id));
        }
    }

    println!("Total time spent: {}", total_time_spent.num_hours());

    let pb = pretty_print::get_progress_bar(worklog_ids.len() as u64);

    for (worklog_id, issue_id) in worklog_ids {
        pb.inc(1);
        let response = client
            .delete(format!(
                "https://jira.internetbrands.com/rest/api/2/issue/{}/worklog/{}",
                issue_id, worklog_id
            ))
            .header("Authorization", format!("Bearer {}", jira_token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .send()?;

        if !response.status().is_success() {
            println!("Failed to delete worklog: {}", response.text()?);
        }
    }

    return Ok(());
}

fn format_date(date: &DateTime<Local>) -> String {
    let started = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let plus_idx = started.find('+').unwrap();

    let first_part = &started[0..plus_idx];
    let second_part = started[plus_idx..started.len()].replace(':', "");

    return format!("{}{}", first_part, second_part);
}
