use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    date: String, // YYYY-MM-DD
    completed: bool,
}

#[derive(Parser)]
#[command(name = "task-cli")]
#[command(about = "A simple CLI for managing tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String, date: String },
    List,
    Toggle { id: u32 },
    Filter { date: String },
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}


fn save_tasks(tasks: &Vec<Task>) {
    if let Ok(json) = serde_json::to_string_pretty(tasks) {
        fs::write(FILE_PATH, json).expect("Failed to write file");
    }
}

fn add_task(description: String, date: String) {
    let mut tasks = load_tasks();
    let id = tasks.len() as u32 + 1;
    tasks.push(Task { id, description, date, completed: false });
    save_tasks(&tasks);
    println!("Task added successfully!");
}

fn list_tasks() {
    let tasks = load_tasks();
    for task in &tasks {
        let status = if task.completed { "[X]" } else { "[ ]" };
        println!("{}. {} {} - {}", task.id, status, task.date, task.description);
    }
}

fn toggle_task(id: u32) {
}

fn tasks_by_date(date: String) {
    let tasks = load_tasks();
    for task in tasks.iter().filter(|t| t.date == date) {
        let status = if task.completed { "[X]" } else { "[ ]" };
        println!("{}. {} {} - {}", task.id, status, task.date, task.description);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description, date } => add_task(description, date),
        Commands::List => list_tasks(),
        Commands::Toggle { id } => toggle_task(id),
        Commands::Filter { date } => tasks_by_date(date),
    }
}
