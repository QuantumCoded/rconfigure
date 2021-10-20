use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};
use toml::value::Table;

#[derive(Deserialize, Debug)]
pub struct Setting {
    setting_table: Option<Table>,
    global_target: Option<Table>,
    targets: Vec<(PathBuf, Table)>,
}

/// Parses a setting into its struct representation
pub fn parse<P: AsRef<Path>>(path: P) -> Setting {
    // FIXME: handle errors for file not found
    let s = fs::read_to_string(path).unwrap();
    let setting: HashMap<String, Table> = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let mut setting_table: Option<Table> = None;
    let mut global_target: Option<Table> = None;

    let mut targets: Vec<(PathBuf, Table)> = Vec::new();

    // sort the setting blocks based on path type
    for (target, table) in setting {
        match target.as_str() {
            "setting" => setting_table = Some(table),
            "global" => global_target = Some(table),
            target => {
                let path = Path::new(target);

                if path.is_absolute() {
                    targets.push((path.to_owned(), table));
                } else {
                    // FIXME: use dirs crate
                    let path = PathBuf::from("./config/templates").join(path);
                    targets.push((path, table));
                }
            }
        }
    }

    Setting {
        setting_table,
        global_target,
        targets,
    }
}
