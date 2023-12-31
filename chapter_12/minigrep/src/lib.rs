use std::fs;
use std::error::Error;


pub struct Config {
    pub filename: String,
    pub query: String
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &str> {
        
        if args.len() < 3 {
            return Err("not enough cmd line args!")
        }
        
        Ok(Config { filename: args[1].clone(), query: args[2].clone() })

    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    
    for line in search(&config.query, &contents) {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result () {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }
}