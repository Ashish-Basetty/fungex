#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    println!("Here's the epsilon: {:?}", '\0');
}

type State = usize;

// First, go through our string, and for any adjacent chars (or parentheses), and insert a carat if necessary
// If alphabet: add it to the end of a list 
// If open paren: push to stack
// If close paren: pop everything until the last paren
// If operator: push onto the stack

#[derive(Debug)]
struct Nfa {
    initial_state: State,
    accepting_state: State,
    /// A transition from state q1 to q2 upon input character c will be represented as:
    /// q1: [(c, q2), ...]   (the vec represents all outgoing transitions from q1)
    /// If c is the zero byte '\0', then the transition is an epsilon transition
    transitions: HashMap<State, Vec<(char, State)>>,
}

#[derive(Debug, PartialEq, Eq)]
enum RegexExpr {
    SingleChar(char), // char should be a lowercase or uppercase letter (a-z or A-Z)
    Star(Box<RegexExpr>),
    Concat(Box<RegexExpr>, Box<RegexExpr>),
    Or(Box<RegexExpr>, Box<RegexExpr>),
}

fn print_vec(v: &Vec<char>){
    print!("[");
    for item in v{
        print!("{}, ", item);
    }
    println!("]");
}

/// Parses an input string representing a regex expression.
/// The input string will consist of uppercase and lowercase English letters
/// as well as characters '(' ')' for grouping expressions '*' for Kleene star and '|' for OR.
///
/// the input "ab|c" should be read as "(ab)|c",  with concatenation having higher precedence than OR.
/// the input "a|b*" should be read as "a|(b*)",  with kleene star   having higher precedence than OR.
/// the input "ab*" should be read as "a(b*)",    with kleene star   having higher precedence than concatenation.
fn parse_regex(input_string: &str) -> RegexExpr {
    let mut stack: Vec<char> = Vec::new();
    let mut rps: String = String::new();

    let chars: Vec<char> = input_string.chars().collect();

    //println!("Original String: {}", input_string);

    for index in 0..chars.len(){
        print_vec(&stack);

        let c = chars[index];

        if c.is_alphanumeric() || c == '*'{
            rps.push(c);
            if index+1 < chars.len() && (chars[index+1].is_alphanumeric() || chars[index+1] == '('){
                stack.push('^');
            }
        }
        else if c == '('{
            stack.push('(');
        }
        else if c == ')'{
            let mut curr = stack.pop();
            while curr != None && curr != Some('('){
                rps.push(curr.unwrap());
                curr = stack.pop();
            }
            if index+1 < chars.len() && (chars[index+1].is_alphanumeric() || chars[index+1] == '('){
                stack.push('^');
            }
        }
        else if c == '|'{
            let mut curr = stack.pop();
            while curr != None && curr == Some('^'){
                rps.push(curr.unwrap());
                curr = stack.pop();
            }
            if curr != None{
                stack.push(curr.unwrap());
            }
            stack.push('|');
        }
        else {
            panic!("Regex character not recognized");
        }
    }

    while stack.len() > 0{
        let curr = stack.pop();
        if (curr != None && curr != Some(')')){
            rps.push(curr.unwrap());
        }
    }

    //println!("Reverse Polish String: {}\n", rps);

    //Start from the end and move left, recursively right tree first

    let c_vec: Vec<char> = rps.chars().collect();

    if c_vec.len() <= 0{
        panic!("Empty Input String");
    }

    tree_from_str(&c_vec, c_vec.len()-1).0

}

fn tree_from_str(polish_str: &Vec<char>, start: usize) -> (RegexExpr, usize){
    let s_size: usize = polish_str.len();

    if start < 0{
        panic!("Invalid Regex String");
    }

    let size: usize = 0;
    let curr = polish_str[start];


    if curr.is_alphanumeric(){
        return (RegexExpr::SingleChar(curr), 1)
    }
    if curr == '*'{
        let (child, ct) = tree_from_str(polish_str, start-1);
        return(RegexExpr::Star(Box::new(child)), 1 + ct);
    }
    else if curr == '^'{
        let (childr, ctr) = tree_from_str(polish_str, start-1);
        let (childl, ctl) = tree_from_str(polish_str, start-ctr-1);
        return(RegexExpr::Concat(Box::new(childl), Box::new(childr)), 1 + ctl + ctr);
    }
    else{
        let (childr, ctr) = tree_from_str(polish_str, start-1);
        let (childl, ctl) = tree_from_str(polish_str, start-ctr-1);
        return(RegexExpr::Or(Box::new(childl), Box::new(childr)), 1 + ctl + ctr);
    }

}

fn convert_regex_to_nfa(expression: &RegexExpr) -> Nfa {
    todo!()
}

/// Runs a NFA on an input string.
/// returns true if the NFA accepts the input string, and false otherwise.
fn run_nfa(nfa: &Nfa, input_string: &str) -> bool {
    todo!()
}

#[test]
fn test_parse_regex1() {
    let expected_expr = RegexExpr::SingleChar('a');
    assert_eq!(parse_regex("a"), expected_expr);
}

#[test]
fn test_parse_regex2() {
    let expected_expr = RegexExpr::Concat(
        Box::new(RegexExpr::SingleChar('a')),
        Box::new(RegexExpr::SingleChar('b')),
    );
    assert_eq!(parse_regex("ab"), expected_expr);
}

#[test]
fn test_parse_regex3() {
    let expected_expr = RegexExpr::Or(
        Box::new(RegexExpr::SingleChar('a')),
        Box::new(RegexExpr::SingleChar('b')),
    );
    assert_eq!(parse_regex("a|b"), expected_expr);
    assert_eq!(parse_regex("(a)|b"), expected_expr);
    assert_eq!(parse_regex("((a)|(((b))))"), expected_expr);
}

#[test]
fn test_parse_regex4() {
    let expected_expr = RegexExpr::Star(Box::new(RegexExpr::SingleChar('a')));
    assert_eq!(parse_regex("a*"), expected_expr);
}

#[test]
fn test_parse_regex5() {
    let e1 = RegexExpr::Star(Box::new(RegexExpr::SingleChar('a')));
    let expected_expr = RegexExpr::Or(Box::new(e1), Box::new(RegexExpr::SingleChar('b')));

    assert_eq!(parse_regex("a*|b"), expected_expr);
    assert_eq!(parse_regex("(a*)|b"), expected_expr);
    assert_eq!(parse_regex("a*|(b)"), expected_expr);
}

#[test]
fn test_parse_regex6() {
    let e1 = RegexExpr::Star(Box::new(RegexExpr::SingleChar('b')));
    let expected_expr = RegexExpr::Or(Box::new(RegexExpr::SingleChar('a')), Box::new(e1));

    assert_eq!(parse_regex("a|b*"), expected_expr);
    assert_eq!(parse_regex("a|(b*)"), expected_expr);
}

#[test]
fn test_parse_regex7() {
    let e1 = RegexExpr::Concat(
        Box::new(RegexExpr::SingleChar('a')),
        Box::new(RegexExpr::SingleChar('b')),
    );
    let expected_expr = RegexExpr::Or(Box::new(e1), Box::new(RegexExpr::SingleChar('c')));

    assert_eq!(parse_regex("ab|c"), expected_expr);
    assert_eq!(parse_regex("ab|(c)"), expected_expr);
    assert_eq!(parse_regex("(ab)|c"), expected_expr);
}
