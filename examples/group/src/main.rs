#![feature(proc_macro_hygiene)]
#![warn(rust_2018_idioms)]

use pear::result::Result;
use pear::macros::{parser, switch, parse_declare, parse};
use pear::input::Span;
use pear::parsers::*;

parse_declare!(Input<'a>(Token = char, Slice = &'a str, Many = &'a str, Context = Span<'a>));

#[derive(Debug)]
struct Tokens(Vec<Token>);

#[derive(Debug)]
enum Token {
    Group(Group),
    Ident(String)
}

#[derive(Debug)]
struct Group {
    start: char,
    tokens: Tokens,
    end: char
}

#[inline]
fn is_whitespace(&byte: &char) -> bool {
    byte == ' ' || byte == '\t' || byte == '\n'
}

#[inline]
fn is_ident_char(&byte: &char) -> bool {
    match byte { '0'..='9' | 'a'..='z' | 'A'..='Z' => true, _ => false }
}

#[inline]
fn is_start_group_char(&c: &char) -> bool {
    c == '[' || c == '('
}

#[inline]
fn inverse(c: char) -> char {
    match c {
        '[' => ']',
        '(' => ')',
        _ => panic!("oh no!")
    }
}

#[parser]
fn group<'a, I: Input<'a>>(input: &mut I, kind: char) -> Result<Group, I> {
    let (start, tokens, end) = (eat(kind)?, tokens()?, eat(inverse(kind))?);
    Group { start, tokens, end }
}

#[parser]
fn ident<'a, I: Input<'a>>(input: &mut I) -> Result<String, I> {
    take_some_while(is_ident_char)?.to_string()
}

#[parser]
fn tokens<'a, I: Input<'a>>(input: &mut I) -> Result<Tokens, I> {
    let mut tokens = Vec::new();
    loop {
        skip_while(is_whitespace)?;
        let token = switch! {
            c@peek_if_copy(is_start_group_char) => Token::Group(group(c)?),
            i@ident() => Token::Ident(i),
            _ => break,
        };

        tokens.push(token);
    }

    Tokens(tokens)
}

const STRING: &str = "(( hi )) ([ (hey)  there ]) hi";

fn main() {
    let result = parse!(tokens: &mut pear::input::Text::from(STRING));

    match result {
        Err(ref e) => println!("Error: {}", e),
        Ok(v) => println!("Got: {:#?}", v)
    }
}