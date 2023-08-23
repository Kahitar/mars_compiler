use std::fs;
use std::process::exit;

use mars::{argparse, tokenizer};

fn main() {
    let (infile, outfile) = argparse::parse_arguments();
    println!("IN: {}, OUT: {}", infile, outfile);

    let input_content = fs::read_to_string(infile).unwrap();
    println!("content: {:?}", input_content);

    let tokens = tokenizer::tokenize(input_content);
    println!("Found tokens: {:?}", tokens);

    exit(1)
}
