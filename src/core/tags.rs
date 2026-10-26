use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TagStore {
    pub tags: HashMap<String, Vec<String>>,
}

impl TagStore {
    pub fn new() -> Self {
        Self { tags: HashMap::new() }
    }
}
