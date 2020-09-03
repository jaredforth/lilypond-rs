//! This contains logic for parsing LilyPond

use nom::{
    IResult,
    sequence::delimited,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::char,
    bytes::complete::is_not
};

/// Gets content between curly brackets
///
/// ## Usage:
///
/// ```
/// use lilypond::parser::curly_brackets;
///
/// let c1 = curly_brackets("{ c e g }").unwrap();
/// let c2 = curly_brackets("{c e g}").unwrap();
///
/// assert_eq!(c1, ("", " c e g "));
/// assert_eq!(c2, ("", "c e g"));
/// ```
pub fn curly_brackets(input: &str) -> IResult<&str, &str> {
    delimited(char('{'), is_not("}"), char('}'))(input)
}