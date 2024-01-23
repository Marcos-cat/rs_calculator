mod tokenize;

use std::io;
use tokenize::Tokenizer;

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let token_list = Tokenizer::list_from(line);

    Ok(())
}
