use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// The values in settings and profiles, used for templating.
#[derive(Deserialize, Serialize, Debug, Clone)]
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
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ScriptValue {
    Array(Vec<ScriptValue>),
    Value(Value),
    Map(HashMap<String, ScriptValue>),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
            Value::MultiLineString(v) => write!(f, "{}", v.join("\n")),
            Value::ProfileValue { using: _v } => todo!(),
            Value::Script { script: _s, value: _v } => todo!(),
            Value::String(v) => write!(f, "{}", v),
        }
    }
}