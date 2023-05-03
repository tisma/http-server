use serde_json::{
    json,
    Value::{self, Null},
};

use std::{env, fs};

trait DataFile {
    fn get_path(&self) -> String;
}

pub struct File {
    pub file_path: String,
}

impl DataFile for File {
    fn get_path(&self) -> String {
        let args: Vec<String> = env::args().collect();

        let default = String::from(if self.file_path != "" {
            self.file_path.to_string()
        } else {
            "./test_mock_data.json".to_string()
        });

        let mut i = 0;

        let file_path = loop {
            let arg = args.get(i).unwrap_or(&default);

            if &[..3] == "-f=" {
                break arg[3..].to_string();
            }

            if arg == &default {
                break default.to_string();
            }

            i += 1;
        };

        return file_path;
    }
}