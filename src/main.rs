// To Do list - add task, remove task, view task, exit

use std::io;

fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        let mut choice = String::new();
        println!("Please enter your choice");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View task");
        println!("4. Exit");
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
            4 => {
                println!("Exiting....");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}

fn add_task(task_list: &mut Vec<String>) {
    let mut description = String::new();
    println!("Please enter a description for the task:");
    io::stdin()
        .read_line(&mut description)
        .expect("Invalid input");
    let description = description.trim().to_string();
    if !description.is_empty() {
        task_list.push(description);
    } else {
        println!("Description cannot be empty");
    }
}

fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Please enter the task no. which you want to remove");
    view_task(task_list);
    let mut task_number = String::new();
    io::stdin()
        .read_line(&mut task_number)
        .expect("Invalid input");
    match task_number.trim().parse::<usize>() {
        Ok(task_number) => {
            if task_number == 0 || task_number > task_list.len() {
                println!("Wrong task number");
                return;
            }
            task_list.remove(task_number - 1);
            println!("Task is removed");
        }
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
}

fn view_task(task_list: &Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Task list:");
    for (i, task) in task_list.iter().enumerate() {
        println!("{}. {}", i + 1, task);
    }
}

fn edit_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("No tasks are found");
        return;
    }
    println!("Please enter the task no. which you want to edit:");
    view_task(&task_list);
    let mut task_num = String::new();
    io::stdin().read_line(&mut task_num).expect("invalid input");
    let task_num: usize = match task_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid");
            return;
        }
    };
    if task_num == 0 || task_num > task_list.len() {
        println!("Wrong task number");
        return;
    }
    println!("Type updated task:");
    let mut new_task = String::new();
    io::stdin()
        .read_line(&mut new_task)
        .expect("something went wrong");
    task_list[task_num - 1]