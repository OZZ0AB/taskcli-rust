use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::{fs, path::Path, error::Error};

#[derive(Parser)]
#[command(name = "taskcli")]
#[command(about = "A simple Rust task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Done { index: usize },
}

#[derive(Serialize, Deserialize)]
struct Task {
    text: String,
    done: bool,
}

const FILE: &str = "tasks.json";

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut tasks = load_tasks()?;

    match &cli.command {
        Commands::Add { task } => {
            tasks.push(Task { text: task.clone(), done: false });
            println!("Added task: {}", task);
        }
        Commands::List => {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {} {}", i + 1, status, task.text);
            }
        }
        Commands::Done { index } => {
            if let Some(task) = tasks.get_mut(index - 1) {
                task.done = true;
                println!("Marked as done: {}", task.text);
            } else {
                println!("No task at index {}", index);
            }
        }
    }

    save_tasks(&tasks)?;
    Ok(())
}

fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    if Path::new(FILE).exists() {
        let data = fs::read_to_string(FILE)?;
        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE, data)?;
    Ok(())
}
