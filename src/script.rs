use crate::setting::Setting;
use rhai::{serde::from_dynamic, Dynamic, Engine, Scope};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::FromIterator, path::PathBuf, str::FromStr};

pub trait Flatten {
    fn flatten(self, name: String) -> HashMap<String, String>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ScriptValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<ScriptValue>),
    Map(HashMap<String, ScriptValue>),
}

impl ScriptValue {
    fn to_dynamic(self) -> Dynamic {
        match self {
            ScriptValue::Boolean(value) => Dynamic::from_bool(value),
            ScriptValue::Integer(value) => Dynamic::from_int(value),
            ScriptValue::Float(value) => Dynamic::from_float(value),
            ScriptValue::String(value) => Dynamic::from_str(value.as_str()).unwrap(),
            ScriptValue::Array(value) => {
                Dynamic::from_iter(value.into_iter().map(|value| value.to_dynamic()))
            }
            ScriptValue::Map(value) => {
                let mut map = rhai::Map::new();

                for (k, v) in value {
                    map.insert(k.into(), v.to_dynamic());
                }

                Dynamic::from(map)
            }
        }
    }
}

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

pub fn eval_rhai(
    path: PathBuf,
    value: ScriptValue,
    setting: &Setting,
    engine: &Engine,
) -> HashMap<String, ScriptValue> {
    let mut scope = Scope::new();

    scope.push_constant("value", value.to_dynamic());

    match engine.eval_file_with_scope::<rhai::Map>(&mut scope, path.clone()) {
        Ok(btree) => {
            let mut map = HashMap::new();

            for (k, v) in btree {
                // FIXME: change this to not use from_dynamic
                let v: ScriptValue = from_dynamic(&v).unwrap();

                map.insert(k.to_string(), v);
            }

            return map;
        }

        Err(e) => {
            println!("Error in rhai script {:?}", path);
            println!("{}", e);
        }
    };

    HashMap::new()
}
