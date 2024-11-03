use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use bincode::Error;
use crate::source::Source;

pub fn get_pdf_files(folder_path: &String) -> Box<[String]>{
    let files = fs::read_dir(folder_path).unwrap();
    files
        .filter(|file| !file.is_err())
        .map(|file| file.unwrap().file_name().into_string().unwrap())
        .filter(|file| file.ends_with(".pdf"))
        .collect()
}