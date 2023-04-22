use clap::Parser;
use todo::Cli;
use std::process;


fn main() {
    let args = Cli::parse();
    let action: todo::Action = args.action.parse().unwrap_or_else(|err| {
        // ?? TODO: Maybe make a loop that a user can keep entering might be annoying tho.
        println!("{err}");
        process::exit(1);
    });

    println!("{:?}", action);
}
