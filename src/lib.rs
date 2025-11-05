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

pub fn parseExpression(s: &str) -> Result<ParseResult, ParseError> {

    if s.trim().is_empty() || !checkBalancedBrackets(s) {

        return Err(ParseError::InvalidExpression);

    } else if s.contains('-'){

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

    } else if s.contains('+') {

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

    } else {

    Err(ParseError::InvalidExpression)

    }
}
