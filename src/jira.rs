use crate::data::ConfigFile;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddWorkLogPayload {
    started: String,
    time_spent_seconds: i64,
}

pub fn authenticate(config: &ConfigFile) {
    let jira_token = config.user_data.inner().get_jira_token();

    let client = reqwest::blocking::Client::new();

    let json = json!({
      "comment": "Automatic worklog by Tempo (cli tool by Domagoj)",
      "started": "2021-01-18T12:35:00.000+0000",
      "timeSpentSeconds": 120,
    })
    .to_string();

    let res = client
        .post(format!(
            "https://jira.internetbrands.com/rest/api/2/issue/{}/worklog?adjustEstimate=leave",
            "TCCN-2522"
        ))
        .body(json)
        .header("Authorization", format!("Bearer {}", jira_token))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .send()
        .unwrap();

    let res = res.text().unwrap();

    dbg!(&res);
}
