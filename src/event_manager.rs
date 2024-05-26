pub mod event;
use csv::Error;
use std::fs::File;
use chrono::NaiveDate;

pub struct EventManager {
    event_list: Vec<event::Event>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            event_list: Vec::new(),
        }
    }

    // Add an event to the event list
    pub fn add_event(&mut self, e: event::Event) {
        self.event_list.push(e);
    }

    // Fetch all event indices that match the given information
    // If all values are None, all event indices are returned
    pub fn fetch_events(&self, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, desc: Option<String>, cate: Vec<Option<String>>, excl: bool) -> Vec<usize> {
        let mut event_indices = Vec::new();
        for (index, e) in self.event_list.iter().enumerate() {
            let date_matches = match (start_date, end_date) {
                (Some(start), Some(end)) if start == end => e.date() == start,
                (Some(start), Some(end)) => e.date() >= start && e.date() < end,
                (Some(start), None) => e.date() >= start,
                (None, Some(end)) => e.date() < end,
                (None, None) => true,
            };
            // If exclusion is true, the category must not be in the list
            let cate_matches;
            if excl {
                cate_matches = cate.is_empty() || !cate.contains(&Some(e.category().clone()));
            } else {
                cate_matches = cate.is_empty() || cate.contains(&Some(e.category().clone()));
            }
            if date_matches
            && desc.as_deref().map_or(true, |des| e.description().starts_with(des))
            && cate_matches {
                event_indices.push(index);
            }
        }
        event_indices
    }

    // Delete all events that match the given indices
    pub fn delete_event(&mut self, delete_indices: Vec<usize>) {
        for index in delete_indices.iter().rev() {
            self.event_list.remove(*index);
        }
    }

    // Print all event indices that match the given information
    // Works like delete_event, but prints the events instead of deleting them
    pub fn print_events(&self, print_indices: Vec<usize>) {
        for index in print_indices {
            println!("{}", self.event_list[index]);
        }
    }

    // Import events from a CSV file
    // Delimiter is ","
    pub fn import_csv(&mut self, file_name: &str) -> Result<(), Error> {
        let file = File::open(file_name)?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        for result in rdr.records() {
            let record = result?;
            let date = &record[0];
            let description = &record[1];
            let category = &record[2];

            let mut e = event::Event::new();
            e.set_date(date);
            e.set_description(description);
            e.set_category(category);
            self.add_event(e);
        }

        Ok(())
    }

    // Export events to a CSV file
    // Delimiter is ","
    pub fn export_csv(&self, file_name: &str) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(file_name)?;

        wtr.write_record(&["date", "description", "category"])?;
        for e in &self.event_list {
            wtr.write_record(&[&e.date().to_string(), &e.description(), &e.category()])?;
        }

        wtr.flush()?;

        Ok(())
    }
}

// Using 'lifetime to specify the lifetime of the iterator
// This requires slice::Iter to be used
impl<'lifetime> IntoIterator for &'lifetime EventManager {
    type Item = &'lifetime event::Event;
    type IntoIter = std::slice::Iter<'lifetime, event::Event>;

    fn into_iter(self) -> Self::IntoIter {
        self.event_list.iter()
    }
}



// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_event_manager() {
        let event_manager = EventManager::new();
        assert_eq!(event_manager.event_list.len(), 0);
    }
    
    #[test]
    fn test_add_event() {
        let mut event_manager = EventManager::new();
        let event = event::Event::new();
        event_manager.add_event(event);
        assert_eq!(event_manager.event_list.len(), 1);
    }
    
    #[test]
    fn test_fetch_events_all() {
        let mut event_manager = EventManager::new();
        let event1 = event::Event::new();
        let event2 = event::Event::new();
        event_manager.add_event(event1);
        event_manager.add_event(event2);
        
        let indices = event_manager.fetch_events(None, None, None, vec![], false);
        assert_eq!(indices.len(), 2);
    }
    
    #[test]
    fn test_delete_event() {
        let mut event_manager = EventManager::new();
        let event1 = event::Event::new();
        let event2 = event::Event::new();
        event_manager.add_event(event1);
        event_manager.add_event(event2);
        
        event_manager.delete_event(vec![0]);
        assert_eq!(event_manager.event_list.len(), 1);
    }
    
    #[test]
    fn test_print_events() {
        let mut event_manager = EventManager::new();
        let event1 = event::Event::new();
        let event2 = event::Event::new();
        event_manager.add_event(event1);
        event_manager.add_event(event2);
        
        let print_indices = vec![0, 1];
        event_manager.print_events(print_indices);
        // Assert that the events are printed correctly (output can't be tested here)
    }
}