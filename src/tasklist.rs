use std::{fs::{create_dir_all, read_to_string, rename, File}, io::Write, path::{Path, PathBuf}};

use dirs::config_local_dir;
use uuid::Uuid;

use crate::Task;

const PATH : &str = "tasks.json";
const BACKUP_PATH : &str = "tasks_backup.json";
const CONFIG_DIR : &str = "task";

pub struct TaskList {
    tasks: Vec<Task>,
    show_completed: bool,
    future_filter: bool,
    show_dotted_only: bool
}

impl Default for TaskList {
    fn default() -> Self {
        Self {
            tasks: Vec::default(),
            show_completed: Default::default(),
            future_filter: true,
            show_dotted_only: Default::default()
        }
    }
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
    /// valid data, or if we can't write back after post-load alterations.
    pub fn load() -> std::io::Result<Self> {
        let serialized = read_to_string(Self::save_path())?;
        let tasks = serde_json::from_str(&serialized)?;
        let mut task_list = TaskList {
            tasks,
            ..Default::default()
        };
        task_list.reset_recurring_and_snoozed()?;
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
            return Box::new(self.tasks.iter().filter(move |t|
                    !t.is_complete()
                    && (!self.future_filter || !t.not_current())
                    && (!self.show_dotted_only || t.dot())
                    ));
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

    /// Performs pre-render processing.
    /// MUST be called before each time the list is going to be rendered
    /// (i.e. ``filtered_tasks`` is going to be called to obtain task list)
    ///
    /// # Errors
    /// Returns an error if it failed to write data to disk
    pub fn pre_render(&mut self) -> std::io::Result<()> {
        self.reset_snoozed()
    }

    /// Move all snoozed tasks to bottom, remove dots if present
    /// We need to do this on every render cycle, for snoozed tasks ONLY
    /// to ensure that any that tasks that become unsnoozed are moved to the bottom in
    /// real-time and not just on next startup
    fn reset_snoozed(&mut self) -> std::io::Result<()> {
        self.reset_task_positions(true, false)
    }

    /// Move all recurring and unsnoozed tasks to bottom, remove dots if present
    fn reset_recurring_and_snoozed(&mut self) -> std::io::Result<()> {
        self.reset_task_positions(true, true)
    }

    /// Move all recurring tasks to bottom, remove dots if present
    ///
    /// NOTE!
    /// The logic below assumes that tasks cannot be both snoozed and recurring, if this becomes
    /// possible, then we'd need to fix.
    /// Basically, if a task is both recurring and snoozed, it will currently be unsnoozed by this
    /// method, which isn't what we'd want.
    fn reset_task_positions(&mut self, snoozed: bool, recurring: bool) -> std::io::Result<()> {
        let mut reset_uuids : Vec<Uuid> = vec![];
        for task in &self.tasks {
            if snoozed && task.snooze_expiring() || recurring && task.is_recurring() {
                reset_uuids.push(task.uuid());
            }
        }
        if reset_uuids.is_empty() {
            Ok(())
        } else {
            for uuid in reset_uuids {
                let mut task = self.get(uuid
                    ).expect("Should be able to find a task we know exists!").clone();
                task.remove_dot();
                task.unsnooze();
                self.replace_at_bottom_nosave(uuid, task);
            }
            self.save()
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

    pub fn toggle_future_filter(&mut self) {
        self.future_filter = !self.future_filter;
    }

    pub fn toggle_dotted_only(&mut self) {
        self.show_dotted_only = !self.show_dotted_only
    }

}

