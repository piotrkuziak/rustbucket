use std::io::{self, Write};
use anyhow::{Result, anyhow};

use rustbucket::task::Task;
use rustbucket::task::Status;
use rustbucket::database::Database;
use clearscreen;

macro_rules! carret {
    () => {
        print!("    > ");
    };
}

macro_rules! read_line {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }};
}

const LOGO: &str = r#"
        ____             __     ____             __        __ 
       / __ \__  _______/ /_   / __ )__  _______/ /_____  / /_
      / /_/ / / / / ___/ __/  / __  / / / / ___/ //_/ _ \/ __/
     / _, _/ /_/ (__  ) /_   / /_/ / /_/ / /__/ ,< /  __/ /_  
    /_/ |_|\__,_/____/\__/  /_____/\__,_/\___/_/|_|\___/\__/  
"#;

fn intro() -> Result<()> {
    let message = r#"
    1. Add task
    2. Edit task
    3. Delete task
    4. List tasks
    5. Clear tasks
    6. Exit                                       
    "#;

    clearscreen::clear()?;
    println!("{}", LOGO);
    println!("{}", message);

    Ok(())
}

fn add_task() -> Result<()> {
    clearscreen::clear()?;
    println!("{}", LOGO);
    println!("    Task name:");
    carret!();
    io::stdout().flush()?;

    let input = read_line!();

    let task = Task::new(0, input.trim().to_string());

    match Database::open() {
        Ok(db) => {
            db.create_task(task)?;
            Ok(())
        },
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
            Err(anyhow!(err))
        }
    }
}

fn edit_task() -> Result<()> {
    clearscreen::clear()?;
    println!("{}", LOGO);

    // List all tasks
    match Database::open() {
        Ok(db) => {
            if let Ok(tasks) = db.get_tasks() {
                for task in tasks {
                    println!("{}", task);
                }
            } else {
                eprintln!("An error occurred while getting tasks from DB");
            }
        },
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
        }
    }

    println!("    Task ID:");
    carret!();
    io::stdout().flush()?;

    let id: String = read_line!();

    let parsed_id: usize = match id.parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Failed to parse ID");
            0
        }
    };

    clearscreen::clear()?;
    println!("{}", LOGO);

    println!(r#"
    New status:
    1. To do
    2. In Progress
    3. Done
    "#);
    carret!();
    io::stdout().flush()?;

    let new_status: String = read_line!();

    let parsed_new_status: Status = match new_status.as_str() {
        "1" => Status::ToDo,
        "2" => Status::InProgress,
        "3" => Status::Done,
        _ => Status::Unknown
    };

    // Update task in database
    match Database::open() {
        Ok(db) => {
            db.update_task(parsed_id, parsed_new_status)?;
            Ok(())
        },
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
            Err(anyhow!(err))
        }
    }
}

fn list_tasks() -> Result<()> {
    clearscreen::clear()?;
    println!("{}", LOGO);

    match Database::open() {
        Ok(db) => {
            if let Ok(tasks) = db.get_tasks() {
                for task in tasks {
                    println!("{}", task);
                }
            } else {
                eprintln!("An error occurred while getting tasks from DB");
            }
        },
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
        }
    }

    let _ = read_line!();

    Ok(())
}

fn delete_task() -> Result<()> {
    clearscreen::clear()?;
    println!("{}", LOGO);

    // List all tasks
    match Database::open() {
        Ok(db) => {
            if let Ok(tasks) = db.get_tasks() {
                for task in tasks {
                    println!("{}", task);
                }
            } else {
                eprintln!("An error occurred while getting tasks from DB");
            }
        },
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
        }
    }

    println!("    Task ID:");
    carret!();
    io::stdout().flush()?;

    let id: String = read_line!();

    let parsed_id: usize = match id.parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Failed to parse ID");
            0
        }
    };

    // Delete task in database
    match Database::open() {
        Ok(db) => {
            db.delete_task(parsed_id)?;
            Ok(())
        },
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
            Err(anyhow!(err))
        }
    }
}

fn delete_all_tasks() -> Result<()> {
    match Database::open() {
        Ok(db) => Ok(db.delete_all_tasks()?),
        Err(err) => {
            eprintln!("Failed to open database: {:?}", err);
            Err(anyhow!(err))
        }
    }
}

fn main() -> Result<()> {
    loop {
        intro()?;
        carret!();
        io::stdout().flush().expect("Failed to flush stdout");

        let input = read_line!();

        let input_word = input.trim().to_lowercase();

        match input_word.as_str() {
            "1" => add_task()?,
            "2" => edit_task()?,
            "3" => delete_task()?,
            "4" => list_tasks()?,
            "5" => delete_all_tasks()?,
            "6" => break,
            _ => println!("You entered something else")
        }
    }

    Ok(())
}