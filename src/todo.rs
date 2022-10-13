use std::{
    fs::OpenOptions,
    io::{Error, Write},
};

#[derive(Debug)]
pub struct Todo {
    pub task: String,
    pub done: bool,
}

impl Todo {
    pub fn create(task: String, done: bool) -> Todo {
        Todo { task, done }
    }

    pub fn save(&self) -> Result<(), Error> {
        let task = format!("{}{}{}", self.task, ":", self.done);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("todo.txt")
            .expect("Something went wrong opening the file");

        writeln!(file, "{}", task).expect("Something went wrong writing the file");
        Ok(())
    }
}
