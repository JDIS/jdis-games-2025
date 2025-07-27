use std::ops::Deref;

use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PlayerId(String);

impl PlayerId {
    pub fn new() -> Self {
        PlayerId(nanoid!())
    }
}

impl Deref for PlayerId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for PlayerId {
    fn from(value: String) -> Self {
        PlayerId(value)
    }
}
