use chrono::{NaiveDate, Local};
use std::fmt;

pub struct Event {
    date: NaiveDate,
    description: String,
    category: String,
}

impl Event {
    pub fn new() -> Event {
        // Default values
        Event {
            date: Local::now().naive_local().into(),
            description: String::from("Empty event"),
            category: String::from(""),
        }
    }
    // Getters and setters

    // Easier to use strings for setting the date (from CSV for example)
    pub fn set_date(&mut self, d: &str) {
        match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
            Ok(date) => self.date = date,
            Err(e) => eprintln!("Failed to parse date: {}", e),
        }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn set_description(&mut self, d: &str) {
        if self.description.len() <= 200 {
            self.description = d.to_string();
        } else {
            eprint!("Description too long");
        }
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn set_category(&mut self, c: &str) {
        if self.category.len() <= 30 {
            self.category = c.to_string();
        } else {
            eprint!("Category too long");
        }
    }

    pub fn category(&self) -> String {
        self.category.clone()
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})", self.date(), self.description(), self.category())
    }
}