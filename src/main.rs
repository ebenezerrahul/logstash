mod commons;
mod parser;
mod sink;
mod utils;
use std::io;

fn main() {
    let mut read = String::new();
    loop {
        let result = io::stdin().read_line(&mut read);
        match result {
            Ok(ok) => {
                println!("{}", read);
                println!("ok {}", ok);
            }
            Err(_err) => println!("err"),
        }
        read.clear();
    }
}
