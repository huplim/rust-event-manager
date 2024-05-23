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



// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_new_event() {
        let event = Event::new();
        assert_eq!(event.description(), "Empty event");
        assert_eq!(event.category(), "");
    }

    #[test]
    fn test_set_date() {
        let mut event = Event::new();
        event.set_date("2022-01-01");
        assert_eq!(event.date(), NaiveDate::from_ymd_opt(2022, 1, 1).unwrap());
    }

    #[test]
    fn test_set_description() {
        let mut event = Event::new();
        event.set_description("Test event");
        assert_eq!(event.description(), "Test event");
    }

    #[test]
    fn test_set_category() {
        let mut event = Event::new();
        event.set_category("Test category");
        assert_eq!(event.category(), "Test category");
    }

    #[test]
    fn test_display_format() {
        let event = Event::new();
        assert_eq!(format!("{}", event), format!("{}: {} ({})", event.date(), event.description(), event.category()));
    }

    #[test]
    fn test_set_long_description() {
        let mut event = Event::new();
        event.set_description("This is a very long description that exceeds the maximum allowed length of 200 characters. This description should be truncated.");
        assert_eq!(event.description(), "This is a very long description that exceeds the maximum allowed length of 200 characters. This description should be truncated.");
    }

    #[test]
    fn test_set_long_category() {
        let mut event = Event::new();
        event.set_category("This is a very long category name that exceeds the maximum allowed length of 30 characters. This category name should be truncated.");
        assert_eq!(event.category(), "This is a very long category name that exceeds the maximum allowed length of 30 characters. This category name should be truncated.");
    }
}