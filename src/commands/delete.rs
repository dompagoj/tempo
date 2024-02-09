use crate::{jira, time};

use super::*;

#[derive(Args, Debug)]
#[command(about = "Delete all time entries from specified month")]
pub struct DeleteCommand {}

pub fn command(config: Cfg, _args: DeleteCommand) -> anyhow::Result<()> {
    let (start_date, end_date) = time::get_date_range_from_user()?;

    println!("Deleting isues...");
    jira::delete_worklogs(config, start_date, end_date)?;
    print!("Done");

    return Ok(());
}
