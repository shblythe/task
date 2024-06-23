use crate::Task;

#[derive(Default)]
pub struct TaskList {
    pending_tasks: Vec<Task>,
}

impl TaskList {
    pub fn add(&mut self, task: Task) {
        self.pending_tasks.push(task);
    }

    #[must_use]
    pub fn tasks(&self) -> &[Task] {
        &self.pending_tasks
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Task> {
        self.pending_tasks.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.pending_tasks.get_mut(index)
    }
}

