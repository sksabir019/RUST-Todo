// To Do list - add task, remove task, view task, exit

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
        println!("6. Mark Complete");
        println!("7. Change Priority");
        println!("8. Exit");
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
            6 => mark_complete(&mut task_list),
            7 => change_priority(&mut task_list),
            8 => {
                println!("Exiting....");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}

fn add_task(task_list: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Please enter a description for the task:");
    io::stdin()
        .read_line(&mut description)
        .expect("Invalid input");
    let description = description.trim().to_string();

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

    if !description.is_empty() {
        task_list.push(Task::new(description, priority));
        println!("Task added.");
    } else {
        println!("Description cannot be empty");
    }
}

fn remove_task(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
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
            task_list.remove(task_number - 1);
            println!("Task is removed");
        }
        _ => println!("Wrong task number"),
    }
}

fn view_task(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Task list:");
    for (i, task) in task_list.iter().enumerate() {
        println!(
            "{}. {:?} (Priority: {}, Completed: {})",
            i + 1,
            task.description,
            task.priority,
            task.completed
        );
    }
}

fn view_completed_task(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Completed tasks:");
    for (i, task) in task_list.iter().enumerate() {
        if task.completed {
            println!(
                "{}. {:?} (Priority: {})",
                i + 1,
                task.description,
                task.priority
            );
        }
    }
}

fn view_pending_task(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Pending tasks:");
    for (i, task) in task_list.iter().enumerate() {
        if !task.completed {
            println!(
                "{}. {:?} (Priority: {})",
                i + 1,
                task.description,
                task.priority
            );
        }
    }
}

fn mark_complete(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Please enter the task no. which you want to mark complete:");
    view_task(task_list);
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");
    match task_number.trim().parse::<usize>() {
        Ok(task_number) if task_number > 0 && task_number <= task_list.len() => {
            task_list[task_number - 1].completed = true;
            println!("Task is marked as completed");
        }
        _ => println!("Wrong task number"),
    }
}

fn change_priority(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks are found");
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
