use crate::setting::Setting;
use rhai::{serde::from_dynamic, serde::to_dynamic, Engine, EvalAltResult, Scope};
use serde::{Deserialize, Serialize};
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

    /*
        value = {
            map: {
                a: 1,
                b: 2,
            },

            str: "string"
        }

        becomes

        {
            "value.map.a": "1",
            "value.map.b": "2",
            "value.str": "string"
        }
    */
}

pub fn eval_rhai(
    path: PathBuf,
    value: ScriptValue,
    setting: &Setting,
    engine: &mut Engine,
) -> HashMap<String, ScriptValue> {
    let mut scope = Scope::new();

    scope.push_constant("value", to_dynamic(value).unwrap());

    match engine.eval_file_with_scope::<rhai::Map>(&mut scope, path.clone()) {
        Ok(btree) => {
            let mut map = HashMap::new();

            for (k, v) in btree {
                let v: ScriptValue = from_dynamic(&v).unwrap();
                map.insert(k.to_string(), v);
            }

            // println!("{:#?}", ScriptValue::Map(map));

            return map;
        }

        Err(e) => {
            println!("Error in rhai script {:?}", path);
            println!("{}", e);
        }
    };

    HashMap::new()
}
