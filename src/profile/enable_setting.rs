use crate::profile::{Profile, ProfileDeserialized, ProfileTable};
use crate::setting;
use dirs::config_dir;
use std::{fs, path::Path};

impl Profile {
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

                if quiz::confirm(&prompt) {
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
                // FIXME: check if the path is relative to the settings directory and use short names if possible
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
}
