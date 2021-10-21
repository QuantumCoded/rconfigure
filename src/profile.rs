use crate::hook::Hook;
use crate::setting::{self, Setting};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
struct ProfileDeserialized {
    profile: ProfileTable,
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

        // FIXME: remove duplicate code
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

    pub fn apply(&self) {
        // check for setting conflicts
        if let Some((setting1, setting2, target)) = self.setting_conflict(None) {
            println!("failed to apply profile, found setting conflict!");
            println!(
                "settings '{}' and '{}' both set values for target {:?}",
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

                // populate the string map to template with
                for (k, v) in setting.compose_map(&target) {
                    use crate::setting::TargetValue::*;

                    match v {
                        Boolean(b) => {
                            map.insert(k, b.to_string());
                        }

                        Integer(i) => {
                            map.insert(k, i.to_string());
                        }

                        Float(f) => {
                            map.insert(k, f.to_string());
                        }

                        String(s) => {
                            map.insert(k, s);
                        }

                        Script { .. } => {
                            // TODO: run the rhai script to generate values and append them
                        }
                    }
                }

                // FIXME: cache and make sure all config files get generated successfully before applying any
                match crate::template::generate_config(target, map) {
                    Ok((path, contents)) => {
                        // FIXME: handle errors for file not found
                        fs::write(path, contents).unwrap();
                    }

                    Err(e) => {
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
    if let Some(profile_settings) = profile.profile.settings {
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
        name: profile.profile.name.unwrap_or(
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
