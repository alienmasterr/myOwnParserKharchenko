use arythemetic_expressions_parser_Kharchenko::ParseError;
use arythemetic_expressions_parser_Kharchenko::parseExpression;

#[test]
fn test_single() {
    let r = parseExpression("1").unwrap();
    assert_eq!(r.result, 1.0);
}

#[test]
fn test_single_neg() {
    let r = parseExpression("-(1)").unwrap();
    assert_eq!(r.result, -1.0);
}

#[test]
fn test_simple_add() {
    let r = parseExpression("1+2").unwrap();
    assert_eq!(r.result, 3.0);

}

#[test]
fn test_space() {
    let r = parseExpression("1+ 2").unwrap();
    assert_eq!(r.result, 3.0);

}

#[test]
fn test_simple_minus() {
    let r = parseExpression("1-2").unwrap();
    assert_eq!(r.result, -1.0);

}

#[test]
fn test_simple_mul() {
    let r = parseExpression("1*2").unwrap();
    assert_eq!(r.result, 2.0);

}

#[test]
fn test_simple_div() {
    let r = parseExpression("1/2").unwrap();
    assert_eq!(r.result, 0.5);

}

#[test]
fn test_add_negative() {
    let r = parseExpression("1+(-2)").unwrap();
    assert_eq!(r.result, -1.0);

}

#[test]
fn test_minus_negative() {
    let r = parseExpression("1-(-2)").unwrap();
    assert_eq!(r.result, 3.0);

}

#[test]
fn test_mul_negative() {
    let r = parseExpression("1*(-2)").unwrap();
    assert_eq!(r.result, -2.0);

}

#[test]
fn test_div_negative() {
    let r = parseExpression("1/(-2)").unwrap();
    assert_eq!(r.result, -0.5);

}

#[test]
fn test_minus_whole_expr() {
    let r = parseExpression("-(1+2)").unwrap();
    assert_eq!(r.result, -3.0);

}

#[test]
fn test_nested_brackets() {
    let r = parseExpression("((1+2))").unwrap();
    assert_eq!(r.result, 3.0);
}

#[test]
fn test_complex_expr() {
    let r = parseExpression("1+2*3-4/5").unwrap();
    assert_eq!(r.result, 6.2);
}

#[test]
fn test_log() {
    let r = parseExpression("log(1)").unwrap();
    assert_eq!(r.result, 0.0);

    let r2 = parseExpression("log(1+2)").unwrap();
    assert!((r2.result - (1.0f64 + 2.0f64).ln()).abs() < 1e-10);
}

#[test]
fn test_sqrt() {
    let r = parseExpression("sqrt(4)").unwrap();
    assert_eq!(r.result, 2.0);

    let r2 = parseExpression("sqrt(2+7)").unwrap();
    assert_eq!(r2.result, 3.0);
}

#[test]
fn test_log_invalid_arg() {
    let err = parseExpression("log(0)").unwrap_err();
    matches!(err, ParseError::InvalidExpression);

    let err2 = parseExpression("log(-5)").unwrap_err();
    matches!(err2, ParseError::InvalidExpression);
}

#[test]
fn test_sqrt_invalid_argument() {
    let err = parseExpression("sqrt(-1)").unwrap_err();
    matches!(err, ParseError::InvalidExpression);
}

#[test]
fn test_invalid_operator() {
    let err = parseExpression("1++2").unwrap_err();
    matches!(err, ParseError::InvalidExpression);

    let err2 = parseExpression("1*/2").unwrap_err();
    matches!(err2, ParseError::InvalidExpression);
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

#[test]
fn test_invalid_minus_after_minus() {
    let err = parseExpression("-(-1)").unwrap_err();
    matches!(err, ParseError::InvalidExpression);
}

#[test]
fn test_letters() {
    let err = parseExpression("abc").unwrap_err();
    matches!(err, ParseError::InvalidExpression);
}