use std::collections::{HashMap, HashSet};
use std::env;
use std::env::args;
use std::error::Error;
use std::fmt::format;
use std::hash::Hash;
use std::io::{stdin, stdout, Write};
use std::ops::Deref;
use crate::index::Index;
use crate::indexer::IndexBuilder;
use crate::source::Source;
use crate::search::SearchEngine;

mod indexer;
mod files;
mod source;
mod other;
mod index;
mod stop_words;
mod search;

fn main() {
    let args: Vec<String> = args().collect();
    let dir: &str = &args[1];
    print_banner();

    let mut index_builder = IndexBuilder::new(dir.to_string());
    let index = index_builder.build().unwrap();

    process_input(index);
}

fn process_input(index: Index) {
    let mut input = String::from("");
    let search_engine = SearchEngine::new(index);

    loop {
        print!("Enter the keywords separated by a whitespace: > ");
        stdout().flush().expect("Failed to output message");

        stdin().read_line(&mut input).expect("Failed to read from terminal");
        if input.trim() == "exit" {
            break;
        }

        let results = search_engine.search(&input);

        let mut i = 0;
        for (source, score) in results {
            if i == 5 {
                break;
            }
            println!("{}, score: {}", source, score);
            i += 1;
        }

        input.clear();
    }
}


fn print_banner() {
    println!(r#"
        ___ _      __                     _
       /   (_)_ __/ _\ ___  __ _ _ __ ___| |__
      / /\ / | '__\ \ / _ \/ _` | '__/ __| '_ \
     / /_//| | |  _\ \  __/ (_| | | | (__| | | |
    /___,' |_|_|  \__/\___|\__,_|_|  \___|_| |_|
                                            "#)
}