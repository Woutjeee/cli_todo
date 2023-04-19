use std::env;
use std::process;

use todo::Todo;

fn main() {
    let args: Vec<String> = env::args().collect();

    let todo = Todo::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Todo title: {}", todo.title);
    println!("Todo desc: {}", todo.description);
    println!("Todo prio: {:?}", todo.prio);
}
