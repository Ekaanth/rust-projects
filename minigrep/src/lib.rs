use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query : String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive})
}
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;


    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut  results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();
    
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()){
            result.push(line);
        }
    }

    result

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        Rust:\nsafe, fast, productive\n
        Pick three. 
        ";

        assert_eq!(vec!["safe, fast, productive"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCt";
        let content = "\
        Rust:\nsafe, fast, productive\n
        Pick three. 
        ";

        assert_eq!(vec!["safe, fast, productive"], search_case_insensitive(query, content));
    }

}