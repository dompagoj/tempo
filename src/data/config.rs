use super::*;

#[derive(Archive, Deserialize, Serialize, Debug, Default, Clone)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct TimeEntry {
    pub id: String,
    pub ticket_name: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

#[derive(Archive, Deserialize, Serialize, Debug, Default)]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct ConfigFileInner {
    pub entries: Vec<TimeEntry>,
    pub current: Vec<TimeEntry>,
}

#[derive(Debug)]
pub struct ConfigFile {
    config: ConfigFileInner,
    pub user_data: UserData,
    file: File,
    path: PathBuf,
    pub is_dirty: bool,
}

impl ConfigFile {
    pub fn get_new() -> Self {
        let path = get_config_file_path();
        let (config, config_file) = get_config_file(&path);
        let user_data = UserData::default();

        ConfigFile {
            is_dirty: false,
            config,
            user_data,
            file: config_file,
            path,
        }
    }

    pub fn save(&mut self) {
        if !self.is_dirty {
            return;
        }

        let data = rkyv::to_bytes::<_, 0>(&self.config).unwrap();
        self.file.write_all(data.as_slice()).unwrap();
    }

    pub fn delete_file(&self) -> Result<(), std::io::Error> {
        std::fs::remove_file(self.path.clone())
    }

    pub fn resume_entry(&mut self, idx: usize) {
        let mut entry = self.entries.remove(idx);
        entry.end_time = None;
        self.current.push(entry);
    }

    pub fn end_active_entry(&mut self, idx: usize) {
        let removed = self.current.remove(idx);
        self.add_done_entry(removed);
    }

    pub fn end_active_entries(&mut self, cb: Option<fn(&TimeEntry)>) {
        for _ in 0..self.current.len() {
            let removed = self.current.remove(0);
            if let Some(cb) = cb {
                cb(&removed);
            }
            self.add_done_entry(removed);
        }
    }

    pub fn add_done_entry(&mut self, mut entry: TimeEntry) {
        entry.end_time = Some(chrono::Local::now().to_string());
        self.entries.push(entry);
    }

    pub fn inner(&self) -> &ConfigFileInner {
        &self.config
    }

    pub fn inner_mut(&mut self) -> &mut ConfigFileInner {
        self.is_dirty = true;
        &mut self.config
    }
}

impl Deref for ConfigFile {
    type Target = ConfigFileInner;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl DerefMut for ConfigFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

fn get_config_file(path: &PathBuf) -> (ConfigFileInner, std::fs::File) {
    let config_exists = path.exists();

    let mut config_file = if config_exists {
        get_file(&path)
    } else {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        get_file(&path)
    };

    let file_content = read_file_to_vec(&mut config_file);

    let inner_config = if !file_content.is_empty() {
        rkyv::check_archived_root::<ConfigFileInner>(file_content.as_slice())
            .unwrap()
            .deserialize(&mut rkyv::Infallible)
            .unwrap()
    } else {
        ConfigFileInner::default()
    };

    (inner_config, config_file)
}

pub fn get_config_file_path() -> PathBuf {
    let home_dir_path = std::env::var("HOME").unwrap();

    PathBuf::from(home_dir_path).join(".tempo").join("config")
}
