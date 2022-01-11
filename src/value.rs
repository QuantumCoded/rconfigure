use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The values in settings and profiles, used for templating.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Script { script: String, value: Box<ScriptValue> },
    ProfileValue { #[serde(rename = "use")] using: String },
    MultiLineString(Vec<String>),
}

/// The values passed into rhai scripts, used for processing, can also be a `Value`.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ScriptValue {
    Array(Vec<ScriptValue>),
    Value(Value),
    Map(HashMap<String, ScriptValue>),
}
