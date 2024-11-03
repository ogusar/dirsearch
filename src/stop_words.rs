use std::collections::HashSet;
use std::iter::Iterator;

const STOP_WORDS_FILE: &str = include_str!("resources/stop-words.txt");

pub struct StopWords {
    stop_words: HashSet<String>
}

impl StopWords {
    pub fn new() -> Self {
        Self {
            stop_words: HashSet::from_iter(
                STOP_WORDS_FILE
                    .lines()
                    .map(|x| x.to_uppercase())
                    .collect::<Vec<_>>()
            )
        }
    }

    pub fn is_stop_word(&self, word: &String) -> bool {
        self.stop_words.contains(word)
    }
}

