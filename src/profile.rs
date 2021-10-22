use crate::hook::Hook;
use crate::script::{self, ScriptValue};
use crate::setting::{self, Setting, TargetValue};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
struct ProfileDeserialized {
    #[serde(rename = "profile")]
    profile_table: ProfileTable,
}

#[derive(Deserialize)]
struct ProfileTable {
    name: Option<String>,
    settings: Option<Vec<String>>,
    hooks: Option<Vec<String>>,
}

pub struct Profile {
    name: String,
    settings: Vec<Setting>,
    hooks: Vec<Hook>,
}

impl Profile {
    /// Checks for settings with conflicting targets and returns a tuple of the two conflicting
    /// setting files and their conflicting target
    fn setting_conflict(
        &self,
        new_setting: Option<Setting>,
    ) -> Option<(Setting, Setting, PathBuf)> {
        let mut targets = HashMap::new();

        // check for conflicts in profile settings
        for setting in self.settings.iter() {
            for target in setting.targets() {
                if !targets.contains_key(&target) {
                    targets.insert(target, setting);
                } else {
                    return Some((
                        (*targets.get(&target).unwrap()).to_owned(),
                        setting.to_owned(),
                        target,
                    ));
                }
            }
        }

        // check for conflicts between the settings of the profile and a new setting
        if let Some(setting) = new_setting {
            for target in setting.targets() {
                if !targets.contains_key(&target) {
                    targets.insert(target, &setting);
                } else {
                    return Some(((*targets.get(&target).unwrap()).to_owned(), setting, target));
                }
            }
        }

        None
    }

    pub fn apply(&self, engine: &mut rhai::Engine) {
        // check for setting conflicts
        if let Some((setting1, setting2, target)) = self.setting_conflict(None) {
            println!("failed to apply profile, found setting conflict!");
            println!(
                "settings {:?} and {:?} both set values for target {:?}",
                setting1.name(),
                setting2.name(),
                target
            );
            std::process::exit(1);
        }

        // go through each setting and apply it
        for setting in self.settings.iter() {
            // go through each target for the current setting
            for target in setting.targets() {
                let mut map = HashMap::new();

                // populate the string map to template with using target values
                for (k, v) in setting.compose_map(&target) {
                    match v {
                        TargetValue::Boolean(b) => {
                            map.insert(k, b.to_string());
                        }

                        TargetValue::Integer(i) => {
                            map.insert(k, i.to_string());
                        }

                        TargetValue::Float(f) => {
                            map.insert(k, f.to_string());
                        }

                        TargetValue::String(s) => {
                            map.insert(k, s);
                        }

                        TargetValue::Script { script, value } => {
                            // FIXME: use dirs crate
                            let path = PathBuf::from(script);
                            let path = if path.is_absolute() {
                                path
                            } else {
                                // FIXME: use dirs crate
                                PathBuf::from("/home/jeff/.config/rconfigure/scripts").join(path)
                            };

                            let returned_values = script::eval_rhai(path, value, setting, engine);

                            for (k, v) in returned_values {
                                match v {
                                    ScriptValue::Boolean(b) => {
                                        map.insert(k, b.to_string());
                                    }
            
                                    ScriptValue::Integer(i) => {
                                        map.insert(k, i.to_string());
                                    }
            
                                    ScriptValue::Float(f) => {
                                        map.insert(k, f.to_string());
                                    }
            
                                    ScriptValue::String(s) => {
                                        map.insert(k, s);
                                    }

                                    ScriptValue::Map(_) => {
                                        // TODO: flatten recursive script values into dot notation
                                    }
                                }
                            }
                        }
                    }
                }

                // FIXME: cache and make sure all config files get generated successfully before applying any
                match crate::template::generate_config(&target, map) {
                    Ok((path, contents)) => {
                        // FIXME: handle errors for file not found
                        fs::write(path, contents).unwrap();
                    }

                    Err(e) => {
                        println!(
                            "failed to template target {:?} with setting {:?}",
                            target,
                            setting.name()
                        );
                        println!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

pub fn parse<P: AsRef<Path>>(path: P) -> Profile {
    // FIXME: handle errors for file not found
    let s = fs::read_to_string(path.as_ref()).unwrap();
    let profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path.as_ref());
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let mut settings = Vec::new();
    let mut hooks = Vec::new();

    // parse all of the settings and add them to the vector
    if let Some(profile_settings) = profile.profile_table.settings {
        for setting in profile_settings {
            let path = PathBuf::from(setting);

            if path.is_absolute() {
                settings.push(setting::parse(path));
            } else {
                // FIXME: use dirs crate
                let path = PathBuf::from("/home/jeff/.config/rconfigure/settings").join(path);
                settings.push(setting::parse(path));
            }
        }
    }

    // TODO: pase all of the hooks and add them to the vector

    Profile {
        name: profile.profile_table.name.unwrap_or(
            path.as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ),
        settings,
        hooks,
    }
}
