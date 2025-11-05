## arythemetic_expressions_parser_Kharchenko

this parser is designed to parse simple arythmetic expressions that have simple operators (+-*/) and numbers and show operators, operands and results of the operations

the examples of the expressions to be parsed are here:

1+2 ; 1+ 2 ; 1-2 ; 1/2 ; 1+(-2) ; 1-(-2) ; 1*(-2) ; 1/(-2) ; -(1+2) ; ((1+2))

the parser works in a way that it removes the extra brackets, changes operators' concatanations (exmpl -(-a) = a) and than devides the expression tooperators and operands and performs arythmetic operations afterwards. it also returns the operators and operands of the expression after the mentioned previously expression simplification.

Here is the main parsing logic:

pub fn parseExpression(s: &str) -> Result<ParseResult, ParseError> {

    if s.starts_with('(') && s.ends_with(')') {
        return parseExpression(&s[1..s.len()-1]);
    }

    if (s.contains("+(-") || s.contains("-(-") || s.contains("*(-")|| s.contains("/(-")) && s.ends_with(')') {

        let mut new_s = s.replace("+(-", "-");
        new_s = new_s.replace("-(-", "+");
        new_s = new_s.replace("*(-", "*-");
        new_s = new_s.replace("/(-", "/-");

        if new_s.ends_with(')') {
            new_s.pop();
        }

        return parseExpression(&new_s);
    }

    if s.starts_with("-(") && s.ends_with(')') {
        
        let inner = &s[2..s.len() - 1];
        let inner_result = parseExpression(inner)?;
    
        return Ok(ParseResult {
            result: -inner_result.result,
            operands: inner_result.operands,
            operators: inner_result.operators,
        });
    }

    if s.trim().is_empty() || !checkBalancedBrackets(s) {

        return Err(ParseError::InvalidExpression);

    } else if s.contains('+'){

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

link to crates.io : https://crates.io/crates/arythemetic_expressions_parser_Kharchenko
