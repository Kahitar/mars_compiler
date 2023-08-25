use std::process::exit;
use std::{fs, process::Command};

use mars::{argparse, parser, tokenizer};

fn main() {
    let (infile, outfile) = argparse::parse_arguments();
    println!("IN: {}, OUT: {}", infile, outfile);

    let input_content = fs::read_to_string(infile).unwrap();
    println!("content: {:?}", input_content);

    let tokens = tokenizer::tokenize(input_content);
    println!("Found tokens: {:?}", tokens);

    let assembly = parser::parse(tokens);
    let assembly = assembly.unwrap();
    println!("Generated assembly:\n\n{}", assembly);

    fs::write(outfile, assembly).unwrap();

    let status = Command::new("nasm").arg("-felf64").arg("test.asm").status();
    if status.is_err() {
        eprintln!("Nasm failed with error: {}", status.unwrap_err());
    }

    let status = Command::new("ld")
        .arg("test.o")
        .arg("-o")
        .arg("test")
        .status();
    if status.is_err() {
        eprintln!("ld failed with error: {}", status.unwrap_err());
    }

    exit(0)
}
