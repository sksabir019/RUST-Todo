use std::io;
#[derive(Debug, Clone)]
struct Task {
    t_id: u32,
    t_title: String,
    t_description: String,
    t_priority: u32,
    t_completed: bool,
}

impl Task {
    fn new(id: u32, description: &String, title: &String, priority: u32) -> Self {
        Task {
            t_id: id,
            t_title: title.to_string(),
            t_description: description.to_string(),
            t_priority: priority,
            t_completed: false,
        }
    }
}

trait TaskMethods {
    fn add_task(&mut self, task: &Task) -> String;
    fn view_all_tasks(&self);
    fn remove_task(&mut self, task_id: u32) -> Vec<&Task>;
    fn mark_complete(&mut self, task_id: u32) -> Result<(), String>;
    fn view_completed_tasks(&self);
    fn view_pending_tasks(&self);
    // sort the todos according to their todo priority
    fn sort_tasks(&mut self);
}

impl TaskMethods for Vec<Task> {
    fn add_task(&mut self, task: &Task) -> String {
        let mut add_to_list = |task: &Task| {
            println!("{:#?}", task);
            self.push(task.clone());
        };
        add_to_list(&task);
        "Task added Successfully".to_string()
    }

    fn view_all_tasks(&self) {
        println!("Tasks:");
        for (i, task) in self.iter().enumerate() {
            println!("{}: {:#?}", i + 1, task);
        }
        let allTasks: Vec<_> = self
            .iter()
            .map(|task| {
                println!("{:#?}", task);
                task
            })
            .collect::<Vec<_>>();

        // // allTasks;
        println!("All Tasks:");
        println!("{:#?}", allTasks);
    }

    fn remove_task(&mut self, task_id: u32) -> Vec<&Task> {
        // firs finding the task
        match self.iter().find(|task| task.t_id == task_id) {
            Some(task) => {
                // print the taskk
                println!("Task to be removed: {:#?}", task);
                self.iter()
                    .filter(|task| task.t_id == task_id)
                    .collect::<Vec<_>>()
            }
            None => {
                println!("Task not found.");
                self.iter().map(|task| task).collect::<Vec<_>>()
            }
        }
    }

    fn mark_complete(&mut self, task_id: u32) -> Result<(), String> {
        let mut tasks_to_complete = self
            .iter_mut()
            .filter(|task| task.t_id == task_id)
            .collect::<Vec<_>>();
        println!("Tasks to be marked as complete: {:#?}", tasks_to_complete);
        if tasks_to_complete.is_empty() {
            Err("Task not found.".to_string())
        } else {
            tasks_to_complete[0].t_completed = true;
            Ok(())
        }
    }
    // view completed tasks
    fn view_completed_tasks(&self) {
        // using iterators
        let completed_tasks: Vec<&Task> = self
            .iter()
            .filter(|task| task.t_completed)
            .collect::<Vec<_>>();
        println!("Completed Tasks:");
        println!("{:#?}", completed_tasks);
        for (i, task) in completed_tasks.iter().enumerate() {
            println!("{}: {:#?}", i + 1, task);
        }
    }

    fn view_pending_tasks(&self) {
        // using iterators
        let pending_tasks: Vec<&Task> = self
            .iter()
            .filter(|task| !task.t_completed)
            .collect::<Vec<_>>();
        println!("Pending Tasks:");
        println!("{:#?}", pending_tasks);
        for (i, task) in pending_tasks.iter().enumerate() {
            println!("{}: {:#?}", i + 1, task);
        }
    }

    fn sort_tasks(&mut self) {
        self.sort_by(|a, b| b.t_priority.cmp(&a.t_priority));
        println!("Tasks sorted by priority:");
        for (i, task) in self.iter().enumerate() {
            println!("{}: {:#?}", i + 1, task);
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        let mut choice: String = String::new();
        println!("Welcome to the Todo list");
        println!("1. Add task: ");
        println!("2. Remove Task: ");
        println!("3. View Task: ");
        println!("4. View All Tasks: ");
        println!("5. View Pending Task: ");
        println!("6. Mark Complete: ");
        println!("7. Viewed All Completed Tasks: ");
        println!("8. View Pending Tasks: ");
        println!("9. Exit: ");

        io::stdin()
            .read_line(&mut choice)
            .expect("Error in Reading the Buffer:");

        let choice: i32 = match choice.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input! Please enter a number.");
                continue;
            }
        };
        println!("Your choice is {:?}", choice);

        match choice {
            1 => {
                println!("Task Added");

                println!("Enter the task Id:");
                let mut task_id = String::new();
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Error in reading the Task ID");

                println!("Enter the task title:");
                let mut task_title = String::new();
                io::stdin()
                    .read_line(&mut task_title)
                    .expect("Error in Reading the Task Title:");

                println!("Enter the task description:");
                let mut task_description = String::new();
                io::stdin()
                    .read_line(&mut task_description)
                    .expect("Error in Reading the Task description:");

                println!("Enter the task priority (0-10):");
                let mut task_priority = String::new();
                io::stdin()
                    .read_line(&mut task_priority)
                    .expect("Error in Reading the Task priority:");

                let title: String = match task_title.trim().parse::<String>() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid task Title!");
                        continue;
                    }
                };

                let id = match task_id.trim().parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid Task ID!");
                        continue;
                    }
                };

                let priority: u32 = match task_priority.trim().parse::<u32>() {
                    Ok(value) => match value {
                        0..=10 => value,
                        _ => {
                            println!("Priority should be between 0 and 10!");
                            continue;
                        }
                    },
                    Err(_) => {
                        println!("Invalid Priority description!");
                        continue;
                    }
                };

                let description: String = match task_description.trim().parse::<String>() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid task description!");
                        continue;
                    }
                };

                let newTask: Task = Task::new(id, &description, &title, priority);
                tasks.add_task(&newTask);
                println!("{:#?}", tasks);
            }
            2 => {
                // println!("Task removed");
                let mut task_id = String::new();
                println!("Enter the TaskId");
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Error reading task id");
                let c_t_id: u32 = match task_id.trim().parse::<u32>() {
                    // c_t_id is converted task ID
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid task id!");
                        continue;
                    }
                };
                let removed_ts = tasks.remove_task(c_t_id);
                println!("After removing the tasks....");
                println!("{:#?}", removed_ts);
            }
            3 => println!("Task Viewed"),
            4 => {
                println!("Viewed All Completed Task");
                let taks = tasks.view_all_tasks();
                println!("{:#?}", tasks);
            }
            5 => println!("Viewed All Pending Task"),
            6 => {
                let mut task_id = String::new();
                println!("Enter the TaskId");
                io::stdin()
                    .read_line(&mut task_id)
                    .expect("Error reading task id");
                let c_t_id: u32 = match task_id.trim().parse::<u32>() {
                    // c_t_id is converted task ID
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid task id!");
                        continue;
                    }
                };

                tasks.mark_complete(c_t_id);
                println!("After marking as complete....");
                println!("{:#?}", tasks);
                println!("Marked Completed");
            }
            7 => {
                println!("Viewed All Completed Tasks");
                tasks.view_completed_tasks();
            }
            8 => {
                println!("Viewed All Pending Task");
                tasks.view_pending_tasks();
            }
            10 => {
                println!("Get Sorted Tasks by Priority");
                tasks.sort_tasks();
            }
            9 => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Invalid choice! Please try again."),
        }
    }
}
