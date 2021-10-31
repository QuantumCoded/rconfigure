use crate::hook::Hook;
use crate::setting::{self, Setting};
use dirs::config_dir;
use quiz::confirm;
use rhai::Engine;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize)]
struct ProfileDeserialized {
    #[serde(rename = "profile")]
    profile_table: Option<ProfileTable>,
}

#[derive(Deserialize, Serialize)]
struct ProfileTable {
    name: Option<String>,
    settings: Option<Vec<String>>,
    #[serde(default)]
    hooks: Vec<Hook>,
}

// this is bad
#[derive(Clone)]
pub struct Profile {
    path: PathBuf,
    name: String,
    settings: Vec<Setting>,
    hooks: Vec<Hook>,
}

impl Profile {
    /// Checks for settings with conflicting targets and returns a tuple of the two conflicting
    /// setting files and their conflicting target
    fn setting_conflict<'a>(
        &'a self,
        new_setting: Option<&'a Setting>,
    ) -> Option<(Setting, &'a Setting, PathBuf)> {
        let mut targets = HashMap::new();

        // check for conflicts in profile settings
        for setting in self.settings.iter() {
            for target in setting.targets() {
                if !targets.contains_key(&target) {
                    targets.insert(target, setting);
                } else {
                    return Some(((*targets.get(&target).unwrap()).to_owned(), setting, target));
                }
            }
        }

        // check for conflicts between the settings of the profile and a new setting
        if let Some(setting) = new_setting {
            for target in setting.targets() {
                if !targets.contains_key(&target) {
                    targets.insert(target, &setting);
                } else {
                    // existing setting, enabled setting, conflicting target
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

    pub fn enable_setting<P: AsRef<Path>>(&mut self, path: P, noconfirm: bool) {
        let setting = setting::parse(&path);

        // resolve all setting conflicts
        while let Some((setting1, setting2, target)) = self.setting_conflict(Some(&setting)) {
            if noconfirm {
                // retain everything that is NOT setting1
                self.settings.retain(|s| s.path() != setting1.path());
            } else {
                println!("failed to apply profile, found setting conflict!");

                let prompt = format!(
                    "settings {0:?} and {1:?} both set values for target {2:?}, replace setting {0:?} with {1:?}?",
                    setting1.name(),
                    setting2.name(),
                    target
                );

                if confirm(&prompt) {
                    // retain everything that is NOT setting1
                    self.settings.retain(|s| s.path() != setting1.path());
                } else {
                    std::process::exit(0);
                }
            }
        }

        // after resolving setting conflicts push the setting to be added
        self.settings.push(setting.clone());

        let path = if path.as_ref().is_absolute() {
            path.as_ref().to_owned()
        } else {
            // FIXME: better error handling
            config_dir()
                .expect("config dir borked")
                .join("rconfigure/profiles")
                .join(path)
        };

        // FIXME: handle errors for file not found
        let s = fs::read_to_string(&self.path).unwrap();
        let mut profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
            Ok(value) => value,
            Err(e) => {
                println!("error when parsing setting {:?}", path);
                println!("{}", e);
                std::process::exit(1);
            }
        };

        if let Some(ProfileTable {
            settings: Some(ref mut settings),
            ..
        }) = profile.profile_table
        {
            *settings = self
                .settings
                .iter()
                .map(|s| s.path().to_str().unwrap().to_string())
                .collect();
        }

        // serialize and write to file
        fs::write(
            &self.path,
            toml::to_string(&profile).expect("failed to serialize profile"),
        )
        .unwrap();
    }

    pub fn disable_setting<P: AsRef<Path>>(&mut self, path: P) {
        let setting = setting::parse(&path);
        self.settings.retain(|s| s.path() != setting.path());

        let path = if path.as_ref().is_absolute() {
            path.as_ref().to_owned()
        } else {
            // FIXME: better error handling
            config_dir()
                .expect("config dir borked")
                .join("rconfigure/profiles")
                .join(path)
        };

        // FIXME: handle errors for file not found
        let s = fs::read_to_string(&self.path).unwrap();
        let mut profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
            Ok(value) => value,
            Err(e) => {
                println!("error when parsing setting {:?}", path);
                println!("{}", e);
                std::process::exit(1);
            }
        };

        if let Some(ProfileTable {
            settings: Some(ref mut settings),
            ..
        }) = profile.profile_table
        {
            let mut settings_buf = vec![];

            for (string, contained_setting) in settings
                .iter()
                .map(|string| (string, setting::parse(string)))
            {
                if setting.path() != contained_setting.path() {
                    settings_buf.push(string.to_owned())
                }
            }

            *settings = settings_buf;
        }

        fs::write(
            &self.path,
            toml::to_string(&profile).expect("failed to serialize profile"),
        )
        .unwrap();
    }
}

pub fn parse<P: AsRef<Path>>(path: P) -> Profile {
    let path = if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        // FIXME: better error handling
        config_dir()
            .expect("config dir borked")
            .join("rconfigure/profiles")
            .join(path)
    };

    // FIXME: handle errors for file not found
    let s = fs::read_to_string(&path).unwrap();
    let profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path);
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

            settings_buf.push(setting::parse(path));
        }
    }

    Profile {
        name: profile
            .profile_table
            .as_ref()
            .and_then(|t| t.name.clone())
            .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap().to_string()),
        settings: settings_buf,
        hooks: profile
            .profile_table
            .and_then(|t| Some(t.hooks))
            .unwrap_or_default(),
        path,
    }
}
