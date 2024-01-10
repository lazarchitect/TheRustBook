use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &str> {
        
        if args.len() < 3 {
            Err("not enough cmd line args!")
        }
        else {
            Ok(Config { 
                filename: args[1].clone(), 
                query: args[2].clone(), 
                ignore_case: env::var("IGNORE_CASE").is_ok() 
            })
        }
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    
    let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

    for line in results {
        println!("{line}");
    };
    Ok(())
}

fn search <'a>(query: & str, contents: & 'a str) -> Vec<&'a str> {

    let mut results = vec!();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

fn search_case_insensitive <'a> (query: & 'a str,  contents: & 'a str) -> Vec<&'a str>{
    
    let mut results: Vec<&str> = Vec::new();
    
    let lowercase_query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result () {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }

    #[test]
    fn case_insensitive () {
        let query = "rUST";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));

    }

}