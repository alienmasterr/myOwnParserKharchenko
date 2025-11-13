# arythemetic_expressions_parser_kharchenko_kma

## overview

this project implements a **command-line parser for simple arithmetic expressions** and handles expressions as such:

- basic arythmetics: `+`, `-`, `*`, `/`
- nested brackets: `((...))`
- negative numbers: `-x` or expressions like `-(1+2)`
- special functions: `log(x)` for natural logarithm and `sqrt(x)` for square root
- basic arythemetics inside special functions (`log` and `sqrt`): `log(10 + 5)`, `sqrt(2 + 7)`
- sequences of basic arythemetics: `((2 + 3) * 4)`
- `log` is a natural logorythm (ln) 

the parser evaluates input expressions and returns the result

## technical description

### parsing process

1. **preprocessing:**
   - remove outer brackets
   - handle minus at the beginning of expressions: `-(...)`
   - validate **balanced brackets**; invalid expressions get rejected

2. **expression evaluation:**
   - if the expression is a **single number**, it is returned as a result
   - if the expression contains arythmetic operators:
     - operators `*` and `/` are evaluated first
     - operators `+` and `-` are evaluated afterwards
   - brackets are recursively evaluated from inner to outer

3. **special functions:**
   - **natural logarythm (`log(x)`):**
     - evaluated only if `x > 0`, else -> error
     - returns `ln(x)` using `f64::ln()`
   - **square root (`sqrt(x)`):**
     - evaluated only if `x >= 0`, else -> error
     - returns `sqrt(x)` using `f64::sqrt()`

4. **error handling:**
   - expressions are validated for:
     - balanced brackets
     - valid numeric parsing
     - division by zero
     - domain errors for `log` and `sqrt`
     - acceptable characters (no random strings allowed)
   - invalid expressions returns `ParseError::InvalidExpression`

## grammar overview**

- expression ::= Term { ("+" | "-") Term }
- term       ::= Factor { ("*" | "/") Factor }
- factor     ::= Number | "(" Expression ")" | Function | "-" Factor
- function   ::= "log" "(" Expression ")" | "sqrt" "(" Expression ")"

#### link to crates.io : https://crates.io/crates/arythemetic_expressions_parser_kharchenko_kma
