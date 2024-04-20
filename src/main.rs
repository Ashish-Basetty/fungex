#![allow(dead_code)]
#![allow(unused_variables)]

mod stage_2;

use std::collections::HashMap;

use crate::stage_2::convert_regex_to_nfa;

fn main() {
    let expr = RegexExpr::SingleChar('a');

    println!("{:?}", convert_regex_to_nfa(&expr));
}

type State = usize;

#[derive(Debug)]
struct Nfa {
    initial_state: State,
    accepting_state: State,
    /// A transition from state q1 to q2 upon input character c will be represented as:
    /// q1: [(c, q2), ...]   (the vec represents all outgoing transitions from q1)
    /// If c is the zero byte '\0', then the transition is an epsilon transition
    transitions: HashMap<State, Vec<(char, State)>>,
}

#[derive(Debug)]
enum RegexExpr {
    SingleChar(char), // char should be a lowercase or uppercase letter (a-z or A-Z)
    Star(Box<RegexExpr>),
    Concat(Box<RegexExpr>, Box<RegexExpr>),
    Or(Box<RegexExpr>, Box<RegexExpr>),
}

/// Parses an input string representing a regex expression.
/// The input string will consist of uppercase and lowercase English letters
/// as well as characters '(' ')' for grouping expressions '*' for Kleene star and '|' for OR.
///
/// the input "ab|c" should be read as "(ab)|c",  with concatenation having higher precedence than OR.
/// the input "a|b*" should be read as "a|(b*)",  with kleene star   having higher precedence than OR.
/// the input "ab*" should be read as "a(b*)",    with kleene star   having higher precedence than concatenation.
fn parse_regex(input_string: &str) -> RegexExpr {
    todo!()
}

/// Runs a NFA on an input string.
/// returns true if the NFA accepts the input string, and false otherwise.
fn run_nfa(nfa: &Nfa, input_string: &str) -> bool {
    todo!()
}
