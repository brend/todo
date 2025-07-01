use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
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

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found");
        } else {
            for task in &self.tasks {
                println!(
                    "{}: [{}] {}",
                    task.id,
                    if task.completed { "X" } else { " " },
                    task.title
                );
            }
        }
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
        }
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
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

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let filename = "todo.json";
    let mut todo_list = TodoList::load_from_file(filename)?;

    match cli.command {
        Commands::Add { title } => {
            todo_list.add_task(title);
            todo_list.save_to_file(filename)?;
        }
        Commands::List => {
            todo_list.list_tasks();
        }
        Commands::Complete { id } => {
            todo_list.complete_task(id);
            todo_list.save_to_file(filename)?;
        }
        Commands::Remove { id } => {
            todo_list.remove_task(id);
            todo_list.save_to_file(filename)?;
        }
    }

    Ok(())
}
