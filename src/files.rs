use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use bincode::Error;
use crate::source::Source;

pub fn get_files(folder_path: &String) -> Box<[String]>{
    let files = fs::read_dir(folder_path).unwrap();
    files
        .filter(|file| !file.is_err())
        .map(|file| file.unwrap().file_name().into_string().unwrap())
        .collect()
}

pub fn save_index(index: &HashMap<String, HashSet<Source>>, file_path: &String) {
    let file = File::create(file_path).expect("Failed to create index file");
    let writer = BufWriter::new(file);
    bincode::serialize_into(writer, index).expect("Failed to save index");
}

pub fn load_index(file_path: &String) -> Result<HashMap<String, HashSet<Source>>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let map: Result<HashMap<String, HashSet<Source>>, Error> = bincode::deserialize_from(reader);
    map
}