use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid expression")]
    InvalidExpression,
}

#[derive(Debug)]
pub struct ParseResult {
    pub result: f64,
    pub operands: Vec<f64>,
    pub operators: Vec<char>,
}

fn checkBalancedBrackets(s: &str) -> bool {
    let mut balance = 0;
    for c in s.chars() {
        if c == '(' {
            balance += 1;
        } else if c == ')' {
            balance -= 1;
            if balance < 0 {
                return false;
            }
        }
    }
    balance == 0
}

    //rule 0: handle (x)
    // if s.starts_with('(') && s.ends_with(')') {
    //     return parseExpression(&s[1..s.len()-1]);
    // }
fn remove_outer_brackets(s: &str) -> Option<&str> {
    if s.starts_with('(') && s.ends_with(')'){
        return Some(&s[1..s.len() - 1]);
    }else{
        return None;
    }
}

//rule 1 : handle +(-x), -(-x), *(-x), /(-x)
fn has_minus(s: &str) -> bool {
    return (s.contains("+(-") || s.contains("-(-") || s.contains("*(-") || s.contains("/(-")) && s.ends_with(')');
}
//rule 1 : handle +(-x), -(-x), *(-x), /(-x)
fn simplify_minus(s: &str) -> String {
    let mut new_s = s.replace("+(-", "-");
        new_s = new_s.replace("-(-", "+");
        new_s = new_s.replace("*(-", "*-");
        new_s = new_s.replace("/(-", "/-");

        if new_s.ends_with(')') {
            new_s.pop();
        }

    return new_s;
}

//rule 1: handle -(-x)
fn has_minus_at_start(s: &str) -> bool {
    return s.starts_with("-(") && s.ends_with(')');
}

//rule 1: handle -(-x)
fn handle_minus_at_start(s: &str) -> Result<ParseResult, ParseError> {
    let inner = &s[2..s.len() - 1];
    let inner_result = parseExpression(inner)?;
    
        return Ok(ParseResult {
            result: -inner_result.result,
            operands: inner_result.operands,
            operators: inner_result.operators,
        });
}

// rule 2: handle +, -, *, /
fn pasre_simple_expression(s: &str) -> Result<ParseResult, ParseError> {

    if s.contains('+'){

        let parts: Vec<&str> = s.split('+').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidExpression);
        }

        let left: f64 = parts[0].trim().parse().map_err(|_| ParseError::InvalidExpression)?;
        let right: f64 = parts[1].trim().parse().map_err(|_| ParseError::InvalidExpression)?;

        Ok(ParseResult {
            result: left + right,
            operands: vec![left, right],
            operators: vec!['+'],
        })

    } else if s.contains('*'){

        let parts: Vec<&str> = s.split('*').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidExpression);
        }

        let left: f64 = parts[0].trim().parse().map_err(|_| ParseError::InvalidExpression)?;
        let right: f64 = parts[1].trim().parse().map_err(|_| ParseError::InvalidExpression)?;

        Ok(ParseResult {
            result: left * right,
            operands: vec![left, right],
            operators: vec!['*'],
        })

    } else if s.contains('/') {

        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(ParseError::InvalidExpression);
        }

        let left: f64 = parts[0].trim().parse().map_err(|_| ParseError::InvalidExpression)?;
        let right: f64 = parts[1].trim().parse().map_err(|_| ParseError::InvalidExpression)?;

        if right == 0.0 {
            return Err(ParseError::InvalidExpression);
        }

        Ok(ParseResult {
            result: left / right,
            operands: vec![left, right],
            operators: vec!['/'],
        })

    } else if s.contains('-') {

    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidExpression);
    }

    let left: f64 = parts[0].trim().parse().map_err(|_| ParseError::InvalidExpression)?;
    let right: f64 = parts[1].trim().parse().map_err(|_| ParseError::InvalidExpression)?;

    Ok(ParseResult {
        result: left - right,
        operands: vec![left, right],
        operators: vec!['-'],
    })

    } else {
    Err(ParseError::InvalidExpression)
    }
}

// the function to parse expression
pub fn parseExpression(s: &str) -> Result<ParseResult, ParseError> {
    if let Some(inner) = remove_outer_brackets(s){
        return parseExpression(inner);
    }

    if has_minus(s) {
        let simplified = simplify_minus(s);
        return parseExpression(&simplified);
    }

    if has_minus_at_start(s) {
        return handle_minus_at_start(s);
    }

    if s.trim().is_empty() || !checkBalancedBrackets(s) {
        return Err(ParseError::InvalidExpression);
    } else{
        return pasre_simple_expression(s);
    }
}