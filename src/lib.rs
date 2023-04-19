use std::str::FromStr;

pub struct Todo {
    pub title: String,
    pub description: String,
    pub prio: Priority
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
