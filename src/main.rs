#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    println!("Here's the epsilon: {:?}", '\0');
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

fn convert_regex_to_nfa(expression: &RegexExpr) -> Nfa {
    todo!()
}

/// Runs a NFA on an input string.
/// returns true if the NFA accepts the input string, and false otherwise.
fn run_nfa(nfa: &Nfa, input_string: &str) -> bool {
    todo!()
}
