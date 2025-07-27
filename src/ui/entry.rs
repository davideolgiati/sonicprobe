use crate::ui::audio::format_db;

pub struct Entry {
        value: String
}

impl Entry {
        pub fn new(value: &str) -> Entry {
                Entry { 
                        value : value.to_string()
                }
        }

        pub fn value(self) -> String {
                self.value
        }

        pub fn from_db(value: f32) -> Entry {
                Entry {
                        value: format_db(value)
                }
        }
}