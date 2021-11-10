use crate::script::{Flatten, ScriptValue};
use std::collections::HashMap;

fn raw_flatten<I: Iterator<Item = (String, ScriptValue)>>(
    name: String,
    iter: I,
) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (i, value) in iter {
        let name = format!("{}.{}", name, i);

        match value {
            ScriptValue::Boolean(value) => {
                map.insert(name, value.to_string());
            }
            ScriptValue::Integer(value) => {
                map.insert(name, value.to_string());
            }
            ScriptValue::Float(value) => {
                map.insert(name, value.to_string());
            }
            ScriptValue::String(value) => {
                map.insert(name, value.to_string());
            }
            ScriptValue::Array(value) => {
                map.extend(value.flatten(name));
            }
            ScriptValue::Map(value) => {
                map.extend(value.flatten(name));
            }
        }
    }

    map
}

impl Flatten for Vec<ScriptValue> {
    fn flatten(self, name: String) -> HashMap<String, String> {
        raw_flatten(
            name,
            self.into_iter()
                .enumerate()
                .map(|(i, v)| (i.to_string(), v)),
        )
    }
}

impl Flatten for HashMap<String, ScriptValue> {
    fn flatten(self, name: String) -> HashMap<String, String> {
        raw_flatten(name, self.into_iter())
    }
}
