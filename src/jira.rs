use crate::data::{ConfigFile, JiraTimeEntry};
use chrono::{DateTime, Local};
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

fn format_date(date: &DateTime<Local>) -> String {
    let started = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let plus_idx = started.find('+').unwrap();

    let first_part = &started[0..plus_idx];
    let second_part = started[plus_idx..started.len()].replace(':', "");

    return format!("{}{}", first_part, second_part);
}
