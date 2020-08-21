use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub word: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Usage: cargo run <word> <file-path>");
        }

        let word = args[1].clone();
        let file = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        return Ok(Config { word, file, case_sensitive });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.file)?;

    for line in search(&config.word, &text, config.case_sensitive) {
        println!("{}", line)
    }
    return Ok(());
}

fn search<'a>(word: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut lines = vec![];
    for line in contents.lines() {
        if case_sensitive {
            if line.contains(word) {
                lines.push(line);
            }
        } else {
            if line.to_lowercase().contains(&word.to_lowercase()) {
                lines.push(line);
            }
        }

    }
    
    return lines;
}

#[cfg(test)]
mod tests {
    use super::search;

    #[test]
    fn test_basic_search() {
        let contents = "\
blastoise and butterfree
pikachu and charizard";
        assert_eq!(vec!["pikachu and charizard"], search("zard", &contents, true));
    }

    #[test]
    fn test_case_insensitive_search() {
        let contents = "\
What you sow is what you reap
my way is my way
whatever comes my way I'll deal with it";
        assert_eq!(
            search("wHAT", &contents, false),
            vec!["What you sow is what you reap", "whatever comes my way I'll deal with it"]
        );
    }
}