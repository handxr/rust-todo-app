use std::{
    fs::{write, OpenOptions},
    io,
    io::{BufRead, Error, Read},
};
use todo::Todo;

mod todo;

fn main() {
    let mut todo: Vec<Todo> = all_todo().expect("Something went wrong reading the file");
    println!("Tasks: {:?}", todo);

    loop {
        let stdin = io::stdin();

        println!("What do you want to do?");

        let action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "exit" {
            break;
        };

        println!("What is the task?");
        let task = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "show" => show_todo(&mut todo, task),
            "create" => create_todo(&mut todo, task),
            "complete" => complete_todo(&mut todo, task),
            "delete" => delete_todo(&mut todo, task),
            _ => println!("Invalid action"),
        }
    }
}

fn all_todo() -> Result<Vec<Todo>, Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("todo.txt")
        .expect("Something went wrong opening the file");
    let mut body = String::new();
    file.read_to_string(&mut body)
        .expect("Something went wrong reading the file");
    let mut list: Vec<Todo> = Vec::new();
    for line in body.lines() {
        // task:false
        let task = line.split(':').collect::<Vec<&str>>();
        list.push(Todo::create(task[0].to_string(), task[1].parse().unwrap()))
    }

    Ok(list)
}

fn show_todo(todo: &mut Vec<Todo>, status_task: String) {
    println!("List of tasks \n");
    for task in todo {
        let status = if !task.done { "Incomplete" } else { "Complete" };
        if status_task == "all" {
            println!("{} - {}", task.task, status);
        } else if status_task == "completed" && task.done {
            println!("{} - {}", task.task, status);
        } else if status_task == "todo" && !task.done {
            println!("{} - {}", task.task, status);
        }
    }
}

fn create_todo(todo: &mut Vec<Todo>, task: String) {
    let todo_instance = Todo::create(task, false);
    match todo_instance.save() {
        Ok(_) => {
            todo.push(todo_instance);
            println!("Task created successfully");
        }
        Err(error) => println!("Something went wrong --> {}", error),
    }
}

fn complete_todo(todo: &mut Vec<Todo>, arg_task: String) {
    let mut body = String::new();
    for task in todo {
        let task_done = if task.task == arg_task {
            true
        } else {
            task.done
        };
        let current_task = format!("{}:{}\n", task.task, task_done);
        task.done = task_done;
        body.push_str(&current_task);
    }

    match write("todo.txt", body) {
        Ok(_) => println!("Task completed successfully"),
        Err(error) => println!("Something went wrong -->  {}", error),
    }
}

fn delete_todo(todo: &mut Vec<Todo>, arg_task: String) {
    let mut body = String::new();
    for task in &mut todo.into_iter() {
        let current_task = format!("{}:{}\n", task.task, task.done);
        if task.task != arg_task {
            body.push_str(&current_task);
        }
    }

    todo.retain(|value| value.task != arg_task);

    match write("todo.txt", body) {
        Ok(_) => println!("Task deleted successfully"),
        Err(error) => println!("Something went wrong -->  {}", error),
    }
}
