use crate::hook::Hook;
use crate::setting::{self, Setting};
use dirs::config_dir;
use rhai::Engine;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
struct ProfileDeserialized {
    #[serde(rename = "profile")]
    profile_table: Option<ProfileTable>,
}

#[derive(Deserialize)]
struct ProfileTable {
    name: Option<String>,
    settings: Option<Vec<String>>,
    #[serde(default)]
    hooks: Vec<Hook>,
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

    pub fn apply(&self, engine: &Engine) {
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
        for setting in &self.settings {
            setting.apply(engine);
        }

        for setting in &self.settings {
            for hook in setting.hooks() {
                hook.run();
            }
        }

        for hook in &self.hooks {
            hook.run();
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

    let mut settings_buf = Vec::new();

    // parse all of the settings and add them to the vector
    if let Some(ProfileTable {
        settings: Some(settings),
        ..
    }) = &profile.profile_table
    {
        for setting in settings {
            let path = PathBuf::from(setting);

            if path.is_absolute() {
                settings_buf.push(setting::parse(path));
            } else {
                // FIXME: better error handling
                let path = config_dir()
                    .expect("config dir borked")
                    .join("rconfigure/settings")
                    .join(path);
                settings_buf.push(setting::parse(path));
            }
        }
    }

    Profile {
        name: profile
            .profile_table
            .as_ref()
            .and_then(|t| t.name.clone())
            .unwrap_or_else(|| {
                path.as_ref()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
        settings: settings_buf,
        hooks: profile
            .profile_table
            .and_then(|t| Some(t.hooks))
            .unwrap_or_default(),
    }
}
