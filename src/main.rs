use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "todo", about = "A simple todo list manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        title: String,
    },
    List,
    Complete {
        #[arg(short, long)]
        id: usize,
    },
    Remove {
        #[arg(short, long)]
        id: usize,
    },
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        self.tasks.push(Task {
            id: self.next_id,
            title,
            completed: false,
        });
        self.next_id += 1;
    }

    fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn remove_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(format!("Task with ID {} not found", id))
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        if Path::new(filename).exists() {
            let json = fs::read_to_string(filename)?;
            let todo_list = serde_json::from_str(&json)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::new())
        }
    }
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
