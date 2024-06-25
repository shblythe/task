use std::{fs::{read_to_string, rename, File}, io::Write};

use uuid::Uuid;

use crate::Task;

const PATH : &str = "tasks.json";
const BACKUP_PATH : &str = "tasks_backup.json";

#[derive(Default)]
pub struct TaskList {
    tasks: Vec<Task>,
    show_completed: bool,
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
            tasks,
            show_completed: false,
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

    #[must_use]
    pub fn filtered_tasks(&self) -> Box<dyn DoubleEndedIterator<Item = &Task> + '_> {
        if !self.show_completed {
            return Box::new(self.tasks.iter().filter(|t| !t.is_complete() && !t.not_current()));
        }
        Box::new(self.tasks.iter())
    }

    /// Returns a slice containing all the tasks
    #[must_use]
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Returns a individual task, by `uuid`
    #[must_use]
    pub fn get(&self, uuid: Uuid) -> Option<&Task> {
        self.tasks.iter().find(|t| t.uuid() == uuid)
    }

    /// Attempts to replace a task in the list, and write to storage.
    /// Fails silently if the task to replace isn't found!
    /// 
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails
    pub fn replace(&mut self, uuid: Uuid, task: Task) -> std::io::Result<()> {
        if let Some(index) = self.tasks.iter().position(|t| t.uuid() == uuid) {
            self.tasks.remove(index);
            self.tasks.insert(index, task);
            self.save()
        } else {
            Ok(())
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.tasks)?;
        let _ = rename(PATH, BACKUP_PATH);
        let mut file = File::create(PATH)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

