use std::error::Error;
use std::fs::create_dir_all;
use std::path::Path;
use lmdb::{Database, Environment, Transaction, WriteFlags};
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

        let source_db = env.create_db(Some("source_db"), lmdb::DatabaseFlags::empty())?;
        let index_db = env.create_db(Some("index_db"), lmdb::DatabaseFlags::empty())?;

        Ok(Self {source_db, index_db, env})
    }

    pub fn add_source(&self, source: &Source) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.begin_rw_txn()?;
        txn.put(self.source_db, &source.get_hash().to_be_bytes(), &bincode::serialize(source)?, WriteFlags::empty())?;
        txn.commit()?;
        Ok(())
    }

    pub fn get_source(&self, key: &u64) -> Result<Source, Box<dyn Error>> {
        let txn = self.env.begin_ro_txn()?;
        match txn.get(self.source_db, &key.to_be_bytes()) {
            Ok(bytes) => { Ok(bincode::deserialize(bytes)?) }
            Err(err) => { Err(err.into()) }
        }
    }

    pub fn add_rev_index(&self, word: &String, record: RevIndexRecord) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.begin_rw_txn()?;
        txn.put(self.index_db, word, &bincode::serialize(&record)?, WriteFlags::empty())?;
        txn.commit()?;
        Ok(())
    }

    pub fn get_rev_index(&self, word: &str) -> Result<Option<RevIndexRecord>, Box<dyn Error>> {
        let txn = self.env.begin_ro_txn()?;
        match txn.get(self.index_db, &word.to_string()) {
            Ok(bytes) => { Ok(Some(bincode::deserialize::<RevIndexRecord>(bytes)?)) }
            Err(_) => { Ok(None) } //Not found error
        }
    }

    pub fn delete(&self) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.begin_rw_txn()?;
        txn.clear_db(self.index_db)?;
        txn.clear_db(self.source_db)?;
        txn.commit()?;
        Ok(())
    }
}