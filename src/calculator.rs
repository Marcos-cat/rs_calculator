#[derive(Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),

            _ => Err(format!("Not a valid Operator: {value}")),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    Value(f32),
}

type Error = String;

/// # Errors
/// Returns an error if the input string is not a valid expression
pub fn parse(line: &str) -> Result<Vec<Token>, Error> {
    let line = line.trim();
    let str_tokens = line.split_whitespace();

    let tokens: Result<Vec<Token>, Error> =
        str_tokens
            .clone()
            .try_fold(Vec::new(), |mut acc, str_token| {
                if let Ok(value) = parse_number(str_token) {
                    acc.push(Token::Value(value));
                    return Ok(acc);
                }

                if let Ok(operator) = str_token.try_into() {
                    acc.push(Token::Operator(operator));
                    return Ok(acc);
                }

                Err(format!(
                    "Token does not match any token types: '{str_token}'"
                ))
            });

    if let Ok(tokens) = &tokens {
        println!("{tokens:?}");
    }

    tokens
}

fn parse_number(string: &str) -> Result<f32, Error> {
    match string.trim().parse() {
        Ok(n) => Ok(n),
        Err(err) => Err(err.to_string()),
    }
}

/// # Errors
/// Returns an `Err` variant when the tokens are not in a valid order
pub fn eval(tokens: &[Token]) -> Result<f32, Error> {
    let Token::Value(x) = tokens[0] else {
        return Err("urmom".to_string());
    };

    let Token::Operator(operator) = &tokens[1] else {
        return Err("urmom".to_string());
    };

    let Token::Value(y) = tokens[2] else {
        return Err("urmom".to_string());
    };

    let result = match operator {
        Operator::Addition => x + y,
        Operator::Subtraction => x - y,
        Operator::Multiplication => x * y,
        Operator::Division => {
            if y == 0.0 {
                return Err("Cannot divide by zero!".to_string());
            }
            x / y
        }
    };

    Ok(result)
}
