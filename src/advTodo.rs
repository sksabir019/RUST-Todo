// To Do list - add, remove, edit, view, mark complete, search, save/load, undo, priorities, exit

use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Clone)]
struct Task {
    description: String,
    completed: bool,
    priority: Option<u8>,
}

impl Task {
    fn display(&self, idx: usize) {
        let status = if self.completed { "[x]" } else { "[ ]" };
        let priority = self
            .priority
            .map(|p| format!("(Priority: {})", p))
            .unwrap_or_default();
        println!("{} {}. {} {}", status, idx + 1, self.description, priority);
    }
}

fn main() {
    let mut task_list: Vec<Task> = load_tasks("tasks.txt");
    let mut undo_stack: VecDeque<Vec<Task>> = VecDeque::new();

    loop {
        println!("\n--- To Do List ---");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View tasks");
        println!("4. Edit task");
        println!("5. Mark task as complete/incomplete");
        println!("6. Search tasks");
        println!("7. Save tasks");
        println!("8. Load tasks");
        println!("9. Undo last action");
        println!("10. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input");
        let choice = choice.trim();

        match choice {
            "1" => {
                undo_stack.push_back(task_list.clone());
                add_task(&mut task_list);
            }
            "2" => {
                undo_stack.push_back(task_list.clone());
                remove_task(&mut task_list);
            }
            "3" => view_tasks(&task_list),
            "4" => {
                undo_stack.push_back(task_list.clone());
                edit_task(&mut task_list);
            }
            "5" => {
                undo_stack.push_back(task_list.clone());
                mark_task(&mut task_list);
            }
            "6" => search_tasks(&task_list),
            "7" => save_tasks("tasks.txt", &task_list),
            "8" => {
                task_list = load_tasks("tasks.txt");
                println!("Tasks loaded from file.");
            }
            "9" => {
                if let Some(prev) = undo_stack.pop_back() {
                    task_list = prev;
                    println!("Undo successful.");
                } else {
                    println!("Nothing to undo.");
                }
            }
            "10" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_task(task_list: &mut Vec<Task>) {
    print!("Enter a description for the new task: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Invalid input");
    let description = description.trim();
    if description.is_empty() {
        println!("Description cannot be empty.");
        return;
    }

    print!("Enter priority (1-5, optional): ");
    io::stdout().flush().unwrap();
    let mut priority = String::new();
    io::stdin().read_line(&mut priority).expect("Invalid input");
    let priority = priority
        .trim()
        .parse::<u8>()
        .ok()
        .filter(|&p| p >= 1 && p <= 5);

    task_list.push(Task {
        description: description.to_string(),
        completed: false,
        priority,
    });
    println!("Task added.");
}

fn remove_task(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks found.");
        return;
    }
    view_tasks(task_list);
    print!("Enter the task number to remove: ");
    io::stdout().flush().unwrap();
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");
    match task_number.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= task_list.len() => {
            print!("Are you sure you want to delete this task? (y/n): ");
            io::stdout().flush().unwrap();
            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm).unwrap();
            if confirm.trim().eq_ignore_ascii_case("y") {
                task_list.remove(num - 1);
                println!("Task removed.");
            } else {
                println!("Cancelled.");
            }
        }
        _ => println!("Invalid task number."),
    }
}

fn edit_task(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks found.");
        return;
    }
    view_tasks(task_list);
    print!("Enter the task number to edit: ");
    io::stdout().flush().unwrap();
    let mut task_num = String::new();
    io::stdin().read_line(&mut task_num).expect("Invalid input");
    match task_num.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= task_list.len() => {
            print!("Enter the updated task description: ");
            io::stdout().flush().unwrap();
            let mut new_task = String::new();
            io::stdin().read_line(&mut new_task).expect("Invalid input");
            let new_task = new_task.trim();
            if !new_task.is_empty() {
                task_list[num - 1].description = new_task.to_string();
                println!("Task updated.");
            } else {
                println!("Description cannot be empty.");
            }
        }
        _ => println!("Invalid task number."),
    }
}

fn mark_task(task_list: &mut Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks found.");
        return;
    }
    view_tasks(task_list);
    print!("Enter the task number to toggle complete/incomplete: ");
    io::stdout().flush().unwrap();
    let mut task_num = String::new();
    io::stdin().read_line(&mut task_num).expect("Invalid input");
    match task_num.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= task_list.len() => {
            task_list[num - 1].completed = !task_list[num - 1].completed;
            let status = if task_list[num - 1].completed {
                "completed"
            } else {
                "pending"
            };
            println!("Task marked as {}.", status);
        }
        _ => println!("Invalid task number."),
    }
}

fn search_tasks(task_list: &Vec<Task>) {
    print!("Enter keyword to search: ");
    io::stdout().flush().unwrap();
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).expect("Invalid input");
    let keyword = keyword.trim().to_lowercase();
    let mut found = false;
    for (i, task) in task_list.iter().enumerate() {
        if task.description.to_lowercase().contains(&keyword) {
            task.display(i);
            found = true;
        }
    }
    if !found {
        println!("No tasks found matching '{}'.", keyword);
    }
}

fn view_tasks(task_list: &Vec<Task>) {
    if task_list.is_empty() {
        println!("No tasks found.");
    } else {
        println!("\n--- Task List ---");
        for (i, task) in task_list.iter().enumerate() {
            task.display(i);
        }
    }
}

fn save_tasks(filename: &str, task_list: &Vec<Task>) {
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
    {
        Ok(f) => f,
        Err(_) => {
            println!("Failed to open file for saving.");
            return;
        }
    };
    for task in task_list {
        let completed = if task.completed { "1" } else { "0" };
        let priority = task.priority.map(|p| p.to_string()).unwrap_or_default();
        let line = format!(
            "{}|{}|{}\n",
            task.description.replace('\n', " "),
            completed,
            priority
        );
        if let Err(_) = file.write_all(line.as_bytes()) {
            println!("Failed to write to file.");
            return;
        }
    }
    println!("Tasks saved to file.");
}

fn load_tasks(filename: &str) -> Vec<Task> {
    let file = File::open(filename);
    let mut tasks = Vec::new();
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                let description = parts[0].to_string();
                let completed = parts[1] == "1";
                let priority = if parts.len() > 2 && !parts[2].is_empty() {
                    parts[2].parse::<u8>().ok()
                } else {
                    None
                };
                tasks.push(Task {
                    description,
                    completed,
                    priority,
                });
            }
        }
    }
    tasks
}
