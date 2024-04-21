use crate::{parse_regex, RegexExpr};

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
