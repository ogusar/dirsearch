use std::collections::{HashMap, HashSet};
use std::env;
use std::env::args;
use std::hash::Hash;
use std::io::{stdin, stdout, Write};
use std::ops::Deref;
use crate::files::{load_index, save_index};
use crate::indexer::{make_index};
use crate::source::Source;

mod indexer;
mod files;
mod source;
mod other;

const INDEX_FILE: &str = "index.bin";
fn main() {
    let args: Vec<String> = args().collect();
    let dir: &str = &args[1];
    let abs_index_file: String = format!("{}/{}", dir, INDEX_FILE);
    print_banner();

    let index: HashMap<String, HashSet<Source>>;
    match load_index(&abs_index_file) {
        Ok(res) => {
            index = res;
        }
        Err(_) => {
            println!("Please wait, creating an index...");
            index = make_index(String::from(dir));
            save_index(&index, &abs_index_file);
            println!("Save at {}", abs_index_file);
        }
    }

    let mut input = String::from("");

    loop {
        print!("Enter the keywords separated by a whitespace: > ");
        stdout().flush().expect("Failed to output message");

        stdin().read_line(&mut input).expect("Failed to read from terminal");
        if input.trim() == "exit" {
            break;
        }
        find_pages(&input, &index);
    }
}

fn intersections<'a, T>(mut sets: impl Iterator<Item = &'a HashSet<T>>) -> HashSet<T>
    where
        T: Clone + Eq + Hash + 'a,
{
    match sets.next() {
        Some(first) => sets.fold(first.clone(), |mut acc, set| {
            acc.retain(|item| set.contains(item));
            acc
        }),

        None => HashSet::new(),
    }
}

fn find_pages(input: &String, index: &HashMap<String, HashSet<Source>>) {
    let keywords = input
        .trim()
        .split_whitespace()
        .map(|word| word.to_uppercase());

    let mut sources_sets: Vec<HashSet<Source>> = Vec::new();
    for keyword in keywords {
        match index.get(&keyword) {
            None => { continue }
            Some(set) => { sources_sets.push(set.clone()) }
        };
        continue;
    }

    let sources: HashSet<Source> = intersections(sources_sets.iter());

    if sources.is_empty() {
        println!("No pages found. Try changing the keywords.");
    }
    else {
        for source in sources {
            println!("{}", source);
        }
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