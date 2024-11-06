use std::collections::HashMap;

use rust_stemmers::{Algorithm, Stemmer};
use std::error::Error;
use anyhow::anyhow;

use crate::index::Index;
use crate::source::Source;

pub struct SearchEngine<'a> {
    index: &'a Index,
    stemmer: Stemmer
}

impl <'a> SearchEngine<'a> {
    pub fn new(index: &'a Index) -> Self {
        let stemmer = Stemmer::create(Algorithm::English);
        Self { index, stemmer }
    }

    pub fn search(&self, input: &str) -> Result<Vec<(Source, f32)>, Box<dyn Error>> {
        let keywords = self.get_keywords(input);
        let source_hash_score_map = self.get_source_hash_score_map(keywords)?;

        let mut sorted_sources: Vec<(u64, f32)> = Vec::new();
        for (source, score) in source_hash_score_map.iter() {
            sorted_sources.push((*source, *score));
        }
        sorted_sources.sort_by(|first, second| second.1.to_owned().partial_cmp(&first.1).unwrap());

        Ok(sorted_sources
            .iter()
            .map(|(source_hash, score)|
                (self.index.get_source(source_hash).expect(format!("No source with hash: {} found", source_hash).as_str()), *score)
            )
            .collect())
    }

    fn get_source_hash_score_map(&self, keywords: Vec<String>) -> Result<HashMap<u64, f32>, Box<dyn Error>> {
        let rev_index_records: Vec<_> = keywords
            .iter()
            .map(|keyword| self.index.get_rev_index(keyword))
            .collect();

        let error_strings: Vec<String> = rev_index_records
            .iter()
            .filter(|result| result.is_err())
            .map(|result| result.as_ref().err().unwrap())
            .map(|err| err.to_string())
            .collect();

        let error_msg = error_strings.into_iter().reduce(|first, second| first + "\n" + second.as_str());
        match error_msg {
            Some(msg) => return Err(Box::from(anyhow!(msg))),
            None => ()
        };

        let rev_index_records = rev_index_records.into_iter()
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .filter(|option| option.is_some())
            .map(|option| option.unwrap());

        let source_hashes_with_scores = rev_index_records
            .flat_map(|record|
                {
                    record.sources
                        .iter()
                        .map(|(source_hash, occur)| (*source_hash, (*occur as f32) / (record.freq as f32)))
                        .collect::<Vec<(u64, f32)>>()
                }
            );

        let mut source_hash_score_map = HashMap::new();
        for (source_hash, score) in source_hashes_with_scores {
            source_hash_score_map
                .entry(source_hash)
                .and_modify(|old_score| *old_score += score)
                .or_insert(score);
        }

        Ok(source_hash_score_map)
    }

    fn get_keywords(&self, string: &str) -> Vec<String> {
        string
            .trim()
            .split_whitespace()
            .map(|word| word.to_lowercase())
            .map(|x| self.stemmer.stem(x.as_str()).to_string())
            .map(|x| x.to_uppercase())
            .collect()
    }

}
