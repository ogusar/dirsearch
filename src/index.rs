use std::error::Error;
use std::fs::create_dir_all;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::str::{from_utf8, Utf8Error};
use lmdb::{Environment, Database, WriteFlags, Transaction, Cursor, Stat};
use serde::{Deserialize, Serialize};
use crate::source::Source;

pub struct Index {
    source_db: Database,
    index_db: Database,
    env: Environment
}

#[derive(Serialize, Deserialize)]
pub struct RevIndexRecord {
    pub freq: usize,
    pub sources: Vec<(u64, usize)>
}

impl RevIndexRecord {
    pub fn new(freq: usize, sources: Vec<(u64, usize)>) -> Self {
        Self { freq, sources }
    }
}

impl Index {
    pub fn new(db_loc: &String, exists: &mut bool) -> Result<Index, Box<dyn Error>> {
        let path = Path::new(db_loc);
        if !path.exists() {
            create_dir_all(path)?;
            *exists = false;
        }
        else {
            *exists = true;
        }
        let env = Environment::new()
            .set_max_dbs(2)
            .set_map_size(1048576000)
            .open(path)?;

        let source_db = env.create_db(Some("source_db"), lmdb::DatabaseFlags::empty())
            .expect("Failed to create 'source_db'");
        let index_db = env.create_db(Some("index_db"), lmdb::DatabaseFlags::empty())
            .expect("Failed to create 'index_db'");

        Ok(Self {source_db, index_db, env})
    }

    pub fn add_source(&self, source: &Source) {
        let mut txn = self.env.begin_rw_txn().expect("Failed to open a transaction");
        txn.put(self.source_db, &source.get_hash().to_be_bytes(), &bincode::serialize(source).expect("Failed to serialize source"), WriteFlags::empty())
            .expect("Failed to insert a source into source_db");
        txn.commit()
            .expect("Failed to commit transaction");
    }

    pub fn get_source(&self, key: &u64) -> Option<Source> {
        let txn = self.env.begin_ro_txn().expect("Failed to open a transaction");
        match txn.get(self.source_db, &key.to_be_bytes()) {
            Ok(bytes) => { Some(bincode::deserialize(bytes).expect("Failed to deserialize source")) }
            Err(_) => { None }
        }
    }

    pub fn add_rev_index(&self, word: &String, record: &RevIndexRecord) {
        let mut txn = self.env.begin_rw_txn().expect("Failed to open a transaction");
        txn.put(self.index_db, word, &bincode::serialize(record).expect("Failed to serialize source"), WriteFlags::empty())
            .expect("Failed to insert a source into index_db");
        txn.commit()
            .expect("Failed to commit transaction");
    }

    pub fn get_rev_index(&self, word: &String) -> Option<RevIndexRecord> {
        let txn = self.env.begin_ro_txn().expect("Failed to open a transaction");
        match txn.get(self.index_db, word) {
            Ok(bytes) => { Some(bincode::deserialize(bytes).expect("Failed to deserialize source")) }
            Err(_) => { None }
        }
    }
}