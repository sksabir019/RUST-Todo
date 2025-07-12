use chrono::{Local, NaiveDate};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Clone, Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
    priority: Option<u8>,
    due_date: Option<NaiveDate>,
}

impl Task {
    fn display(&self, idx: usize) {
        let status = if self.completed {
            "[x]".green()
        } else {
            "[ ]".yellow()
        };
        let priority = self
            .priority
            .map(|p| {
                if p == 1 {
                    format!("(Priority: {})", p).red().to_string()
                } else if p <= 3 {
                    format!("(Priority: {})", p).yellow().to_string()
                } else {
                    format!("(Priority: {})", p).normal().to_string()
                }
            })
            .unwrap_or_default();

        let due = if let Some(date) = self.due_date {
            let today = Local::now().naive_local().date();
            if date < today && !self.completed {
                format!("(Due: {})", date).red().to_string()
            } else {
                format!("(Due: {})", date).cyan().to_string()
            }
        } else {
            "".to_string()
        };

        println!(
            "{} {}. {} {} {}",
            status,
            idx + 1,
            self.description,
            priority,
            due
        );
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
        println!("7. Save tasks (text)");
        println!("8. Load tasks (text)");
        println!("9. Export tasks (JSON)");
        println!("10. Import tasks (JSON)");
        println!("11. Undo last action");
        println!("12. Sort tasks");
        println!("13. Exit");
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
            "9" => export_json("tasks.json", &task_list),
            "10" => {
                task_list = import_json("tasks.json");
                println!("Tasks imported from JSON.");
            }
            "11" => {
                print!("Are you sure you want to undo the last action? (y/n): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim().eq_ignore_ascii_case("y") {
                    if let Some(prev) = undo_stack.pop_back() {
                        task_list = prev;
                        println!("Undo successful.");
                    } else {
                        println!("Nothing to undo.");
                    }
                } else {
                    println!("Undo cancelled.");
                }
            }
            "12" => sort_tasks(&mut task_list),
            "13" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_task(task_list: &mut Vec<Task>) {
    println!("Enter a description for the new task (end with a single '.' on a new line):");
    let mut description = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Invalid input");
        let line = line.trim_end();
        if line == "." {
            break;
        }
        if !description.is_empty() {
            description.push('\n');
        }
        description.push_str(line);
    }
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

    print!("Enter due date (YYYY-MM-DD, optional): ");
    io::stdout().flush().unwrap();
    let mut due = String::new();
    io::stdin().read_line(&mut due).expect("Invalid input");
    let due_date = match due.trim() {
        "" => None,
        s => NaiveDate::parse_from_str(s, "%Y-%m-%d").ok(),
    };

    task_list.push(Task {
        description: description.to_string(),
        completed: false,
        priority,
        due_date,
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
            println!("Enter the updated task description (end with a single '.' on a new line):");
            let mut new_task = String::new();
            loop {
                let mut line = String::new();
                io::stdin().read_line(&mut line).expect("Invalid input");
                let line = line.trim_end();
                if line == "." {
                    break;
                }
                if !new_task.is_empty() {
                    new_task.push('\n');
                }
                new_task.push_str(line);
            }
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
        let due = task.due_date.map(|d| d.to_string()).unwrap_or_default();
        let line = format!(
            "{}|{}|{}|{}\n",
            task.description.replace('\n', "\\n"),
            completed,
            priority,
            due
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
                let description = parts[0].replace("\\n", "\n");
                let completed = parts[1] == "1";
                let priority = if parts.len() > 2 && !parts[2].is_empty() {
                    parts[2].parse::<u8>().ok()
                } else {
                    None
                };
                let due_date = if parts.len() > 3 && !parts[3].is_empty() {
                    NaiveDate::parse_from_str(parts[3], "%Y-%m-%d").ok()
                } else {
                    None
                };
                tasks.push(Task {
                    description,
                    completed,
                    priority,
                    due_date,
                });
            }
        }
    }
    tasks
}

fn export_json(filename: &str, task_list: &Vec<Task>) {
    match serde_json::to_string_pretty(task_list) {
        Ok(json) => {
            if std::fs::write(filename, json).is_ok() {
                println!("Tasks exported to JSON.");
            } else {
                println!("Failed to write JSON file.");
            }
        }
        Err(_) => println!("Failed to serialize tasks."),
    }
}

fn import_json(filename: &str) -> Vec<Task> {
    match std::fs::read_to_string(filename) {
        Ok(data) => match serde_json::from_str(&data) {
            Ok(tasks) => tasks,
            Err(_) => {
                println!("Failed to parse JSON.");
                Vec::new()
            }
        },
        Err(_) => {
            println!("Failed to read JSON file.");
            Vec::new()
        }
    }
}

fn sort_tasks(task_list: &mut Vec<Task>) {
    println!("Sort by: 1. Priority  2. Status  3. Due Date");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "1" => {
            task_list.sort_by_key(|t| t.priority.unwrap_or(99));
            println!("Tasks sorted by priority.");
        }
        "2" => {
            task_list.sort_by_key(|t| t.completed);
            println!("Tasks sorted by status.");
        }
        "3" => {
            task_list.sort_by_key(|t| {
                t.due_date
                    .unwrap_or(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap())
            });
            println!("Tasks sorted by due date.");
        }
        _ => println!("Invalid choice."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_mark_complete() {
        let mut tasks = Vec::new();
        tasks.push(Task {
            description: "Test".to_string(),
            completed: false,
            priority: Some(2),
            due_date: None,
        });
        assert_eq!(tasks.len(), 1);
        tasks[0].completed = true;
        assert!(tasks[0].completed);
    }

    #[test]
    fn test_save_and_load() {
        let filename = "test_tasks.txt";
        let mut tasks = Vec::new();
        tasks.push(Task {
            description: "Test Save".to_string(),
            completed: false,
            priority: Some(1),
            due_date: None,
        });
        save_tasks(filename, &tasks);
        let loaded = load_tasks(filename);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].description, "Test Save");
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_json_export_import() {
        let filename = "test_tasks.json";
        let mut tasks = Vec::new();
        tasks.push(Task {
            description: "Test JSON".to_string(),
            completed: false,
            priority: Some(3),
            due_date: None,
        });
        export_json(filename, &tasks);
        let loaded = import_json(filename);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].description, "Test JSON");
        std::fs::remove_file(filename).unwrap();
    }
}
