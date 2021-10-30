use crate::bool_false_as_none;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Deserialize, Serialize, Clone)]
struct StringOrFalseAsNone(#[serde(with = "bool_false_as_none")] Option<String>);

impl Deref for StringOrFalseAsNone {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: make hooks that run on unset
#[derive(Deserialize, Serialize, Clone)]
pub struct Hook {
    cmd: String,
    cwd: Option<PathBuf>,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, StringOrFalseAsNone>,
}

impl Hook {
    pub fn run(&self) -> Option<Output> {
        let mut cmd = Command::new(&self.cmd);

        cmd.args(&self.args)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::inherit());

        if let Some(dir) = &self.cwd {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.env {
            match v.deref() {
                Some(v) => cmd.env(k, v),
                None => cmd.env_remove(k),
            };
        }

        cmd.output().ok()
    }
}
