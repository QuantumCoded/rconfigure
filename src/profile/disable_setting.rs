use crate::errors::CrateError;
use crate::profile::{Profile, ProfileDeserialized, ProfileTable};
use crate::{errors::ProfileError, setting};
use dirs::config_dir;
use std::{fs, path::Path};

impl Profile {
    pub fn disable_setting<P: AsRef<Path>>(&mut self, path: P) -> Result<(), CrateError> {
        let setting = setting::parse(&path);
        self.settings.retain(|s| s.path() != setting.path());

        let path = if path.as_ref().is_absolute() {
            path.as_ref().to_owned()
        } else {
            config_dir()
                .ok_or_else(|| CrateError::NoConfigDir)?
                .join("rconfigure/profiles")
                .join(path)
        };

        // FIXME: maybe add a specific thing for file not found
        let s = fs::read_to_string(&self.path)
            .map_err(|err| ProfileError::ErrorReadingProfile(self.path.to_owned(), err))?;

        let mut profile: ProfileDeserialized =
            toml::from_str(s.as_str())?; // .map_err(|err| -> ProfileError { err.into() })?;

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

        let contents = toml::to_string(&profile)?; // .map_err(|err| -> ProfileError { err.into() })?;

        fs::write(&self.path, contents)
            .map_err(|err| ProfileError::ErrorWritingProfile(self.path.to_owned(), err))?;

        Ok(())
    }
}
