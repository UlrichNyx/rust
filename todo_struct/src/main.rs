#[derive(Clone)]
struct Task {
    description: String,
    status: bool,
    due_date: String
}
#[derive(Clone)]
struct Todos {
    list: Vec<Task>
}


fn main() {
    let task1 = Task{description:String::from("Task1"), status: false, due_date: String::from("tomorrow")};
    let task2 = Task{description:String::from("Task2"), status: false, due_date: String::from("yesterday")};
    let task3 = Task{description:String::from("Task3"), status: true, due_date: String::from("today")};


    let todos = Todos {list : [task1, task2, task3].to_vec()};

    print_todos(todos);
}

fn print_todos(tasks : Todos) {
    println!("====TODOS====");
    for t in tasks.list {
        println!("{} - {} - {}", t.description, t.status, t.due_date);
    }
    println!("=============");
}
