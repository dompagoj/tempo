use super::*;
use crate::data::repos::Repos;
use crate::data::wrapper::DataWrapper;

#[derive(Debug)]
pub struct ConfigFile {
    pub user_data: DataWrapper<UserData>,
    pub repos: DataWrapper<Repos>,
}

impl ConfigFile {
    pub fn new() -> Self {
        let user_data = DataWrapper::new("user_data");
        let repos = DataWrapper::new("repos");

        ConfigFile { user_data, repos }
    }

    pub fn save(&mut self) {
        self.user_data.save();
        self.repos.save();
    }
}
