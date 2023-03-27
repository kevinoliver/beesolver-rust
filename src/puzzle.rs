use std::collections::HashSet;

use super::WordResult;

pub struct Puzzle {
    required_letter: char,
    allowed_letters: HashSet<char> 
}

impl Puzzle {

    pub fn from(required_letter: char, other_letters: &str) -> Result<Puzzle, String> {
        if other_letters.len() != 6 {
            return Err(format!("Must have 6 other letters, found {}", other_letters.len()))
        }
    
        let mut allowed_letters = HashSet::new();
        for ch in other_letters.chars() {
            if !allowed_letters.insert(ch) {
                return Err(format!("other letters cannot have any duplicates: '{ch}'"))
            }
        }
        if !allowed_letters.insert(required_letter) {
            return Err(format!("other letters cannot contain the required letter: '{required_letter}'"))
        }
    
        Ok(Puzzle { required_letter, allowed_letters })
    }

    pub fn result_for(&self, candidate: &str) -> WordResult {
        let mut found_required = false;
        let mut all_are_allowed = true;
        for candidate_letter in candidate.chars() {
            if candidate_letter == self.required_letter {
                found_required = true;
            }
            if !self.allowed_letters.contains(&candidate_letter) {
                all_are_allowed = false;
                break;
            }
        }
        if found_required && all_are_allowed {
            let mut unique_letters = HashSet::new();
            for ch in candidate.chars() {
                unique_letters.insert(ch);
            }
            if unique_letters.len() == 7 {
                WordResult::Pangram(candidate.to_string())
            } else {
                WordResult::Valid(candidate.to_string())
            }
        } else {
            WordResult::Invalid
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use crate::WordResult;

    #[test]
    fn puzzle_new_other_letters_must_have_6_letters() {
        assert!(Puzzle::from('z', "12345").is_err());
        assert!(Puzzle::from('z', "1234567").is_err());
    }

    #[test]
    fn puzzle_new_other_letters_cannot_have_duplicates() {
        assert!(Puzzle::from('z', "123455").is_err());
    }

    #[test]
    fn puzzle_new_other_letters_cannot_have_required_letter() {
        assert!(Puzzle::from('z', "12345z").is_err());
    }

    #[test]
    fn puzzle_result_for_valid_words() {
        let puzzle = Puzzle::from('d', "ogselm").unwrap();
        assert_eq!(WordResult::Valid(String::from("dogs")), puzzle.result_for("dogs"));
        assert_eq!(WordResult::Valid(String::from("doom")), puzzle.result_for("doom"));
        assert_eq!(WordResult::Valid(String::from("does")), puzzle.result_for("does"));
        assert_eq!(WordResult::Valid(String::from("moods")), puzzle.result_for("moods"));
    }

    #[test]
    fn puzzle_result_for_pangrams() {
        let puzzle = Puzzle::from('d', "ogselm").unwrap();
        assert_eq!(WordResult::Pangram(String::from("dogselm")), puzzle.result_for("dogselm"));
        assert_eq!(WordResult::Pangram(String::from("dogselmm")), puzzle.result_for("dogselmm"));
        assert_eq!(WordResult::Pangram(String::from("dogselmdogselm")), puzzle.result_for("dogselmdogselm"));
    }

    #[test]
    fn puzzle_result_for_words_missing_required_letter() {
        let puzzle = Puzzle::from('d', "ogselm").unwrap();
        assert_eq!(WordResult::Invalid, puzzle.result_for("logs"));
        assert_eq!(WordResult::Invalid, puzzle.result_for("ogselm"));
    }

    #[test]
    fn puzzle_result_for_words_with_unallowed_letters() {
        let puzzle = Puzzle::from('d', "ogselm").unwrap();
        assert_eq!(WordResult::Invalid, puzzle.result_for("dogz"));
    }

}
