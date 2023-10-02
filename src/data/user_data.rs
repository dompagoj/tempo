use super::*;
use colored::Colorize;
use paste::paste;
use serde::{Deserialize, Serialize};

macro_rules! add_getters {
    (struct $name:ident { $($v:vis $fname:ident : Option<$ftype:ty>),* $(,)? }) => {

        #[derive(Deserialize, Serialize, Debug, Default)]
        pub struct $name {
            $($v $fname : Option<$ftype>),*,
            #[serde(skip_serializing,skip_deserializing)]
            pub is_dirty: bool,
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
                        self.is_dirty = true;
                        self.$fname = Some(val);
                    }
                }
            )+
        }


    }
}

add_getters! {
struct UserData {
    jira_token: Option<String>,
    pub user_aliases: Option<Vec<String>>,
}
}

impl OnDataInit for UserData
{
    fn on_init(&mut self)
    {
        if self.user_aliases.is_none() {
            self.user_aliases = Some(vec![]);
        }
    }
}
impl DirtyTracker for UserData
{
    fn is_dirty(&self) -> bool
    {
        self.is_dirty
    }

    fn set_dirty(&mut self)
    {
        self.is_dirty = true
    }
}
