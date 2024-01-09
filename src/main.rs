use std::{
    fmt,
    io
};

#[derive(Debug)]
enum Command {
    Add(ToDo),
    Remove(usize),
    Show,
    Help,
    None,
}

#[derive(Debug)]
struct ToDo {
    description: String,
}

impl fmt::Display for ToDo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

static HELP_TEXT: &str = r#"+---------------------------+
List of commands and syntax:
    add <list item>
        (adds a new item to the todo list)
    remove <# index>
        (removes the item at # index)
    show
        (prints contents of list)
    help
        (prints this list of commands and syntax)
+---------------------------+"#;

fn parse_command() -> Command {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("failed to read from stdin");

    let mut args = line.trim().split_whitespace();

    let command: &str;
    if let Option::Some(cmd) = args.next() {
        command = cmd;
    } else {
        println!("Error parsing command from args");
        return Command::None;
    }

    let args = args.collect::<Vec<&str>>().join(" ");

    match command {
        "help" => return Command::Help,
        "show" => return Command::Show,
        "add" => return Command::Add(ToDo{description: args}),
        "remove" => match args.parse::<usize>() {
            Ok(index) => Command::Remove(index),
            Err(_) => {
                println!("Problem parsing \"{}\" as number", args);
                return Command::None;
            }
        },
        _ => {
            println!("No such command {}", command);
            return Command::None;
        },
    }
}

fn main() {
    let mut todo_list: Vec<ToDo> = Vec::new();

    println!("Welcome to the Todo List! Type \"help\" to get started...");
    
    loop {
        println!("Enter Command > ");

        let command = parse_command();

        match command {
            Command::Add(item) => todo_list.push(item),
            Command::Remove(index) => {
                todo_list.remove(index);
            },
            Command::Show => {
                println!("Todo List:");
                for (index, item) in todo_list.iter().enumerate() {
                    println!("{}> {}", index, item);
                }
            }
            Command::Help => {
                println!("{}", HELP_TEXT);
            }
            Command::None => println!("Not a valid command!"),
        }
    }
}
