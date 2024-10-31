use std::collections::{HashMap, HashSet};
use lopdf::Document;

use crate::files::get_files;
use crate::source::Source;

pub fn make_index(folder: String) -> HashMap<String, HashSet<Source>> {
    let files = get_files(&folder);
    let mut index: HashMap<String, HashSet<Source>> = HashMap::new();
    for file in files {
        add_file(&folder, &file, &mut index)
    }
    index
}

fn add_file(folder_path: &String, file_path: &String, index: &mut HashMap<String, HashSet<Source>>) {
    match Document::load(folder_path.to_string() + "/" + file_path) {
        Ok(document) => { add_pages(folder_path, file_path, &document, index); }
        Err(err) => { println!("Failed to read the PDF {}: {}", file_path, err); }
    }
}

fn add_pages(folder_path: &String, file_path: &String, document: &Document, index: &mut HashMap<String, HashSet<Source>>) {
    let number_of_pages = document.page_iter().count();
    for i in 1..=number_of_pages {
        let page_text_result = document.extract_text(&[i as u32]);
        match page_text_result {
            Ok(page) => { add_page(folder_path, file_path, &page, &i, index) }
            Err(err) => { println!("Failed to read page {}: {}", i, err); }
        }
    }
}

fn add_page(folder_path: &String, file_path: &String, page: &String, page_number: &usize, index: &mut HashMap<String, HashSet<Source>>) {
    let words: Vec<String> = get_words(page);
    for word in words {
        let new_source = Source::new(folder_path.clone(), file_path.clone(), page_number.clone());
        let processed_word = process(word);
        if index.contains_key(&processed_word) {
            index.get_mut(&processed_word).unwrap().insert(new_source);
        }
        else {
            index.insert(processed_word, HashSet::from([new_source]));
        }
    }
}

fn get_words(text: &String) -> Vec<String> {
    let mut temp = text.clone();
    temp.retain(|c| c.is_alphabetic() || c == '\n' || c == ' ');
    temp
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|x| x.to_string()).collect()
}

fn process(str: String) -> String {
    str.to_uppercase()
}