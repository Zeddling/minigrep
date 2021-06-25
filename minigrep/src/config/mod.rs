use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


#[path = "../search/mod.rs"] mod search;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments lol");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let is_case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(
            Config { 
            query: query.to_string(),
            filename: filename.to_string(),
            is_case_sensitive
            }
        )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let results = if config.is_case_sensitive {
        search::search_case_sensitive(&config.query, &contents)
    } else {
        search::search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}