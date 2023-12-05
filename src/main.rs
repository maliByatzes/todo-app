use std::env;

struct Todo {
    title: String,
    done: bool,
}

impl Todo {
    fn new(title: String) -> Self {
        Self {
            title,
            done: false,
        }
    }
}

fn main() {
    // Get the arguments from the environment
    let _args: Vec<String> = env::args().collect();

    let todos: Vec<Todo> = vec![Todo::new(String::from("Hello World!"))];

    for todo in todos {
        println!("title: {}", todo.title);
    }


    // Add todo prototype cmd => ./todo-app add -t "Fix a bug"
    // Remove todo prototype => ./todo-app remove -t "Fix a bug"
    // Edit todo prototype => ./todo-app edit -t "Fix a bug"
    // List todos prototype => ./todo-app list
}
