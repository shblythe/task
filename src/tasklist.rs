use std::{fs::{read_to_string, rename, File}, io::{Write}};

use crate::Task;

const PATH : &str = "tasks.json";
const BACKUP_PATH : &str = "tasks_backup.json";

#[derive(Default)]
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    /// Attempts to load the ``TaskList`` object from storage, and will
    /// return it if found, and if valid.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the file doesn't exist, or doesn't contain
    /// valid data.
    pub fn load() -> std::io::Result<Self> {
        let serialized = read_to_string(PATH)?;
        let tasks = serde_json::from_str(&serialized)?;
        Ok(TaskList {
            tasks
        })
    }

    /// Attempts to add a task to the list, and write to storage.
    /// 
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails
    pub fn add(&mut self, task: Task) -> std::io::Result<()> {
        self.tasks.push(task);
        self.save()
    }

    /// Returns a slice containing all the tasks
    #[must_use]
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Returns a individual task, by `index`
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    /// Attempts to replace a task in the list, and write to storage.
    /// 
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails
    pub fn replace(&mut self, index: usize, task: Task) -> std::io::Result<()> {
        self.tasks.remove(index);
        self.tasks.insert(index, task);
        self.save()
    }

    fn save(&self) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.tasks)?;
        let _ = rename(PATH, BACKUP_PATH);
        let mut file = File::create(PATH)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

