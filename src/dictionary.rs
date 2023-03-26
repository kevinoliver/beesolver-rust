use std::io;
use std::io::BufRead;
use std::fs;
use std::error::Error;
use std::path::Path;

extern crate unidecode;
use unidecode::unidecode;
pub struct Dictionary {
    name: String,
    words: Vec<String>,
}

const DEFAULT_PATH: &str = "./american-english-large";
const DEFAULT_NAME: &str = "(default)";

impl Dictionary {

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
        let file = fs::File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn load() -> Result<Dictionary, Box<dyn Error>> {
        Dictionary::load_internal(DEFAULT_NAME, DEFAULT_PATH)
    }

    pub fn load_path(filename: &str) -> Result<Dictionary, Box<dyn Error>> {
        Dictionary::load_internal(filename, filename)
    }

    fn load_internal(name: &str, filename: &str) -> Result<Dictionary, Box<dyn Error>> {
        let mut words = Vec::new();

        let lines = Dictionary::read_lines(filename)?;
        for line in lines {
            let word = line?;
            if word.len() > 3 {
                // todo: remove duplicates
                words.push(unidecode(&word));
            }
        }

        Ok(Dictionary { 
            name: name.to_string(),
            words 
        })
    }

    fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }

}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::Dictionary;

    const TEST_DICT: &str = "src/test/dictionary.txt";

    #[test]
    fn load_filters_out_short_words() -> Result<(), Box<dyn Error>> {
        let dict = Dictionary::load_path(TEST_DICT)?;
        assert!(!dict.contains("cat"));
        Ok(())
    }

    #[test]
    fn load_removes_duplicates() -> Result<(), Box<dyn Error>> {
        // todo ...
        Ok(())
    }

    #[test]
    fn load_normalizes_accents() -> Result<(), Box<dyn Error>> {
        let dict = Dictionary::load_path(TEST_DICT)?;
        assert!(!dict.contains("éclair"));
        assert!(dict.contains("eclair"));
        Ok(())
    }

}