use clap::{Parser, Subcommand};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "todo", about = "A simple todo list manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        title: String,
        #[arg(short, long, default_value_t = false)]
        quiet: bool,
    },
    List,
    Complete {
        id: usize,
    },
    Remove {
        id: usize,
    },
    Info,
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

    fn add_task(&mut self, title: String) -> usize {
        let id = self.next_id;
        self.tasks.push(Task {
            id,
            title,
            completed: false,
        });
        self.next_id += 1;
        id
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found");
        } else {
            for task in &self.tasks {
                println!(
                    "{}\t[{}]\t{}",
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

    fn save_to_file(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load_from_file(path: &Path) -> io::Result<Self> {
        if path.exists() {
            let json = fs::read_to_string(path)?;
            let todo_list = serde_json::from_str(&json)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::new())
        }
    }
}

fn get_todo_file_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;
    let todo_dir = config_dir.join("todo");
    Ok(todo_dir.join("todo.json"))
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let todo_file_path = get_todo_file_path()?;
    let mut todo_list = TodoList::load_from_file(&todo_file_path)?;

    match cli.command {
        Commands::Add { title, quiet } => {
            let task_id = todo_list.add_task(title);
            if !quiet {
                println!("{}", task_id);
            }
            todo_list.save_to_file(&todo_file_path)?;
        }
        Commands::List => {
            todo_list.list_tasks();
        }
        Commands::Complete { id } => {
            todo_list.complete_task(id);
            todo_list.save_to_file(&todo_file_path)?;
        }
        Commands::Remove { id } => {
            todo_list.remove_task(id);
            todo_list.save_to_file(&todo_file_path)?;
        }
        Commands::Info => {
            println!("data file: {}", todo_file_path.display());
        }
    }

    Ok(())
}
