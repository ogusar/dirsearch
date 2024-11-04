use std::env::args;
use std::error::Error;
use std::hash::Hash;
use std::io::{stdin, stdout, Write};
use std::ops::Deref;
use std::process::Command;

use crate::index::Index;
use crate::indexer::IndexBuilder;
use crate::search::SearchEngine;
use crate::source::Source;

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

    let index_builder = IndexBuilder::new(dir.to_string());
    let index = index_builder.build().unwrap();

    process_input(index);
}

fn process_input(index: Index) {
    let mut input = String::from("");
    let search_engine = SearchEngine::new(index);

    loop {
        let _ = Command::new("clear").status();
        print_banner();
        println!("------------------------------------------------");
        print!("Enter the keywords separated by a whitespace: > ");
        stdout().flush().expect("Failed to output message");

        stdin().read_line(&mut input).expect("Failed to read from terminal");
        if input.trim() == "exit" {
            break;
        }

        let results = search_engine.search(&input);

        let mut i = 0;
        let first_results: Vec<&(Source, f32)> = results.iter().take(5).collect();
        for (source, score) in &first_results {
            println!("{}, score: {}", source, score);
        }

        let _ = stdin().read_line(&mut String::new()).unwrap();
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