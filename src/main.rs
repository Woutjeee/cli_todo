use clap::Parser;
use todo::Cli;
use std::process;


fn main() {
    let args = Cli::parse();
    
    if let Err(e) = todo::file_exitis(){
        println!("App error: {e}");
        process::exit(1);
    }

    let action: todo::Action = args.action.parse().unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    
    todo::Todo::create_todo(args.title, args.description);

    println!("{:?}", action);
}
