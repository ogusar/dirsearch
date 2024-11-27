use std::error::Error;
use lopdf::Document;
use rust_stemmers::{Algorithm, Stemmer};

use crate::files::get_pdf_files;
use crate::index::{Index, RevIndexRecord};
use crate::source::Source;
use crate::stop_words::StopWords;

const DB_NAME: &str = "/.index";

pub struct IndexBuilder {
    stop_words: StopWords,
    index: Index,
    index_exists: bool,
    folder_path: String
}

impl IndexBuilder {
    pub fn new(folder_path: String) -> Self {
        let mut exists = true;
        let index = Index::new(&(folder_path.clone() + DB_NAME), &mut exists).unwrap();
        Self {
            stop_words: StopWords::new(),
            index,
            index_exists: exists,
            folder_path
        }
    }

    pub fn get_index(self) -> Result<Index, Box<dyn Error>> {
        if !self.index_exists {
            self.build_index()
        } else {
            Ok(self.index)
        }
    }

    fn build_index(self) -> Result<Index, Box<dyn Error>> {
        let files = get_pdf_files(&self.folder_path);
        println!("{} files detected:", files.len());
        files.iter().for_each(|file| println!("\t{file}"));
        for file in files {
            let res = self.add_file(&file);
            if res.is_err() {
                println!("Failed to scan {}", &file);
                println!("Error: {}", res.err().unwrap());
            }
        }

        Ok(self.index)
    }

    pub fn rebuild_index(self) -> Result<(), Box<dyn Error>> {
        self.index.delete()?;
        self.build_index()?;
        Ok(())
    }

    fn add_file(&self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let document =  Document::load(self.folder_path.to_string() + "/" + file_path)?;
        self.add_pages(file_path, document)?;
        Ok(())
    }

    fn add_pages(&self, file_path: &String, document: Document) -> Result<(), Box<dyn Error>> {
        let number_of_pages = document.page_iter().count();
        for i in 1..=number_of_pages {
            let page = document.extract_text(&[i as u32])?;
            self.add_page(file_path, &page, &i)?;
        }
        Ok(())
    }

    fn add_page(&self, file_path: &String, page: &String, page_number: &usize) -> Result<(), Box<dyn Error>> {
        let keywords: Vec<String> = self.get_keywords(page);
        let new_source = &Source::new(self.folder_path.clone(), file_path.to_string(), *page_number);

        self.index.add_source(&new_source)?;
        for keyword in keywords {
            let rev_index: RevIndexRecord;
            match self.index.get_rev_index(&keyword)? {
                Some(old_rev_index) => {
                    let mut new_sources = old_rev_index.sources;
                    for i in 0..new_sources.len() {
                        let (source, occ) = &mut new_sources[i];
                        if *source == new_source.get_hash() {
                            *occ = *occ + 1;
                            continue;
                        }
                        if i == new_sources.len() - 1 {
                            new_sources.push((new_source.get_hash(), 1));
                        }
                    }
                    rev_index = RevIndexRecord::new(old_rev_index.freq + 1, new_sources);
                }
                None => {
                    rev_index = RevIndexRecord::new(1, vec![(new_source.get_hash(), 1)])
                }
            }
            self.index.add_rev_index(&keyword, rev_index)?;
        }
        Ok(())
    }

    fn get_keywords(&self, text: &String) -> Vec<String> {
        let stemmer = Stemmer::create(Algorithm::English);
        let mut temp = text.clone();
        temp.retain(|c| c.is_alphabetic() || c == '\n' || c == ' ');
        temp
            .lines()
            .flat_map(|line| line.split_whitespace())
            .map(|x| stemmer.stem(x.to_lowercase().as_str()).to_string())
            .map(|x| x.to_uppercase())
            .filter(|x| !self.stop_words.is_stop_word(x))
            .collect()
    }
}

