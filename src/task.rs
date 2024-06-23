use std::fmt::Display;

pub struct Task {
    description: String,
    dot: bool,
    complete: bool
}

impl Task {
    #[must_use]
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            dot: false,
            complete: false
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

