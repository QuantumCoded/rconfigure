use crate::setting::{Setting, TargetValue};
use dirs::config_dir;
use std::{collections::HashMap, path::Path};

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
}
