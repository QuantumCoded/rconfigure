use crate::setting::{Error, Setting};
use crate::{dirs::templates_dir, path::force_absolute};
use std::{collections::HashMap, path::Path};

impl Setting {
    /// Nests setting values
    pub fn compose_template_map<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<HashMap<String, String>, Error> {
        let mut path = force_absolute(path, templates_dir()?);
        let mut map = HashMap::new();

        loop {
            for (target, settings) in self.targets()? {
                if path == target {
                    for (k, v) in settings {
                        if !map.contains_key(&k) {
                            map.insert(k, v.to_string());
                        }
                    }
                }
            }

            if let Some(parent) = path.parent() {
                path = parent.to_owned();
            } else {
                for (k, v) in &self.data.global {
                    if !map.contains_key(k) {
                        map.insert(k.to_owned(), v.to_string());
                    }
                }

                break;
            }
        }

        Ok(map)
    }
}
