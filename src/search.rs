use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use crate::index::{Index, RevIndexRecord};
use rust_stemmers::{Algorithm, Stemmer};
use crate::source::Source;

pub struct SearchEngine {
    index: Index,
    stemmer: Stemmer
}

impl SearchEngine {
    pub fn new(index: Index) -> Self {
        let stemmer = Stemmer::create(Algorithm::English);
        Self { index, stemmer }
    }

    pub fn search(&self, input: &String) -> Vec<(Source, f32)>{
        let keywords = self.get_keywords(&input);
        let rev_index_records = keywords
            .iter()
            .map(|keyword| self.index.get_rev_index(keyword))
            .filter(|record| record.is_some())
            .map(|option| option.unwrap());

        let sources_with_scores = rev_index_records
            .flat_map(|record|
                {
                    record.sources
                        .iter()
                        .map(|(source_hash, occur)| (source_hash.clone(), (*occur as f32) / (record.freq as f32)))
                        .collect::<Vec<(u64, f32)>>()
                }
            );

        let mut source_score = HashMap::new();
        for (source, score) in sources_with_scores {
            if source_score.contains_key(&source) {
                *source_score.get_mut(&source).unwrap() += score;
            }
            else {
                source_score.insert(source, score);
            }
        }

        let mut sorted_sources: Vec<(u64, f32)> = Vec::new();
        for (source, score) in source_score {
            sorted_sources.push((source, score));
        }
        sorted_sources.sort_by(|first, second| second.1.to_owned().partial_cmp(&first.1).unwrap());
        sorted_sources
            .iter()
            .map(|(source_hash, score)| (self.index.get_source(source_hash).expect("No source with given hash found"), *score))
            .collect()
    }

    fn get_keywords(&self, string: &String) -> Vec<String> {
        string
            .trim()
            .split_whitespace()
            .map(|word| word.to_lowercase())
            .map(|x| self.stemmer.stem(x.as_str()).to_string())
            .map(|x| x.to_uppercase())
            .collect()
    }

}
