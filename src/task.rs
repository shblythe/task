use std::{fmt::Display};

use chrono::{Days, Local, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    description: String,
    dot: bool,
    uuid: Uuid,
    // Contains an Instant if the task is complete
    completed: Option<NaiveDateTime>,
    // Contains an Instant which is the next recurrence, if it's a
    // recurring task.
    recur_next: Option<NaiveDateTime>,
    // Contains a recur interval in days, if it's a recurring task
    recur_interval_days: Option<u64>
}

impl Task {
    #[must_use]
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_string(),
            dot: false,
            uuid: Uuid::new_v4(),
            completed: None,
            recur_next: None,
            recur_interval_days: None
        }
    }

    /// Sets the task to recur daily
    ///
    /// # Panics
    ///
    /// Will panic if some apparently impossible things happen!
    pub fn set_recur_daily(&mut self) {
        self.recur_interval_days = Some(1);
    }

    /// Update ``recur_next`` field for recurring tasks
    fn recur_next(&mut self) {
        if let Some(interval) = self.recur_interval_days {
            let mut next_5am = NaiveDateTime::new(Local::now().date_naive(), NaiveTime::from_hms_opt(5,0,0).expect("5am not a valid time!"));
            if next_5am < Local::now().naive_local() {
                next_5am = next_5am.checked_add_days(Days::new(1)).expect("Couldn't add 1 day in set_recur_daily");
            }
            next_5am = next_5am.checked_add_days(Days::new(interval - 1)).expect("Couldn't add days in set_recur_daily");
            self.recur_next = Some(next_5am);
        }
    }

    pub fn clear_recur(&mut self) {
        self.recur_next = None;
        self.recur_interval_days = None;
    }

    /// Returns true if the task is not current - i.e. is not complete
    /// but is not currently eligible to be displayed - because it has
    /// snoozed, or it is a recurring task that we've completed and it
    /// isn't yet time for it to occur again.
    #[must_use]
    pub fn not_current(&self) -> bool {
        if let Some(next) = self.recur_next {
            Local::now().naive_local() < next
        } else {
            false
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
        if self.recur_interval_days.is_some() {
            self.recur_next();
        } else {
            self.completed = Some(Local::now().naive_local());
        }
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completed.is_some()
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.write_str(
            if self.is_complete() { "D " }
            else if self.dot { "- " }
            else { "  " }
        )?;
        f.write_str(
            if self.recur_interval_days.is_some() { "R " }
            else { "  " }
        )?;
        f.write_str(&self.description)
    }
}

