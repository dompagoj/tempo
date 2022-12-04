use super::*;

#[derive(Archive, Deserialize, Serialize, Debug, Default)]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct UserDataInner {
    name: Option<String>,
    tempo_api_key: Option<String>,
}

#[derive(Default, Debug)]
pub struct UserData {
    inner: Option<UserDataInner>,
}

impl Deref for UserData {
    type Target = UserDataInner;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl DerefMut for UserData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl UserData {
    pub fn new() -> Self {
        Self {
            inner: Some(UserDataInner::default()),
        }
    }

    pub fn inner(&self) -> &UserDataInner {
        self.inner.as_ref().unwrap()
    }

    pub fn inner_mut(&mut self) -> &mut UserDataInner {
        self.inner.as_mut().unwrap()
    }

    pub fn initialize(&mut self) -> &UserDataInner{
        if let Some(ref inner) = self.inner {
            return inner;
        }

        let path = get_user_data_path();

        let mut user_data_file = if path.exists() {
            get_file(&path)
        } else {
            create_dir_all(path.parent().unwrap()).unwrap();
            get_file(&path)
        };

        let file_content = read_file_to_vec(&mut user_data_file);

        if file_content.is_empty() {
            *self = UserData::new();
            return self.inner();
        }

        self.inner = Some(
            rkyv::check_archived_root::<UserDataInner>(file_content.as_slice())
                .unwrap()
                .deserialize(&mut rkyv::Infallible)
                .unwrap(),
        );

        self.inner()
    }
}

pub fn get_user_data_path() -> PathBuf {
    let home_dir_path = std::env::var("HOME").unwrap();

    PathBuf::from(home_dir_path)
        .join(".tempo")
        .join("user_data")
}
