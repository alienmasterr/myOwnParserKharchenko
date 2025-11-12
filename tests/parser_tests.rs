use arythemetic_expressions_parser_Kharchenko::ParseError;
use arythemetic_expressions_parser_Kharchenko::parseExpression;
use anyhow::{Result, anyhow};

#[test]
fn test_single() -> Result<()>{
    let r = parseExpression("1")?;
    assert_eq!(r.result, 1.0);
    Ok(())
}

#[test]
fn test_single_neg() -> Result<()>{
    let r = parseExpression("-(1)")?;
    assert_eq!(r.result, -1.0);
    Ok(())
}

#[test]
fn test_simple_add() -> Result<()>{
    let r = parseExpression("1+2")?;
    assert_eq!(r.result, 3.0);
    Ok(())

}

#[test]
fn test_space() -> Result<()>{
    let r = parseExpression("1+ 2")?;
    assert_eq!(r.result, 3.0);
    Ok(())

}

#[test]
fn test_simple_minus() -> Result<()>{
    let r = parseExpression("1-2")?;
    assert_eq!(r.result, -1.0);
    Ok(())

}

#[test]
fn test_simple_mul()-> Result<()> {
    let r = parseExpression("1*2")?;
    assert_eq!(r.result, 2.0);
    Ok(())

}

#[test]
fn test_simple_div() -> Result<()>{
    let r = parseExpression("1/2")?;
    assert_eq!(r.result, 0.5);
    Ok(())

}

#[test]
fn test_add_negative()-> Result<()> {
    let r = parseExpression("1+(-2)")?;
    assert_eq!(r.result, -1.0);
    Ok(())

}

#[test]
fn test_minus_negative()-> Result<()> {
    let r = parseExpression("1-(-2)")?;
    assert_eq!(r.result, 3.0);
    Ok(())

}

#[test]
fn test_mul_negative() -> Result<()>{
    let r = parseExpression("1*(-2)")?;
    assert_eq!(r.result, -2.0);
    Ok(())

}

#[test]
fn test_div_negative() -> Result<()>{
    let r = parseExpression("1/(-2)")?;
    assert_eq!(r.result, -0.5);
    Ok(())

}

#[test]
fn test_minus_whole_expr() -> Result<()>{
    let r = parseExpression("-(1+2)")?;
    assert_eq!(r.result, -3.0);
    Ok(())

}

#[test]
fn test_nested_brackets() -> Result<()>{
    let r = parseExpression("((1+2))")?;
    assert_eq!(r.result, 3.0);
    Ok(())
}

#[test]
fn test_complex_expr() -> Result<()>{
    let r = parseExpression("1+2*3-4/5")?;
    assert_eq!(r.result, 6.2);
    Ok(())
}

#[test]
fn test_log() -> Result<()>{
    let r = parseExpression("log(1)")?;
    assert_eq!(r.result, 0.0);

    let r2 = parseExpression("log(1+2)")?;
    assert!((r2.result - (1.0f64 + 2.0f64).ln()).abs() < 1e-10);
    Ok(())
}

#[test]
fn test_sqrt() -> Result<()>{
    let r = parseExpression("sqrt(4)")?;
    assert_eq!(r.result, 2.0);

    let r2 = parseExpression("sqrt(2+7)")?;
    assert_eq!(r2.result, 3.0);
    Ok(())
}

#[test]
fn test_log_invalid_arg()-> Result<()> {
    match parseExpression("log(0)") {
        Ok(_) => { return Err(anyhow!("Expected InvalidExpression error")); }
        Err(e) => { assert!(matches!(e, ParseError::InvalidExpression)); }
    }

    match parseExpression("log(-5)") {
        Ok(_) => { return Err(anyhow!("Expected InvalidExpression error")); }
        Err(e) => { assert!(matches!(e, ParseError::InvalidExpression)); }
    }

    Ok(())
}

#[test]
fn test_sqrt_invalid_argument() -> Result<()>{
    match parseExpression("sqrt(-1)") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }
    Ok(())
}

#[test]
fn test_invalid_operator()-> Result<()> {
    match parseExpression("1++2") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }

    match parseExpression("1*/2") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }
    Ok(())
}

#[test]
fn test_invalid_empty()-> Result<()> {
    match parseExpression("") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }
    Ok(())
}

#[test]
fn test_invalid_unbalanced_brackets()-> Result<()> {
    match parseExpression("(1+2") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }
    Ok(())
}

#[test]
fn test_invalid_minus_after_minus()-> Result<()> {
    match parseExpression("-(-1)") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }
    Ok(())
}

#[test]
fn test_letters()-> Result<()> {
    match parseExpression("abc") {
        Ok(_) => {return Err(anyhow!("Expected InvalidExpression error"));}
        Err(e)=> {assert!(matches!(e, ParseError::InvalidExpression));}
    }
    Ok(())
}