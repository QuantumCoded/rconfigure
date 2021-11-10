use crate::profile::{Profile, ProfileDeserialized, ProfileTable};
use crate::setting;
use dirs::config_dir;
use std::{fs, path::Path};

impl Profile {
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
