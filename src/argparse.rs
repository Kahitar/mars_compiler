pub fn parse_arguments() -> (String, String) {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        print_usage_and_exit();
    }
    (args[0].clone(), args[1].clone())
}

pub fn print_usage_and_exit() {
    eprintln!("Incorrect usage. Correct ussage is...");
    eprintln!("  <infile> <outfile>");
    std::process::exit(-1);
}
