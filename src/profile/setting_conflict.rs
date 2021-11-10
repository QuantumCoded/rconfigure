use crate::{profile::Profile, setting::Setting};
use std::collections::HashMap;
use std::path::PathBuf;

impl Profile {
    /// Checks for settings with conflicting targets and returns a tuple of the two conflicting
    /// setting files and their conflicting target
    pub fn setting_conflict<'a>(
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
}
