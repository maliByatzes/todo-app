use std::{env, process};

#[allow(dead_code)]
struct Todo {
    title: String,
    done: bool,
}

#[allow(dead_code)]
impl Todo {
    fn new(title: String) -> Self {
        Self {
            title,
            done: false,
        }
    }
}

fn print_help() {
    println!("Usage: todo-app [ACTION] [OPTION] [MESSAGE]");
    println!("");
    println!("ACTION:");
    println!("    add [OPTION] [MESSAGE]        Add a new todo");
    println!("    remove [OPTION] [MESSAGE]     Remove a todo");
    println!("    edit [OPTION] [MESSAGE]       Edit a todo");
    println!("    list                          List all todos");
    println!("");
    println!("OPTION:");
    println!("    -t, --title [TITLE]           Provide todo's title");
    println!("    -h, --help                    Print this usage information");
    println!("");
    println!("Examples:");
    println!("    todo-app add -t \"Add a feature\"");
    process::exit(1);
}

/* Modurality to improve */
fn parse_args(args: &Vec<String>) {
    if args.len() < 2 { panic!("Invalid number of arguments!") }

    match &args[1][..] {
        "add" => {
            if args.len() > 4 { 
                eprintln!("Invalid number of args for adding a new todo!");
                print_help();
            }
            println!("Adding a new todo!");
            match &args[2][..] {
                "-t" | "--title" => { 
                    println!("The title is provided, proceed!");
                    match &args[3][..] {
                        title => println!("The title of the todo is: {}", title),
                    }
                },
                _ => {
                    eprintln!("Invalid option provided!");
                    print_help();
                },
            }
        },
        "remove" => println!("Removing a todo!"),
        "edit" => println!("Editing a todo!"),
        "list" => println!("Listing all todos!"),
        "-h" | "--help" => print_help(),
        _ => {
            eprintln!("Invalid argument!");
            print_help();
        },
    }   
}

fn main() {
    // Get the arguments from the environment
    let args: Vec<String> = env::args().collect();
    
    parse_args(&args);
    
    /*
    let mut todos = HashMap::new();

    todos.insert(String::from("Hello"), Todo::new(String::from("Hello")));

    // let todos: Vec<Todo> = vec![Todo::new(String::from("Hello World!"))];

    for todo in todos.values() {
        println!("title: {}", todo.title);
    }
    */


    // Add todo prototype cmd => ./todo-app add -t "Fix a bug"
    // Remove todo prototype => ./todo-app remove -t "Fix a bug"
    // Edit todo prototype => ./todo-app edit -t "Fix a bug"
    // List todos prototype => ./todo-app list
}
