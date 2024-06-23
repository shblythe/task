use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    description: String,
    dot: bool,
    complete: bool,
    uuid: Uuid
}

impl Task {
    #[must_use]
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            dot: false,
            complete: false,
            uuid: Uuid::new_v4()
        }
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn dot(&self) -> bool {
        self.dot
    }

    #[must_use]
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn update_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn toggle_dot(&mut self) {
        self.dot = !self.dot;
    }

    pub fn complete(&mut self) {
        self.dot = false;
        self.complete = true;
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.complete
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_str(
            if self.complete { "D " }
            else if self.dot { "- " }
            else { "  " }
        )?;
        f.write_str(&self.description)
    }
}

