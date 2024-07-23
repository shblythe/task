use std::{fs::{create_dir_all, read_to_string, rename, File}, io::Write, path::{Path, PathBuf}};

use dirs::config_local_dir;
use uuid::Uuid;

use crate::Task;

const PATH : &str = "tasks.json";
const BACKUP_PATH : &str = "tasks_backup.json";
const CONFIG_DIR : &str = "task";

#[derive(Default)]
pub struct TaskList {
    tasks: Vec<Task>,
    show_completed: bool,
}

impl TaskList {
    fn config_dir_pathbuf() -> PathBuf {
        let dir_path_buf = config_local_dir().unwrap_or_default();
        dir_path_buf.join(Path::new(CONFIG_DIR))
    }

    fn path_to_save_file(file: &str) -> String {
        Self::config_dir_pathbuf().join(Path::new(file)).to_string_lossy().to_string()
    }

    fn save_path() -> String {
        Self::path_to_save_file(PATH)
    }

    fn backup_path() -> String {
        Self::path_to_save_file(BACKUP_PATH)
    }

    /// Attempts to load the ``TaskList`` object from storage, and will
    /// return it if found, and if valid.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the file doesn't exist, or doesn't contain
    /// valid data.
    pub fn load() -> std::io::Result<Self> {
        let serialized = read_to_string(Self::save_path())?;
        let tasks = serde_json::from_str(&serialized)?;
        let mut task_list = TaskList {
            tasks,
            show_completed: false,
        };
        task_list.reset_recurring();
        Ok(task_list)
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

    /// Attempts to replace a task in the list, but repositioned to the bottom
    /// and write to storage.
    /// Fails silently if the task to replace isn't found!
    /// 
    /// # Errors
    ///
    /// Will return `Err` if the write to storage fails
    pub fn replace_at_bottom(&mut self, uuid: Uuid, task: Task) -> std::io::Result<()> {
        self.replace_at_bottom_saveopt(uuid, task, true)
    }

    fn replace_at_bottom_saveopt(&mut self, uuid: Uuid, task: Task, save: bool) -> std::io::Result<()> {
        if let Some(index) = self.tasks.iter().position(|t| t.uuid() == uuid) {
            self.tasks.remove(index);
            self.tasks.push(task);
            if save {
                self.save()
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn replace_at_bottom_nosave(&mut self, uuid: Uuid, task: Task) {
        let _ = self.replace_at_bottom_saveopt(uuid, task, false);
    }

    /// Move all recurring tasks to bottom, remove dots if present
    fn reset_recurring(&mut self) {
        let mut recurring_uuids : Vec<Uuid> = vec![];
        for task in &self.tasks {
            if task.is_recurring() {
                recurring_uuids.push(task.uuid());
            }
        }
        for uuid in recurring_uuids {
            let mut task = self.get(uuid
                ).expect("Should be able to find a task we know exists!").clone();
            task.remove_dot();
            self.replace_at_bottom_nosave(uuid, task);
        }
    }

    fn save(&self) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.tasks)?;
        let config_dir_path = &Self::config_dir_pathbuf();
        if !config_dir_path.exists() {
            create_dir_all(config_dir_path).expect("Couldn't create config dir");
        }
        assert!(config_dir_path.is_dir(), "Config dir path exists but is not a directory");
        let _ = rename(Self::save_path(), Self::backup_path());
        let mut file = File::create(Self::save_path())?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

