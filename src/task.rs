use std::fmt::Display;

use chrono::{Days, Local, NaiveDateTime, NaiveTime, TimeDelta};
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
    recur_interval_days: Option<u64>,
    snooze_until: Option<NaiveDateTime>,
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
            recur_interval_days: None,
            snooze_until: None
        }
    }

    #[must_use]
    pub fn detail_string(&self) -> String {
        let mut output = String::new();
        output.push_str("uuid: ");
        output.push_str(&self.uuid.to_string());
        output.push('\n');
        output.push_str("completed: ");
        if let Some(completed) = self.completed {
            output.push_str(&completed.to_string());
        } else {
            output.push_str("None");
        }
        output.push('\n');
        output.push_str("recur_next: ");
        if let Some(recur_next) = self.recur_next {
            output.push_str(&recur_next.to_string());
        } else {
            output.push_str("None");
        }
        output.push('\n');
        output.push_str("recur_interval_days: ");
        if let Some(recur_interval_days) = self.recur_interval_days {
            output.push_str(&recur_interval_days.to_string());
        } else {
            output.push_str("None");
        }
        output.push('\n');
        output.push_str("snooze_until: ");
        if let Some(snooze_until) = self.snooze_until {
            output.push_str(&snooze_until.to_string());
        } else {
            output.push_str("None");
        }
        output.push('\n');
        output
    }

    /// Sets the task to recur daily
    ///
    /// # Panics
    ///
    /// Will panic if some apparently impossible things happen!
    pub fn set_recur_daily(&mut self) {
        self.recur_interval_days = Some(1);
    }

    fn in_n_days_5am(interval: u64) -> NaiveDateTime {
        let mut next_5am = NaiveDateTime::new(Local::now().date_naive(),
        NaiveTime::from_hms_opt(5,0,0).expect("5am not a valid time!"));
        if next_5am < Local::now().naive_local() {
            next_5am = next_5am.checked_add_days(Days::new(1)).expect("Couldn't add 1 day in set_recur_daily");
        }
        next_5am.checked_add_days(Days::new(interval - 1)).expect("Couldn't add days in set_recur_daily")
    }

    /// Update ``recur_next`` field for recurring tasks
    fn recur_next(&mut self) {
        if let Some(interval) = self.recur_interval_days {
            self.recur_next = Some(Task::in_n_days_5am(interval));
        }
    }

    #[must_use]
    pub fn is_recurring(&self) -> bool {
        self.recur_interval_days.is_some()
    }

    pub fn clear_recur(&mut self) {
        self.recur_next = None;
        self.recur_interval_days = None;
    }

    pub fn snooze_tomorrow(&mut self) {
        self.snooze_until = Some(Task::in_n_days_5am(1));
    }

    /// Snooze a task for 1 second, for testing purposes
    ///
    /// # Panics
    ///
    /// Will panic if some apparently impossible things happen!
    pub fn snooze_1s(&mut self) {
        let mut snooze_to = NaiveDateTime::new(Local::now().date_naive(), Local::now().time());
        snooze_to = snooze_to.checked_add_signed(TimeDelta::seconds(1)).expect("Can't add 1 sec in snooze_1s");
        self.snooze_until = Some(snooze_to);
    }

    /// Returns true if a task is currently marked as snoozed, but the snooze time
    /// is before the current time, and has therefore expired
    #[must_use]
    pub fn snooze_expiring(&self) -> bool {
        if let Some(snooze_until) = self.snooze_until {
            Local::now().naive_local() >= snooze_until
        } else {
            false
        }
    }

    pub fn unsnooze(&mut self) {
        self.snooze_until = None;
    }

    /// Returns true if the task is not current - i.e. is not complete
    /// but is not currently eligible to be displayed - because it has
    /// snoozed, or it is a recurring task that we've completed and it
    /// isn't yet time for it to occur again.
    #[must_use]
    pub fn not_current(&self) -> bool {
        if let Some(next) = self.recur_next {
            Local::now().naive_local() < next
        } else if let Some(snooze_until) = self.snooze_until {
            Local::now().naive_local() < snooze_until
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

    pub fn remove_dot(&mut self) {
        self.dot = false;
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

