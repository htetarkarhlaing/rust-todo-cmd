use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};
use::structopt::StructOpt;

#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, title: String) -> Todo {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    fn to_string(&self) -> String {
        if self.completed {
            format!("[x] {}", self.title)
        } else {
            format!("[ ] {}", self.title)
        }
    }
}

fn add_task(title: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todoList.txt")?;

    let id = get_next_id()?;
    let todo = Todo::new(id, title);
    writeln!(file, "{}", todo.to_string())?;
    println!("Task added: {}", todo.to_string());

    Ok(())
}

fn get_next_id() -> io::Result<u32> {
    let file = File::open("todoList.txt")?;
    let reader = BufReader::new(file);

    let mut next_id = 1;

    for _ in reader.lines() {
        next_id += 1;
    }
    Ok(next_id)
}

fn list_task() -> io::Result<()> {
    let file = File::open("todoList.txt")?;
    let reader = BufReader::new(file);

    println!("Todo list");
    for (index, line) in reader.lines().enumerate() {
        if let Ok(task) = line {
            println!("{}. {}", index + 1, task);
        }
    }
    Ok(())
}

fn complete_task(task_index: usize) -> io::Result<()> {
    let file = File::open("todoList.txt")?;
    let lines: Vec<_> = BufReader::new(file).lines().collect();

    if task_index > 0 && task_index <= lines.len() {
        let tasks = lines.into_iter().enumerate().map(|(i, task)| {
            if i == task_index - 1 {
                format!("[x] {}", task.unwrap())
            } else {
                task.unwrap()
            }
        });

        let mut output = File::create("todoList.txt")?;

        for task in tasks {
            writeln!(output, "{}", task)?;
        }

        println!("Task marked as completed!")
    } else {
        println!("Invalid task number");
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add")]
    Add {
        #[structopt(short, long)]
        title: String,
    },

    #[structopt( name = "list")]
    List,

    #[structopt( name = "complete")]
    Complete {
        #[structopt(short, long)]
        task_index: usize
    }
}

fn main() -> io::Result<()> {
    let opt = Command::from_args();

    match opt {
        Command::Add { title } => add_task(title),
        Command::List => list_task(),
        Command::Complete { task_index } => complete_task(task_index)
    }
}
