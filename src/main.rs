use std::io;

#[derive(Debug)]
enum Command {
    Add(String),
    Remove(usize),
    Show,
    Help,
    Quit,
    None,
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
    quit
        (exits the program)
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
        "quit" => return Command::Quit,
        "add" => return Command::Add(args),
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
    let mut todo_list: Vec<String> = Vec::new();

    println!("Welcome to the Todo List! Type \"help\" to get started...");
    
    loop {
        println!("Enter Command > ");

        let command = parse_command();
        match command {
            Command::Add(item) => todo_list.push(item),
            Command::Remove(index) => {
                if index >= todo_list.len() {
                    println!("Removal index out of bounds!");
                    continue;
                }
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
            Command::Quit => return,
        }
    }
}
