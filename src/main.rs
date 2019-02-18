use std::env;
use std::process;

use postfixcalccli;
use postfixcalccli::Expression;

fn main() {
    let expression = Expression::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match expression.evaluate() {
        Ok(val) => println!("{}", val),
        Err(e) => {
            eprintln!("Error evaluating expression: {}", e);
            process::exit(1);
        }
    }
}
