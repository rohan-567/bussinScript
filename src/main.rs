mod lexer;
mod parser;

use std::env;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len()>2{
        println!("Invalid argument count");
        exit(132);
    }
    else if args.len()==2 {
        lexer::run_file(&args[1]);

    }














}
