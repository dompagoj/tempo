use colored::Colorize;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::path::PathBuf;

fn get_app_dir() -> PathBuf {
    let home_dir_path = std::env::var("HOME").unwrap();

    PathBuf::from(home_dir_path).join(".tempo")
}

pub trait DirtyTracker {
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self);
}

pub trait OnDataInit {
    fn on_init(&mut self);
}

#[derive(Debug)]
pub struct DataWrapper<T> {
    path: PathBuf,
    inner: Option<T>,
}

impl<T: Serialize + DeserializeOwned + Default + DirtyTracker + Debug + OnDataInit> std::ops::Deref for DataWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target { self.inner() }
}

impl<T: Serialize + DeserializeOwned + Default + DirtyTracker + Debug + OnDataInit> DataWrapper<T> {
    pub fn new<TPath: Into<PathBuf>>(p: TPath) -> Self {
        Self {
            path: get_app_dir().join(p.into()),
            inner: None,
        }
    }

    pub fn inner(&self) -> &T { self.inner.as_ref().unwrap() }

    pub fn inner_mut(&mut self) -> &mut T {
        let inner = self.inner.as_mut().unwrap();
        inner.set_dirty();

        return inner;
    }

    pub fn initialize(&mut self) -> &T {
        if let Some(ref inner) = self.inner {
            return inner;
        }

        if !self.path.exists() {
            self.inner = Some(T::default());
            self.inner.as_mut().unwrap().on_init();
            return self.inner();
        }

        let file_content = std::fs::read(self.path.clone()).expect("Couldnt read user data file");

        if file_content.is_empty() {
            self.inner = Some(T::default());
            self.inner.as_mut().unwrap().on_init();
            return self.inner();
        }

        match bincode::deserialize::<T>(file_content.as_slice()) {
            Ok(val) => {
                self.inner = Some(val);
            }
            Err(err) => {
                println!("{} \n {err}", "Failed to parse the config file".bright_red(),);

                let msg = format!("Delete {} and configure again?", self.path.to_str().unwrap_or(""),);
                let res = inquire::Confirm::new(&msg).prompt().unwrap_or(false);

                if !res {
                    println!("{}", "Aborted".bright_green());
                    std::process::exit(0);
                }

                std::fs::remove_file(self.path.clone()).expect("Failed to delete user config file");

                self.inner = Some(T::default());
            }
        }
        self.inner.as_mut().unwrap().on_init();

        return self.inner();
    }

    pub fn save(&self) {
        if self.inner.is_none() || !self.inner().is_dirty() {
            return;
        }
        let data = bincode::serialize(self.inner.as_ref().unwrap()).expect("Failed to serialize user data");

        std::fs::write(self.path.clone(), data.as_slice()).expect("Couldnt save user data file");
    }

    pub fn delete(&mut self) { std::fs::remove_file(self.path.clone()).expect("Failed to delete file"); }
}
