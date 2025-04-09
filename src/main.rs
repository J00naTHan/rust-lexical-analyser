pub mod lex;
pub mod token;

use lex::LexAnalyser;
use std::io::stdin;

fn main() {
    println!("\nRust Lexical Analyser!");
    println!("\nSUBMIT YOUR CODE!!!\n-----------------------------------");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("\nFailed to read the line!");
    println!("-----------------------------------");
    println!("\nProcessing your code...");

    let input = input.trim().to_string();
    let input: Vec<&str> = input.split("//n").collect();

    let lex = LexAnalyser::new(input);

    //let table = lex.token_table();
    //let list = lex.token_list();

    // debug results
    //println!("{}", table);
    //println!("{}", list);

    // debug :)
    for i in lex.code {
        println!(" separação: {} ", i);
    }
}
