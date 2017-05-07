#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate lalrpop_intern;
extern crate lalrpop_util;

pub mod ast;
pub mod errors;
mod parser;

use errors::Result;
use lalrpop_util::ParseError;
use std::fmt::Write;

pub fn parse_program(text: &str) -> Result<ast::Program> {
    match parser::parse_Program(text) {
        Ok(v) => Ok(v),
        Err(e) => bail!("parse error: {:?}", e),
    }
}

pub fn parse_ty(text: &str) -> Result<ast::Ty> {
    match parser::parse_Ty(text) {
        Ok(v) => Ok(v),
        Err(e) => bail!("error parsing `{}`: {:?}", text, e),
    }
}

pub fn parse_goal(text: &str) -> Result<Box<ast::Goal>> {
    match parser::parse_Goal(text) {
        Ok(v) => Ok(v),
        Err(e) => {
            let position_string = |start: usize, end: usize| {
                let mut output = String::new();
                let text = text.replace("\n", " ").replace("\r", " ");
                write!(output, "position: `{}`\n", text).expect("str-write cannot fail");
                write!(output, "           ").expect("str-write cannot fail");
                for _ in 0..start { output.push_str(" "); }
                for _ in start..end { output.push_str("^"); }
                output.push_str("\n");
                output
            };
            match e {
                ParseError::InvalidToken { location } =>
                    bail!("parse error: {:?}\n{}", e, position_string(location, location+1)),
                ParseError::UnrecognizedToken { token: Some((start, _, end)), .. } =>
                    bail!("parse error: {:?}\n{}", e, position_string(start, end)),
                ParseError::ExtraToken { token: (start, _, end), .. } =>
                    bail!("parse error: {:?}\n{}", e, position_string(start, end)),
                _ =>
                    bail!("parse error: {:?}", e),
            }
        }
    }
}

