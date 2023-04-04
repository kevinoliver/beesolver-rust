use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::fs;
use std::error::Error;
use std::path::Path;

extern crate unidecode;
use unidecode::unidecode;

const DEFAULT_PATH: &str = "./american-english-large";
const DEFAULT_NAME: &str = "(default)";

pub struct Dictionary {
    name: String,
    words: HashSet<String>,
}

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
        let mut words = HashSet::new();

        let lines = Dictionary::read_lines(filename)?;
        for line in lines {
            let word = line?;
            if word.len() > 3 {
                words.insert(unidecode(&word));
            }
        }

        Ok(Dictionary { 
            name: name.to_string(),
            words 
        })
    }

    // todo not sure if this is the right way to return a generic iterator. maybe?
    pub fn words(&self) -> impl Iterator<Item = &str> {
        self.words.iter().map(|w| &w[..])
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn num_words(&self) -> usize {
        self.words.len()
    }

}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::Dictionary;

    const TEST_DICT: &str = "src/test/dictionary.txt";

    fn contains(dict: &Dictionary, word: &str) -> bool {
        for w in dict.words() {
            if w == word {
                return true;
            }
        }
        false
    }

    #[test]
    fn load_filters_out_short_words() -> Result<(), Box<dyn Error>> {
        let dict = Dictionary::load_path(TEST_DICT)?;
        assert!(!contains(&dict, "cat"));
        Ok(())
    }

    #[test]
    fn load_removes_duplicates() -> Result<(), Box<dyn Error>> {
        let dict = Dictionary::load_path(TEST_DICT)?;
        let mut seen_dogs = false;
        for w in dict.words() {
            if w == "dogs" {
                assert!(!seen_dogs);
                seen_dogs = true;
            }
        }
        Ok(())
    }

    #[test]
    fn load_normalizes_accents() -> Result<(), Box<dyn Error>> {
        let dict = Dictionary::load_path(TEST_DICT)?;
        assert!(!contains(&dict, "éclair"));
        assert!(contains(&dict, "eclair"));
        Ok(())
    }

}