use crate::scanner::Scanner;
use std::io::{self, Write};

pub mod error;
pub mod scanner;
pub mod token;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                let mut scanner = Scanner::new(line);

                match scanner.scan() {
                    Ok(tokens) => {
                        for token in tokens {
                            println!("{:?}", token);
                        }
                    }
                    Err(errs) => {
                        for err in errs {
                            eprintln!("\x1b[31m{}\x1b[0m", err);
                        }
                    }
                };
            }
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }
    }
}
