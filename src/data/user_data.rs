use super::*;
use colored::Colorize;
use paste::paste;

macro_rules! add_getters {
    (struct $name:ident { $($v:vis $fname:ident : Option<$ftype:ty>),* $(,)? }) => {

        #[derive(Archive, Deserialize, Serialize, Debug, Default)]
        #[archive_attr(derive(CheckBytes, Debug))]
        pub struct $name {
            $($v $fname : Option<$ftype>),*
        }

        impl $name {
            $(
                paste! {
                    pub fn [<get_ $fname>](&self) -> &$ftype{
                        match self.$fname {
                            Some(ref itm) => itm,
                            None => {
                                let name = stringify!($fname);
                                println!("{} {} {} {} --{}=insert_value_here", name,"Not found, you can configure it by doing".red(), "tempo".bright_green(), "configure".green(), name);
                                std::process::exit(1);
                            }
                        }
                    }

                     pub fn [<set_ $fname>](&mut self, val: $ftype) {
                        self.$fname = Some(val);
                    }
                }
            )+
        }
    }
}

add_getters! {
struct UserDataInner {
    pub name: Option<String>,
    pub tempo_api_key: Option<String>,
    pub test123: Option<String>,
}
}

#[derive(Debug)]
pub struct UserData {
    inner: Option<UserDataInner>,
    file: Option<File>,
    is_dirty: bool,
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
            inner: None,
            is_dirty: false,
            file: None,
        }
    }

    pub fn inner(&self) -> &UserDataInner {
        self.inner.as_ref().unwrap()
    }

    pub fn inner_mut(&mut self) -> &mut UserDataInner {
        self.is_dirty = true;
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
        self.file = Some(user_data_file);

        if file_content.is_empty() {
            self.inner = Some(UserDataInner::default());
            return self.inner_mut();
        }

        self.inner = Some(
            rkyv::check_archived_root::<UserDataInner>(file_content.as_slice())
                .unwrap()
                .deserialize(&mut rkyv::Infallible)
                .unwrap(),
        );

        self.inner_mut()
    }

    #[inline(always)]
    pub fn save(&self) {
        if !self.is_dirty || self.file.is_none() {
            return;
        }
        let data = rkyv::to_bytes::<_, 0>(&self.inner).unwrap();
        self.file.as_ref().unwrap().write_all(data.as_slice()).unwrap();
    }
}

pub fn get_user_data_path() -> PathBuf {
    let home_dir_path = std::env::var("HOME").unwrap();

    PathBuf::from(home_dir_path)
        .join(".tempo")
        .join("user_data")
}
