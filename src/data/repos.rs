use crate::data::{DirtyTracker, OnDataInit};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Repos
{
    is_dirty: bool,
    pub list: Vec<PathBuf>,
}

impl DirtyTracker for Repos
{
    fn is_dirty(&self) -> bool
    {
        self.is_dirty
    }
    fn set_dirty(&mut self)
    {
        self.is_dirty = true;
    }
}

impl OnDataInit for Repos
{
    fn on_init(&mut self) {}
}
