use std::env; //environment
use std::fs; //filesystem
use std::process;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive,})
    }
}



pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{ //tie the lifetime of the result to the lifetime of the inputs
    contents
        .lines().
        filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str,contents: &'a str,) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}", line);
    }
    Ok(())
}
