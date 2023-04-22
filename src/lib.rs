use std::io::Write;
use std::str::FromStr;
use std::fs::{self, File};
use std::{error, process};
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(short, long)]
    pub action: String,
    #[arg(short, long)]
    pub title: String,
    #[arg(short, long)]
    pub description: String
}

#[derive(Debug)]
pub enum Action {
    Add,
    Delete,
    Update,
    Get,
    GetAll
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Action, Self::Err> {
        match s {
            "add" | "a" => Ok(Action::Add),
            "delete" | "d" => Ok(Action::Delete),
            "update" | "u" => Ok(Action::Update),
            "get" | "g" => Ok(Action::Get),
            "getall" | "ga" => Ok(Action::GetAll),
            _ => Err(format!("Could not parse '{}' to an Action", s))
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub description: String
}

impl Todo {
    pub fn create_todo(title: String, description: String) -> Result<(), &'static str> {
        if title.is_empty() || description.is_empty() {
            return Err("Either the title or description is empty.");
        }

        let new_todo = Todo { title, description };
        let serialized = serde_json::to_string(&new_todo).unwrap();
        println!("serialized = {}", serialized);


        Ok(())
    }
}

pub fn file_exitis() -> Result<(), Box<dyn error::Error>> {
    let path = "C:/todo_app";
    let file_exists = fs::metadata(path).is_ok();
    
    if !file_exists {
        let file_path = "C:/todo_app/todos.json";
        fs::create_dir_all(path)?;
        let mut file = File::create(file_path)?;
        if let Err(e) = file.write_all(b"testingdspigjknhksD") {
            println!("Something went wrong: {e}");
            process::exit(1);
        }
    }

    Ok(())
}
