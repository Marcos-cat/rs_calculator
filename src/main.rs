pub mod calculator;

fn main() {
    /* let mut line = String::new();
    io::stdin().read_line(&mut line)?; */

    let line = "14+9";

    let Ok(tokens) = calculator::parse(line) else {
        eprintln!("Parse failed!");
        std::process::exit(1)
    };

    let Ok(result) = calculator::eval(&tokens)  else {
        eprintln!("Eval failed!");
        std::process::exit(1)
    };

    println!("{result:?}");
}
