use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;

extern crate unidecode;
use unidecode::unidecode;

pub struct Dictionary {
    name: String,
    words: HashSet<String>,
}

impl Dictionary {
    const DEFAULT_PATH: &str = "./american-english-large";
    const DEFAULT_NAME: &str = "(default)";

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn load() -> Result<Dictionary, Box<dyn Error>> {
        Dictionary::load_internal(Dictionary::DEFAULT_NAME, Dictionary::DEFAULT_PATH)
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
            words,
        })
    }

    fn iter(&self) -> DictIter<'_> {
        DictIter {
            iter: self.words.iter(),
        }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn num_words(&self) -> usize {
        self.words.len()
    }

    #[cfg(test)]
    fn contains(&self, word: &str) -> bool {
        for w in &self.words {
            if w == word {
                return true;
            }
        }
        false
    }
}

pub struct DictIter<'a> {
    iter: Iter<'a, String>,
}

impl<'a> Iterator for DictIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> IntoIterator for &'a Dictionary {
    type Item = &'a String;

    type IntoIter = DictIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        let dict = Dictionary::load_path(TEST_DICT)?;
        let mut seen_dogs = false;
        for w in &dict {
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
        assert!(!dict.contains("éclair"));
        assert!(dict.contains("eclair"));
        Ok(())
    }
}
