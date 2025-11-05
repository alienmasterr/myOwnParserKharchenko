// peg::parser! {
//     pub grammar list_parser() for str {
//       rule number() -> u32
//         = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }
  
//       pub rule list() -> Vec<u32>
//         = "[" l:(number() ** ",") "]" { l }
//     }
//   }

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid expression")]
    InvalidExpression,
}

pub fn parse_expression(s: &str) -> Result<f64, ParseError> {

    let parts: Vec<&str> = s.split('+').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidExpression);
    }

    let left: f64 = parts[0].trim().parse().map_err(|_| ParseError::InvalidExpression)?;
    let right: f64 = parts[1].trim().parse().map_err(|_| ParseError::InvalidExpression)?;

    Ok(left + right)
}
