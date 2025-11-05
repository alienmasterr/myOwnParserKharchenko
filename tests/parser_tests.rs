use arythemetic_expressions_parser_Kharchenko::ParseError;
use arythemetic_expressions_parser_Kharchenko::parseExpression;

#[test]
fn test_simple_add() {
    let r = parseExpression("1+2").unwrap();
    assert_eq!(r.result, 3.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['+']);
}

#[test]
fn test_space() {
    let r = parseExpression("1+ 2").unwrap();
    assert_eq!(r.result, 3.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['+']);
}

#[test]
fn test_simple_minus() {
    let r = parseExpression("1-2").unwrap();
    assert_eq!(r.result, -1.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['-']);
}

#[test]
fn test_simple_mul() {
    let r = parseExpression("1*2").unwrap();
    assert_eq!(r.result, 2.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['*']);
}

#[test]
fn test_simple_div() {
    let r = parseExpression("1/2").unwrap();
    assert_eq!(r.result, 0.5);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['/']);
}

#[test]
fn test_add_negative() {
    let r = parseExpression("1+(-2)").unwrap();
    assert_eq!(r.result, -1.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['-']);
}

#[test]
fn test_minus_negative() {
    let r = parseExpression("1-(-2)").unwrap();
    assert_eq!(r.result, 3.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['+']);
}

#[test]
fn test_mul_negative() {
    let r = parseExpression("1*(-2)").unwrap();
    assert_eq!(r.result, -2.0);
    assert_eq!(r.operands, vec![1.0, -2.0]);
    assert_eq!(r.operators, vec!['*']);
}

#[test]
fn test_div_negative() {
    let r = parseExpression("1/(-2)").unwrap();
    assert_eq!(r.result, -0.5);
    assert_eq!(r.operands, vec![1.0, -2.0]);
    assert_eq!(r.operators, vec!['/']);
}

#[test]
fn test_minus_whole_expr() {
    let r = parseExpression("-(1+2)").unwrap();
    assert_eq!(r.result, -3.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['+']);
}

#[test]
fn test_nested_brackets() {
    let r = parseExpression("((1+2))").unwrap();
    assert_eq!(r.result, 3.0);
    assert_eq!(r.operands, vec![1.0, 2.0]);
    assert_eq!(r.operators, vec!['+']);
}

#[test]
fn test_invalid_empty() {
    let err = parseExpression("").unwrap_err();
    matches!(err, ParseError::InvalidExpression);
}

#[test]
fn test_invalid_unbalanced_brackets() {
    let err = parseExpression("(1+2").unwrap_err();
    matches!(err, ParseError::InvalidExpression);
}