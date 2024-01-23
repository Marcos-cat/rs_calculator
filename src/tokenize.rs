enum Token {
    Value(Value),
    Operator(Operator),
}

enum Value {
    Integer(i32),
    Float(f32),
}

enum Operator {
    Add,
    Subtract,
    Mutlipy,
    Divide,
}

pub struct Tokenizer {
    token_stream: Vec<Token>,
}

pub enum TokenizeError {}

type TokenizeResult = Result<Tokenizer, TokenizeError>;

impl Tokenizer {
    pub fn list_from(expression: String) -> TokenizeResult {
        let mut expression = expression;

        let mut token_stream = Vec::new();

        loop {
            let current_char = match expression.chars().next() {
                Some(c) => c,
                None => break,
            };

            if 

        }

        Ok(Tokenizer { token_stream })
    }
}

fn first_uninterrupted_number(input: &str) -> String {
    let mut result = String::new();
    let mut has_decimal = false;

    for c in input.chars() {
        if c.is_ascii_digit() || (c == '.' && !has_decimal) {
            result.push(c);
            if c == '.' {
                has_decimal = true;
            }
        } else if !result.is_empty() {
            break;
        }
    }

    result
}
