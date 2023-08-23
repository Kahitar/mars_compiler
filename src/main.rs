use std::fs;

use mars::argparse;

fn main() {
    let (infile, outfile) = argparse::parse_arguments();
    println!("IN: {}, OUT: {}", infile, outfile);

    let input_content = fs::read_to_string(infile).unwrap();
    println!("content: {:?}", input_content);
}
