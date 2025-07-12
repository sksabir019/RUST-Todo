// To Do list - add task, remove task, view task, change priority, exit

use std::io;

#[derive(Debug)]
struct Task {
    description: String,
    priority: u8,
    completed: bool,
}

struct TaskList {
    task_list: Vec<Task>,
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

impl TaskList {
    fn new() -> Self {
        Self {
            task_list: Vec::new(),
        }
    }

    fn add_task(&mut self) {
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
            self.task_list.push(Task::new(description, priority));
            println!("Task added.");
        } else {
            println!("Description cannot be empty");
        }
    }

    fn remove_task(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return;
        }
        println!("Please enter the task no. which you want to remove:");
        self.view_task();
        let mut task_number = String::new();
        io::stdin()
            .read_line(&mut task_number)
            .expect("Invalid input");
        match task_number.trim().parse::<usize>() {
            Ok(task_number) if task_number > 0 && task_number <= self.task_list.len() => {
                self.task_list.remove(task_number - 1);
                println!("Task is removed");
            }
            _ => println!("Wrong task number"),
        }
    }

    fn view_task(&self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return;
        }
        println!("Task list:");
        for (i, task) in self.task_list.iter().enumerate() {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!(
                "{}. {} (Priority: {}, Status: {})",
                i + 1,
                task.description,
                task.priority,
                status
            );
        }
    }

    fn change_priority(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return;
        }
        println!("Please enter the task no. for which you want to change priority:");
        self.view_task();
        let mut task_number = String::new();
        io::stdin()
            .read_line(&mut task_number)
            .expect("Invalid input");
        let task_number: usize = match task_number.trim().parse() {
            Ok(num) if num > 0 && num <= self.task_list.len() => num,
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
        self.task_list[task_number - 1].priority = new_priority;
        println!("Priority updated.");
    }

    fn toggle_complete(&mut self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return;
        }
        println!("Please enter the task no. to toggle complete/incomplete:");
        self.view_task();
        let mut task_number = String::new();
        io::stdin()
            .read_line(&mut task_number)
            .expect("Invalid input");
        match task_number.trim().parse::<usize>() {
            Ok(task_number) if task_number > 0 && task_number <= self.task_list.len() => {
                let task = &mut self.task_list[task_number - 1];
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
    fn is_empty(&self) -> bool {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            true
        } else {
            false
        }
    }
    fn view_pending_tasks(&self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return;
        }
        let mut found = false;
        println!("Pending tasks:");
        for (i, task) in self.task_list.iter().enumerate() {
            if !task.completed {
                found = true;
                println!(
                    "{}. {} (Priority: {})",
                    i + 1,
                    task.description,
                    task.priority
                );
            }
        }
        if !found {
            println!("No pending tasks found.");
        }
    }
    fn view_completed_tasks(&self) {
        if self.task_list.is_empty() {
            println!("No tasks are found");
            return; 
        }
    }
    
    let completed_tasks: Vec<&Task> = self
            .task_list
            .iter()
            .filter(|task| task.completed)
            .collect();
        if completed_tasks.is_empty() {
            println!("No completed tasks found.");
        } else {
            println!("Completed tasks:");
            for (i, task) in completed_tasks.iter().enumerate() {
                println!(
                    "{}. {} (Priority: {})",
                    i + 1,
                    task.description,
                    task.priority
                );
            }
        }
    
    
    
}

fn main() {
    let mut task_list = TaskList::new();
    loop {
        println!("\nPlease enter your choice");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. View task");
        println!("4. Change Priority");
        println!("5. Exit");
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
            1 => task_list.add_task(),
            2 => task_list.remove_task(),
            3 => task_list.view_task(),
            4 => task_list.change_priority(),
            5 => {
                println!("Exiting....");
                break;
            }
            _ => println!("Wrong Input: Try Again"),
        }
    }
}