use crate::script::{self, Flatten, ScriptValue};
use crate::setting::{Setting, TargetValue};
use dirs::config_dir;
use rhai::Engine;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

impl Setting {
    pub fn apply(&self, engine: &Engine) {
        // go through each target for the current setting
        for (target, ..) in &self.targets {
            let mut map = HashMap::new();

            // populate the string map to template with using target values
            for (k, v) in self.compose_map(&target) {
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
                        let path = PathBuf::from(script);
                        let path = if path.is_absolute() {
                            path
                        } else {
                            // FIXME: better error handling
                            config_dir()
                                .expect("config dir borked")
                                .join("rconfigure/script")
                                .join(path)
                        };

                        let returned_values = script::eval_rhai(path, value, engine);

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

                                ScriptValue::Array(a) => map.extend(a.flatten(k).into_iter()),

                                ScriptValue::Map(m) => map.extend(m.flatten(k).into_iter()),
                            }
                        }
                    }
                }
            }

            // FIXME: make a backup of all config files when applying
            // FIXME: cache and make sure all config files get generated successfully before applying any
            match crate::template::generate_config(&target, map) {
                Ok((path, contents)) => {
                    // FIXME: handle errors for file not found
                    fs::write(path, contents).unwrap();
                }

                Err(e) => {
                    println!(
                        "failed to template target {:?} with setting {:?}",
                        target, self.name
                    );
                    println!("{}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
