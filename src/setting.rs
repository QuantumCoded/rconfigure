use crate::{hook::Hook, script::ScriptValue};
use dirs::config_dir;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum TargetValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Script { script: String, value: ScriptValue },
}

#[derive(Deserialize)]
struct SettingDeserialized {
    #[serde(rename = "setting")]
    setting_table: Option<SettingTable>,
    #[serde(rename = "global")]
    global_target: Option<HashMap<String, TargetValue>>,
    #[serde(flatten)]
    targets: HashMap<String, HashMap<String, TargetValue>>,
}

#[derive(Deserialize)]
struct SettingTable {
    name: Option<String>,
    #[serde(default)]
    hooks: Vec<Hook>,
}

#[derive(Clone)]
pub struct Setting {
    name: String,
    hooks: Vec<Hook>,
    path: PathBuf,
    global_target: Option<HashMap<String, TargetValue>>,
    targets: Vec<(PathBuf, HashMap<String, TargetValue>)>,
}

impl Setting {
    /// Composes a map from all of the setting targets for a given path
    pub fn compose_map<P: AsRef<Path>>(&self, path: P) -> HashMap<String, TargetValue> {
        let mut path = if path.as_ref().is_absolute() {
            path.as_ref().to_owned()
        } else {
            // FIXME: better error handling
            config_dir()
                .expect("config dir borked")
                .join("rconfigure/templates")
                .join(path)
        };

        let mut map: HashMap<String, TargetValue> = HashMap::new();

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

    /// Get the paths for all the targeted templates
    pub fn targets(&self) -> Vec<PathBuf> {
        self.targets
            .iter()
            .map(|(path, ..)| path.to_owned())
            .collect()
    }

    /// Get the hooks
    pub fn hooks(&self) -> &Vec<Hook> {
        &self.hooks
    }

    /// Get the name of the setting
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
}

/// Parses a setting into its struct representation
pub fn parse<P: AsRef<Path>>(path: P) -> Setting {
    // FIXME: handle errors for file not found
    let s = fs::read_to_string(path.as_ref()).unwrap();
    let setting: SettingDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path.as_ref());
            println!("{}", e);
            std::process::exit(1);
        }
    };

    // FIXME: switch to deserializing to pathbufs instead of strings everywhere for settings
    let mut targets: Vec<(PathBuf, HashMap<String, TargetValue>)> = Vec::new();

    // expand paths for all targets
    for (target, table) in setting.targets {
        let path = PathBuf::from(target);

        if path.is_absolute() {
            targets.push((path.to_owned(), table));
        } else {
            // FIXME: better error handling
            let path = config_dir()
                .expect("config dir borked")
                .join("rconfigure/templates")
                .join(path);
            targets.push((path.to_owned(), table));
        }
    }

    Setting {
        name: match setting.setting_table {
            Some(SettingTable {
                name: Some(ref name),
                ..
            }) => name.clone(),
            _ => path
                .as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        },
        hooks: match setting.setting_table {
            Some(SettingTable { hooks, .. }) => hooks,
            _ => Vec::new(),
        },
        path: path.as_ref().to_owned(),
        global_target: setting.global_target,
        targets,
    }
}
