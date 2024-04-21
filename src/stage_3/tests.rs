#![cfg(test)]

use super::run_nfa;
use crate::{stage_2::convert_regex_to_nfa, RegexExpr};

#[test]
fn test_run_nfa1() {
    let m = convert_regex_to_nfa(&RegexExpr::SingleChar('a'));

    assert!(run_nfa(&m, "a"));
    assert!(!run_nfa(&m, ""));
    assert!(!run_nfa(&m, "aa"));
    assert!(!run_nfa(&m, "c"));
}

#[test]
fn test_run_nfa2() {
    let expr = RegexExpr::Concat(
        Box::new(RegexExpr::SingleChar('a')),
        Box::new(RegexExpr::SingleChar('b')),
    );
    let m = convert_regex_to_nfa(&expr);

    assert!(run_nfa(&m, "ab"));
    assert!(!run_nfa(&m, "a"));
    assert!(!run_nfa(&m, ""));
    assert!(!run_nfa(&m, "aa"));
    assert!(!run_nfa(&m, "c"));
}

#[test]
fn test_run_nfa3() {
    let expr = RegexExpr::Star(Box::new(RegexExpr::SingleChar('a')));
    let m = convert_regex_to_nfa(&expr);

    assert!(run_nfa(&m, ""));
    assert!(run_nfa(&m, "a"));
    assert!(run_nfa(&m, "aa"));
    assert!(run_nfa(&m, "aaaaaa"));

    assert!(!run_nfa(&m, "ab"));
    assert!(!run_nfa(&m, "z"));
    assert!(!run_nfa(&m, "c"));
    assert!(!run_nfa(&m, "ca"));
}

#[test]
fn test_run_nfa4() {
    // test the regular expression "(a*)|(bc)(d*))"

    let rhs_expr = RegexExpr::Concat(
        Box::new(RegexExpr::Concat(
            Box::new(RegexExpr::SingleChar('b')),
            Box::new(RegexExpr::SingleChar('c')),
        )),
        Box::new(RegexExpr::Star(Box::new(RegexExpr::SingleChar('d')))),
    );
    let lhs_expr = RegexExpr::Star(Box::new(RegexExpr::Star(Box::new(RegexExpr::SingleChar(
        'a',
    )))));
    let expr = RegexExpr::Or(Box::new(lhs_expr), Box::new(rhs_expr));
    let m = convert_regex_to_nfa(&expr);

    // these strings match
    assert!(run_nfa(&m, ""));
    assert!(run_nfa(&m, "a"));
    assert!(run_nfa(&m, "aa"));
    assert!(run_nfa(&m, "aaaaaa"));
    assert!(run_nfa(&m, "bc"));
    assert!(run_nfa(&m, "bcd"));
    assert!(run_nfa(&m, "bcdd"));
    assert!(run_nfa(&m, "bcdddddd"));

    // these strings don't match
    assert!(!run_nfa(&m, "abc"));
    assert!(!run_nfa(&m, "abcd"));
    assert!(!run_nfa(&m, "abcd"));
    assert!(!run_nfa(&m, "dabc"));
    assert!(!run_nfa(&m, "dbc"));
    assert!(!run_nfa(&m, "dbcd"));
    assert!(!run_nfa(&m, "d"));
    assert!(!run_nfa(&m, "bccd"));
    assert!(!run_nfa(&m, "bbcd"));
}
