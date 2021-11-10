mod apply;
mod compose_map;

use crate::hook::Hook;
use crate::script::ScriptValue;
use dirs::config_dir;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Clone, Debug)]
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
    targets: HashMap<PathBuf, HashMap<String, TargetValue>>,
}

#[derive(Deserialize)]
struct SettingTable {
    name: Option<String>,
    #[serde(default)]
    hooks: Vec<Hook>,
}

// FIXME: Setting can derive `Deserialize`, can we refactor SettingDeserialized out?
#[derive(Clone, Debug)]
pub struct Setting {
    name: String,
    hooks: Vec<Hook>,
    path: PathBuf,
    global_target: Option<HashMap<String, TargetValue>>,
    targets: Vec<(PathBuf, HashMap<String, TargetValue>)>,
}

impl Setting {
    /// Get the paths for all the targeted templates
    pub fn targets(&self) -> Vec<PathBuf> {
        self.targets
            .iter()
            .map(|(path, ..)| path.to_owned())
            .collect()
    }

    /// Get the path of the setting file
    pub fn path(&self) -> PathBuf {
        self.path.to_owned()
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
    let path = if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        // FIXME: better error handling
        config_dir()
            .expect("config dir borked")
            .join("rconfigure/settings")
            .join(path)
    };

    // FIXME: handle errors for file not found
    let s = fs::read_to_string(&path).unwrap();
    let setting: SettingDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path);
            println!("{}", e);
            std::process::exit(1);
        }
    };

    Setting {
        name: match setting.setting_table {
            Some(SettingTable {
                name: Some(ref name),
                ..
            }) => name.clone(),
            _ => path.file_name().unwrap().to_str().unwrap().to_string(),
        },
        hooks: match setting.setting_table {
            Some(SettingTable { hooks, .. }) => hooks,
            _ => Vec::new(),
        },
        path,
        global_target: setting.global_target,
        targets: setting
            .targets
            .into_iter()
            .map(|(path, table)| {
                if path.is_absolute() {
                    (path, table)
                } else {
                    // FIXME: better error handling
                    let path = config_dir()
                        .expect("config dir borked")
                        .join("rconfigure/templates")
                        .join(path);

                    (path, table)
                }
            })
            .collect::<Vec<_>>(),
    }
}
