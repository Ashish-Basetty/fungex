#![cfg(test)]

use crate::RegexExpr;

use super::{convert_regex_to_nfa, rename_nfa_states};

#[test]
fn test_generate_simple_nfa() {
    let m = convert_regex_to_nfa(&RegexExpr::SingleChar('a'));
    println!("Here is a nfa which accepts a single string 'a':");
    println!("{:?}", m);
}

#[test]
fn test_rename_nfas() {
    let mut m1 = convert_regex_to_nfa(&RegexExpr::SingleChar('a'));
    let mut m2 = convert_regex_to_nfa(&RegexExpr::SingleChar('b'));

    rename_nfa_states(&mut m1, &mut m2);

    println!("This test generated 2 simple nfas and then renamed them:");
    println!("{:?}", m1);
    println!("{:?}", m2);
}

#[test]
fn test_generate_simple_concat_nfa() {
    let expr = RegexExpr::Concat(
        Box::new(RegexExpr::SingleChar('a')),
        Box::new(RegexExpr::SingleChar('b')),
    );
    let m = convert_regex_to_nfa(&expr);
    println!("Here is a nfa which accepts a single string 'ab':");
    println!("{:?}", m);
}

#[test]
fn test_generate_simple_or_nfa() {
    let expr = RegexExpr::Or(
        Box::new(RegexExpr::SingleChar('a')),
        Box::new(RegexExpr::SingleChar('b')),
    );
    let m = convert_regex_to_nfa(&expr);
    println!("Here is a nfa which accepts the string 'a' or 'b':");
    println!("{:?}", m);
}
