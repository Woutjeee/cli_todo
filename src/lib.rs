use std::str::FromStr;
use clap::Parser;

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

#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub description: String
}
