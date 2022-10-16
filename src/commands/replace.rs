use std::{error::Error, fmt};

use regex::{Regex, RegexBuilder};

use crate::{domain, domain::Id, request};

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingQuery,
    UnrecognizedFlag(char),
    DuplicatedFlag(char),
    InvalidRegex(regex::Error),
}

impl From<regex::Error> for ParseError {
    fn from(cause: regex::Error) -> Self {
        Self::InvalidRegex(cause)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingQuery => {
                write!(fmtr, "missing query regex in rule")
            },
            Self::UnrecognizedFlag(flag) => {
                write!(fmtr, "{:?} is an unrecognized flag", flag)
            },
            Self::DuplicatedFlag(flag) => {
                write!(fmtr, "{:?} flag is duplicated", flag)
            },
            Self::InvalidRegex(cause) => {
                write!(fmtr, "invalid query regex: {}", cause)
            },
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRegex(cause) => Some(cause),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Flags {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    octal: bool,
    global: bool,
}

#[derive(Debug, Clone)]
pub enum ReplacementNode {
    Text(String),
    Index(usize),
}

#[derive(Debug, Clone)]
pub struct Replacement {
    pub nodes: Vec<ReplacementNode>,
}

impl Default for Replacement {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
}

impl Replacement {
    fn parse(replacement_str: &str) -> Self {
        let mut this = Self::default();
        let mut curr_text = String::new();
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    pub query: Regex,
    pub replacement: Replacement,
    pub is_global: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct RequestParser;

impl RequestParser {
    fn split_bar_escaping<'input>(
        &self,
        input: &'input str,
    ) -> Option<(&'input str, &'input str)> {
        let mut escape = false;

        input.split_once(|character| {
            if escape && (character == '/' || character == '\\') {
                escape = false;
                false
            } else {
                escape = character == '\\';
                character == '/'
            }
        })
    }
}

impl<M, C> request::Parser<M, C> for RequestParser
where
    M: Id,
    C: Id,
{
    type Error = ParseError;
    type Request = Request;

    fn parse(
        &self,
        _bot: &domain::Bot,
        message: &domain::Message<M, C>,
    ) -> Option<Result<Self::Request, Self::Error>> {
        let (_, mut tail) = message.data.content.split_once("s/")?;
        let (query_str, new_tail) = match self.split_bar_escaping(tail) {
            Some(split) => split,
            None => return Some(Err(ParseError::MissingQuery)),
        };
        tail = new_tail;
        let (replacement_str, flags_str) = match self.split_bar_escaping(tail) {
            Some((replacement, flags)) => (replacement, flags),
            None => (tail, ""),
        };
        let flags = match Flags::parse(flags_str) {
            Ok(flags) => flags,
            Err(error) => return Some(Err(error)),
        };
        let query = match RegexBuilder::new(query_str)
            .case_insensitive(flags.case_insensitive)
            .multi_line(flags.multi_line)
            .dot_matches_new_line(flags.dot_matches_new_line)
            .swap_greed(flags.swap_greed)
            .ignore_whitespace(flags.ignore_whitespace)
            .unicode(flags.unicode)
            .octal(flags.octal)
            .build()
        {
            Ok(regex) => regex,
            Err(error) => return Some(Err(ParseError::InvalidRegex(error))),
        };
        let replacement = Replacement::parse(replacement_str);
        Some(Ok(Request { query, replacement, is_global: flags.global }))
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            ignore_whitespace: false,
            unicode: false,
            octal: false,
            global: false,
        }
    }
}

impl Flags {
    fn parse(flags_str: &str) -> Result<Self, ParseError> {
        let mut this = Self::default();

        for character in flags_str.chars() {
            match character {
                'i' => Self::set(&mut this.case_insensitive, character)?,
                'm' => Self::set(&mut this.multi_line, character)?,
                's' => Self::set(&mut this.dot_matches_new_line, character)?,
                'U' => Self::set(&mut this.swap_greed, character)?,
                'x' => Self::set(&mut this.ignore_whitespace, character)?,
                'o' => Self::set(&mut this.octal, character)?,
                'g' => Self::set(&mut this.global, character)?,
                _ => Err(ParseError::UnrecognizedFlag(character))?,
            }
        }

        Ok(this)
    }

    fn set(field: &mut bool, flag_char: char) -> Result<(), ParseError> {
        if *field {
            Err(ParseError::DuplicatedFlag(flag_char))
        } else {
            *field = true;
            Ok(())
        }
    }
}
