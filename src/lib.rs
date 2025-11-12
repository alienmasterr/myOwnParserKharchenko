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

    //rule 0: handle ((x))
fn remove_outer_brackets(s: &str) -> Option<&str> {
    if s.starts_with('(') && s.ends_with(')'){
        let mut balance = 0;
        let mut i = 0;
        let mut chars = s.chars();

        while let Some(c) = chars.next(){
            if c == '(' {
                balance += 1;
            } else if c == ')' {
                balance -= 1;
            }

            if balance == 0 && i < s.len() - 1{
                return None;
            }
            i += 1;
        }

        if balance == 0 {
            return Some(&s[1..s.len()-1]);
        }

    }
        return None;
}

fn has_minus(s: &str) -> bool {
    return (s.contains("+(-") || s.contains("-(-") || s.contains("*(-") || s.contains("/(-")) && s.ends_with(')');
}

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

fn has_minus_at_start(s: &str) -> bool {
    return s.starts_with("-(") && s.ends_with(')');
}

fn handle_minus_at_start(s: &str) -> Result<ParseResult, ParseError> {
    let inner = &s[2..s.len() - 1];
    let inner_result = parseExpression(inner)?;
    
        return Ok(ParseResult {
            result: -inner_result.result,
            operands: inner_result.operands,
            operators: inner_result.operators,
        });
}

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

fn handle_long_expression(s: &str) -> Result<ParseResult, ParseError> {
    let mut nums = Vec::new();
    let mut ops = Vec::new();
    let mut current_num = String::new();

    let mut k = 0;
    let chars: Vec<char> = s.chars().collect();

    while k<chars.len(){
        let i = chars[k];

        if i.is_whitespace(){
            k += 1;
            continue;
        }

        //rule 1: handle (x)
        if i=='('{
            let mut d = 1;
            let mut start = k + 1;
            while start<chars.len() && d>0 {
                if chars[start]=='('{
                    d += 1;
                } else if chars[start]==')'{
                    d -= 1;
                }
                start += 1;
            }

            if d != 0 {
                return Err(ParseError::InvalidExpression);
            }

            let inside = &s[k+1..start-1];
            let inside_res = handle_long_expression(inside)?;
            // nums.push(inside_res.result);
            // current_num.clear();
            current_num.push_str(&inside_res.result.to_string());
            k=start;
            continue;
        }

        // rule 2: handle operators
        if i=='+' || i=='-' || i=='*' || i=='/'{

            if current_num.trim().is_empty() {
                if i == '-' {
                    current_num.push(i);
                    k += 1;
                    continue;
                } else {
                    return Err(ParseError::InvalidExpression);
                }

            }

                let num: f64 = current_num.parse().map_err(|_| ParseError::InvalidExpression)?;
                nums.push(num);
                ops.push(i);
                current_num.clear();

        } else {
            current_num.push(i);
        }

        k += 1;
    }

    if current_num.trim().is_empty() {
        return Err(ParseError::InvalidExpression);
    }

    nums.push(current_num.trim().parse().map_err(|_| ParseError::InvalidExpression)?);

    //rule 3: evaluate expression
    // handle * and / first
    let mut j = 0;
    while j<ops.len() {
        if ops[j]=='*' || ops[j]=='/'{
            let left = nums[j];
            let right = nums[j+1];
            let mut res:f64=0.0;
            if ops[j]=='*'{
                res = left * right;
            } else {
                if right == 0.0 {
                    return Err(ParseError::InvalidExpression);
                }
                res = left / right;
            }
            nums[j] = res;
            nums.remove(j+1);
            ops.remove(j);
        } else {
            j += 1;
        }
    }

    // then handle + and -
    let mut result = nums[0];
    for k in 0..ops.len(){
        let right = nums[k+1];
        if ops[k]== '+' {
            result += right;
        } else if ops[k]== '-' {
            result -= right;
        }else{
            return Err(ParseError::InvalidExpression);
        }
    }

    Ok(ParseResult {
        result,
        operands: nums,
        operators: ops,
    })

}

//rule 4: handle log(x)
fn handle_log(s: &str) -> Result<ParseResult, ParseError> {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix("log(").and_then(|s| s.strip_suffix(")")){
        let inner_res = parseExpression(inner)?;
        if inner_res.result <= 0.0{
            return Err(ParseError::InvalidExpression);
        }
        Ok(ParseResult {
            result: inner_res.result.ln(),
            operands: inner_res.operands,
            operators: inner_res.operators,
        })
    }else {
        Err(ParseError::InvalidExpression)
    }
}

//rule 5: handle sqrt(x)
fn handle_sqrt(s: &str) -> Result<ParseResult, ParseError> {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix("sqrt(").and_then(|s| s.strip_suffix(")")) {
        let inner_res = parseExpression(inner)?;
        if inner_res.result < 0.0{
            return Err(ParseError::InvalidExpression);
        }
        Ok(ParseResult {
            result: inner_res.result.sqrt(),
            operands: inner_res.operands,
            operators: inner_res.operators,
        })
    }else {
        Err(ParseError::InvalidExpression)
    }
}


// the function to parse expression
pub fn parseExpression(s: &str) -> Result<ParseResult, ParseError> {
    let s = s.trim();

    if let Some(inner) = remove_outer_brackets(s){
        return parseExpression(inner);
    }

    // if has_minus(s) {
    //     let simplified = simplify_minus(s);
    //     return parseExpression(&simplified);
    // }

    // if has_minus_at_start(s) {
    //     return handle_minus_at_start(s);
    // }
    

    if s.trim().is_empty() || !checkBalancedBrackets(s) {
        return Err(ParseError::InvalidExpression);
    } else{

        if let Ok(num) = s.parse::<f64>() {
            return Ok(ParseResult {
                result: num,
                operands: vec![num],
                operators: vec![],
            });
        }

        if s.starts_with("log(") && s.ends_with(")") {
            return handle_log(s);
        } 
        
        if s.starts_with("sqrt(") && s.ends_with(")") {
            return handle_sqrt(s);
        }
    
        if s.contains('+') || s.contains('-') || s.contains('*') || s.contains('/') {
            return handle_long_expression(s);
        }
    }

    return Err(ParseError::InvalidExpression);
}