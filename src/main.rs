use std::env::args;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::error::Error;

use crate::index::Index;
use crate::indexer::IndexBuilder;
use crate::search::SearchEngine;
use crate::source::Source;

mod indexer;
mod files;
mod source;
mod index;
mod stop_words;
mod search;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("Usage example: dirsearch [command] [dir]");
        return Ok(())
    }

    let command: &str = &args[1];
    let dir: &str = &args[2];


    match command {
        "search" => {
            let index_builder = IndexBuilder::new(dir.to_string());
            let index = index_builder.get_index()?;
            open_search(&index)
        },
        "index-rebuild" => {
            let index_builder = IndexBuilder::new(dir.to_string());
            index_builder.rebuild_index()
        }
        cmd => {
            eprintln!("Unknown command: {cmd}");
            return Ok(())
        }
    }
}

fn open_search(index: &Index) -> Result<(), Box<dyn Error>> {
    let mut input = String::from("");
    let search_engine = SearchEngine::new(&index);

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

        let results = search_engine.search(&input)?;

        let first_results: Vec<&(Source, f32)> = results.iter().take(5).collect();
        for (source, score) in &first_results {
            println!("{}, score: {}", source, score);
        }

        let _ = stdin().read_line(&mut String::new()).unwrap();
        input.clear();
    }

    Ok(())
}


fn print_banner() {
    println!(r#"
    ____  _      _____                      __
   / __ \(_)____/ ___/___  ____ ___________/ /_
  / / / / / ___/\__ \/ _ \/ __ `/ ___/ ___/ __ \
 / /_/ / / /   ___/ /  __/ /_/ / /  / /__/ / / /
/_____/_/_/   /____/\___/\__,_/_/   \___/_/ /_/
                                                v1.0.0"#)
}