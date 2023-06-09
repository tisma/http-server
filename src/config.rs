use std::{collections::HashMap, fs};

fn all() -> HashMap<String, String> {
    let configs = fs::read_to_string(".env.example").unwrap();

    let parse: Vec<&str> = configs.split("\n").collect();

    let mut final_configuration: HashMap<String, String> = HashMap::new();

    for conf in parse {
        let split: Vec<&str> = conf.split("=").collect();

        final_configuration.insert(
            String::from(split[0]),
            split[1].replace('\"', "").to_string(),
        );
    }

    return final_configuration;
}

pub fn get(key: &str) -> String {
    let configs = all();

    let empty = String::from("");

    return configs.get(key).unwrap_or(&empty).to_string();
}
