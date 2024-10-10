// ### Author
// 
// * UlrichNyx
//
// ### Date
//
// * 2024-10-10
//
// ### Description
//
// A Rust program that defines a `Task` struct to represent individual tasks with a description, status, and due date.
// The `Todos` struct holds a list of tasks. The main function demonstrates creating a list of tasks and printing 
// them using the `print_todos` function.

#[derive(Clone)]
struct Task {
    description: String,
    status: bool,
    due_date: String,
}

#[derive(Clone)]
struct Todos {
    list: Vec<Task>,
}

/// ### Main function
///
/// Creates several `Task` instances, groups them into a `Todos` struct, and prints the list of tasks.
fn main() {
    let task1 = Task {
        description: String::from("Task1"),
        status: false,
        due_date: String::from("tomorrow"),
    };
    let task2 = Task {
        description: String::from("Task2"),
        status: false,
        due_date: String::from("yesterday"),
    };
    let task3 = Task {
        description: String::from("Task3"),
        status: true,
        due_date: String::from("today"),
    };

    let todos = Todos { list: vec![task1, task2, task3] };

    print_todos(todos);
}

/// Prints all tasks in the `Todos` list.
///
/// ### Arguments
///
/// * `tasks` - A `Todos` struct containing a list of tasks.
fn print_todos(tasks: Todos) {
    println!("====TODOS====");
    for t in tasks.list {
        println!("{} - {} - {}", t.description, t.status, t.due_date);
    }
    println!("=============");
}
