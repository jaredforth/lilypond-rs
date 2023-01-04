//! This contains logic for parsing LilyPond

use lazy_static::lazy_static;

use regex::Regex;

/// Gets content between curly brackets
///
/// # Examples
///
/// ```
/// use lilypond::parser::curly_brackets;
///
/// let c1 = curly_brackets("{ c e g }");
///
/// assert_eq!(c1, Some(" c e g "));
/// ```
pub fn curly_brackets(input: &str) -> Option<&str> {
    lazy_static! {
        static ref MU_EXP: Regex = Regex::new(r"\{(?P<music>.*)\}").unwrap();
    }
    MU_EXP
        .captures(input)
        .and_then(|cap| cap.name("music").map(|login| login.as_str()))
}
