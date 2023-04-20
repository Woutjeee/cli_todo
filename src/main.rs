use std::env;
use clap::Parser;
use todo::Cli;

fn main() {
    //let args: Vec<String> = env::args().collect();
    let args = Cli::parse();

    println!("{:?}", args.pattern);
}
