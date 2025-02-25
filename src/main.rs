use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

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
    Vec::new()
}

fn save_tasks(tasks: &Vec<Task>) {
}

fn add_task(description: String, date: String) {
}

fn list_tasks() {
}

fn toggle_task(id: u32) {
}

fn tasks_by_date(date: String) {
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
