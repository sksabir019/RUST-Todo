use std::io;

#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}

impl Task {
    fn new(description: String, priority: u8) -> Self {
        Self {
            description,
            priority,
            completed: false,
        }
    }
}

fn main() {
    let mut task_list: Vec<Task> = Vec::new();
    loop {
        println!("\nPlease enter your choice");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View all tasks");
        println!("4. View Completed tasks");
        println!("5. View Pending tasks");
        println!("6. Toggle Complete/Incomplete");
        println!("7. Change Priority");
        println!("8. Sort Tasks");
        println!("9. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };
        match choice {
            1 => add_task(&mut task_list),
            2 => remove_task(&mut task_list),
            3 => view_task(&task_list),
            4 => view_completed_task(&task_list),
            5 => view_pending_task(&task_list),
            6 => toggle_complete(&mut task_list),
            7 => change_priority(&mut task_list),
            8 => sort_tasks(&mut task_list),
            9 => {
                println!("Exiting....");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}

fn is_empty(task_list: &Vec<Task>) -> bool {
    if task_list.is_empty() {
        println!("No tasks are found");
        true
    } else {
        false
    }
}

fn add_task(task_list: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Please enter a description for the task:");
    io::stdin()
        .read_line(&mut description)
        .expect("Invalid input");
    let description = description.trim();
    if description.is_empty() {
        println!("Description cannot be empty");
        return;
    }

    let mut priority = String::new();
    println!("Please enter a priority for the task (1-5):");
    io::stdin().read_line(&mut priority).expect("Invalid input");
    let priority: u8 = match priority.trim().parse() {
        Ok(p) if (1..=5).contains(&p) => p,
        _ => {
            println!("Priority must be between 1 and 5.");
            return;
        }
    };

    // Prevent duplicate
    if task_list
        .iter()
        .any(|t| t.description == description && t.priority == priority)
    {
        println!("Task with same description and priority already exists.");
        return;
    }

    task_list.push(Task::new(description.to_string(), priority));
    println!("Task added.");
}

fn remove_task(task_list: &mut Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Please enter the task no. which you want to remove:");
    view_task(task_list);
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");
    match task_number.trim().parse::<usize>() {
        Ok(task_number) if task_number > 0 && task_number <= task_list.len() => {
            println!("Are you sure you want to delete this task? (y/n):");
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).expect("Invalid input");
            if confirm.trim().eq_ignore_ascii_case("y") {
                task_list.remove(task_number - 1);
                println!("Task is removed");
            } else {
                println!("Task not removed");
            }
        }
        _ => println!("Wrong task number"),
    }
}

fn view_task(task_list: &Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Task list:");
    for (i, task) in task_list.iter().enumerate() {
        let status = if task.completed {
            "Completed"
        } else {
            "Pending"
        };
        println!(
            "{}. {} (Priority: {}, Status: {})",
            i + 1,
            task.description,
            task.priority,
            status
        );
    }
}

fn view_completed_task(task_list: &Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Completed tasks:");
    let mut found = false;
    for (i, task) in task_list.iter().enumerate() {
        if task.completed {
            println!(
                "{}. {} (Priority: {})",
                i + 1,
                task.description,
                task.priority
            );
            found = true;
        }
    }
    if !found {
        println!("No completed tasks found.");
    }
}

fn view_pending_task(task_list: &Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Pending tasks:");
    let mut found = false;
    for (i, task) in task_list.iter().enumerate() {
        if !task.completed {
            println!(
                "{}. {} (Priority: {})",
                i + 1,
                task.description,
                task.priority
            );
            found = true;
        }
    }
    if !found {
        println!("No pending tasks found.");
    }
}

fn toggle_complete(task_list: &mut Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Please enter the task no. to toggle complete/incomplete:");
    view_task(task_list);
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");
    match task_number.trim().parse::<usize>() {
        Ok(task_number) if task_number > 0 && task_number <= task_list.len() => {
            let task = &mut task_list[task_number - 1];
            task.completed = !task.completed;
            let status = if task.completed {
                "completed"
            } else {
                "pending"
            };
            println!("Task is now marked as {}.", status);
        }
        _ => println!("Wrong task number"),
    }
}

fn change_priority(task_list: &mut Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Please enter the task no. for which you want to change priority:");
    view_task(task_list);
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");
    let task_number: usize = match task_number.trim().parse() {
        Ok(num) if num > 0 && num <= task_list.len() => num,
        _ => {
            println!("Wrong task number");
            return;
        }
    };
    let mut new_priority = String::new();
    println!("Enter new priority (1-5):");
    io::stdin()
        .read_line(&mut new_priority)
        .expect("Invalid input");
    let new_priority: u8 = match new_priority.trim().parse() {
        Ok(p) if (1..=5).contains(&p) => p,
        _ => {
            println!("Priority must be between 1 and 5.");
            return;
        }
    };
    task_list[task_number - 1].priority = new_priority;
    println!("Priority updated.");
}

fn sort_tasks(task_list: &mut Vec<Task>) {
    if is_empty(task_list) {
        return;
    }
    println!("Sort by: 1. Priority  2. Status");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    match choice.trim() {
        "1" => {
            task_list.sort_by_key(|t| t.priority);
            println!("Tasks sorted by priority.");
        }
        "2" => {
            task_list.sort_by_key(|t| t.completed);
            println!("Tasks sorted by status.");
        }
        _ => println!("Invalid choice."),
    }
}
