use std::fmt;
use std::hash::Hasher;
use lopdf::Document;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Source {
    dir: String,
    file_name: String,
    page_number: usize
}

impl Source {
    pub fn new(dir: String, file_name: String, page_number: usize) -> Self {
        Self { dir, file_name, page_number }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.pad(&format!("document: {}, page: {}", self.file_name, self.page_number)[..])
    }
}