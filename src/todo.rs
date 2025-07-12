// To Do list - add, remove, edit, view, exit

use std::io::{self, Write};

fn main() {
    let mut task_list: Vec<String> = Vec::new();

    loop {
        println!("\n--- To Do List ---");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View tasks");
        println!("4. Edit task");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid input");
        let choice = choice.trim();

        match choice {
            "1" => add_task(&mut task_list),
            "2" => remove_task(&mut task_list),
            "3" => view_tasks(&task_list),
            "4" => edit_task(&mut task_list),
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_task(task_list: &mut Vec<String>) {
    print!("Enter a description for the new task: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Invalid input");
    let description = description.trim();
    if !description.is_empty() {
        task_list.push(description.to_string());
        println!("Task added.");
    } else {
        println!("Description cannot be empty.");
    }
}

fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks found.");
        return;
    }
    view_tasks(task_list);
    print!("Enter the task number to remove: ");
    io::stdout().flush().unwrap();
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).expect("Invalid input");
    match task_number.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= task_list.len() => {
            task_list.remove(num - 1);
            println!("Task removed.");
        }
        _ => println!("Invalid task number."),
    }
}

fn edit_task(task_list: &mut Vec<String>) {
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
                task_list[num - 1] = new_task.to_string();
                println!("Task updated.");
            } else {
                println!("Description cannot be empty.");
            }
        }
        _ => println!("Invalid task number."),
    }
}

fn view_tasks(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks found.");
    } else {
        println!("\n--- Task List ---");
        for (i, task) in task_list.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }