use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};
use toml::value::{Table, Value};

#[derive(Deserialize, Debug)]
pub struct Setting {
    setting_table: Option<Table>,
    global_target: Option<Table>,
    targets: Vec<(PathBuf, Table)>,
}

impl Setting {
    /// Composes a map from all of the setting targets for a given path
    fn compose_map<P: AsRef<Path>>(&self, path: P) -> HashMap<String, Value> {
        let mut path = if path.as_ref().is_absolute() {
            path.as_ref().to_owned()
        } else {
            // FIXME: use dirs crate
            Path::new("./config/templates").join(path)
        };

        let mut map: HashMap<String, Value> = HashMap::new();

        loop {
            // check each of the target paths
            for (target_path, table) in self.targets.iter() {
                // if the current path is a target add all its new entries
                if path == *target_path {
                    for (k, v) in table {
                        if !map.contains_key(k) {
                            map.insert(k.to_owned(), v.to_owned());
                        }
                    }
                }
            }

            if let Some(parent) = path.parent() {
                path = parent.to_owned();
            } else {
                // if there is a global target append its entries
                if let Some(table) = &self.global_target {
                    for (k, v) in table {
                        if !map.contains_key(k) {
                            map.insert(k.to_owned(), v.to_owned());
                        }
                    }
                }

                break;
            }
        }

        map
    }
}

/// Parses a setting into its struct representation
pub fn parse<P: AsRef<Path>>(path: P) -> Setting {
    // FIXME: handle errors for file not found
    let s = fs::read_to_string(path.as_ref()).unwrap();
    let setting: HashMap<String, Table> = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path.as_ref());
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
