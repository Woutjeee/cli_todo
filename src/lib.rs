use std::str::FromStr;
use std::error::Error;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub pattern: String,
    //pub path: std::path::PathBuf
}


#[derive(Debug, PartialEq)]
pub enum Operation {
    Get,
    GetAll,
    Add,
    Update,
    Delete
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "get" => Ok(Operation::Get),
            "getall" => Ok(Operation::GetAll),
            "add" => Ok(Operation::Add),
            "update" => Ok(Operation::Update),
            "delete" => Ok(Operation::Delete),
            _ => Err(()),
        }
    }
}

impl Operation {
    fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
        // TODO: Get action user wants to do.
        // TODO: Check length of args

        Ok(())
     }
    
    pub fn determine_action(action: &Operation) {
        match action {
            Operation::Get => println!("Getting todos..."),
            Operation::Add => println!("Adding todo..."),
            _ => panic!("Something went wrong..."),
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High
}

impl FromStr for Priority {
    type Err = ();
    fn from_str(s: &str) -> Result<Priority, Self::Err> {
        match s {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(()),
        }
    }
}

pub struct Todo {
    pub title: String,
    pub description: String,
    pub prio: Priority
}

impl Todo {
    pub fn new(args: &[String]) -> Result<Todo, &'static str> {
        if args.len() < 4 {
            return Err("Not enough parameters");
        }
        
        let title = args[1].clone();
        let description = args[2].clone();
        let prio = Priority::from_str(&args[3].clone()).unwrap();

        println!("Creating new todo.");
        Ok(Todo {title, description, prio})
    }
}
