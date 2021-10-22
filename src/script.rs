use crate::setting::Setting;
use rhai::{Engine, EvalAltResult, Scope, serde::to_dynamic, serde::from_dynamic};
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ScriptValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Map(HashMap<String, ScriptValue>),
}

impl ScriptValue {
    // fn flatten(self) -> HashMap<String, String> {}
}

pub fn eval_rhai(
    path: PathBuf,
    value: ScriptValue,
    setting: &Setting,
    engine: &mut Engine,
) -> HashMap<String, ScriptValue> {
    let mut scope = Scope::new();

    scope.push_constant("value", to_dynamic(value).unwrap());

    match engine.eval_file_with_scope::<rhai::Map>(&mut scope, path) {
        Ok(btree) => {
            let mut map = HashMap::new();

            for (k, v) in btree {
                let v: ScriptValue = from_dynamic(&v).unwrap();
                map.insert(k.to_string(), v);
            }

            // println!("{:#?}", ScriptValue::Map(map));

            return map
        },

        Err(e) => {
            println!("{}", e);
        }
    };

    HashMap::new()
}
