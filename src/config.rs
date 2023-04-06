use argh::FromArgs;

#[derive(FromArgs)]
/// A Spelling Bee solver
pub struct Config {
    #[argh(positional)]
    /// the letter required in all words
    required_letter: char,

    #[argh(positional)]
    /// the 6 other allowed letters
    other_letters: String,

    #[argh(option)]
    /// path to a custom dictionary file
    dict: Option<String>,

    /// when off, only the stats for the solution are output (default on)
    #[argh(option, default = "true")]
    words_output: bool,
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(path) = &self.dict {
            if !std::path::Path::new(path).exists() {
                return Err(format!("dictionary not found: '{path}'"));
            }
        }
        Ok(())
    }

    pub fn required_letter(&self) -> char {
        self.required_letter
    }

    pub fn other_letters(&self) -> &str {
        &self.other_letters
    }

    pub fn words_output(&self) -> bool {
        self.words_output
    }

    pub fn dict(&self) -> Option<&str> {
        self.dict.as_ref().map(String::as_str)
    }
}
