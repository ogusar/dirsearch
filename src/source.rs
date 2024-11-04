use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Source {
    pub dir: String,
    pub file_name: String,
    pub page_number: usize
}

impl Source {
    pub fn new(dir: String, file_name: String, page_number: usize) -> Self {
        Self { dir, file_name, page_number }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Display for Source {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.pad(&format!("document: {}, page: {}", self.file_name, self.page_number)[..])
    }
}