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

    pub fn add_event(&mut self, e: event::Event) {
        self.event_list.push(e);
    }

    // Delete all events that match the given information
    pub fn delete_event(&mut self, date: Option<NaiveDate>, desc: Option<&str>, cate: Option<&str>) {
        let mut index_list = Vec::new();
        for (i, e) in self.event_list.iter().enumerate() {
            if date.map_or(true, |date| e.date() == date) 
            && desc.map_or(true, |desc| e.description() == desc)
            && cate.map_or(true, |cate| e.category() == cate) {
                index_list.push(i);
            }
        }
        for i in index_list.iter().rev() {
            self.event_list.remove(*i);
        }
        
    }

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
    fn test_add_event() {
        let mut event_manager = EventManager::new();
        let event = event::Event::new();
        event_manager.add_event(event);

        assert_eq!(event_manager.event_list.len(), 1);
    }

    #[test]
    fn test_delete_event() {
        let mut event_manager = EventManager::new();
        let event1 = event::Event::new();
        let event2 = event::Event::new();
        let event3 = event::Event::new();

        event_manager.add_event(event1);
        event_manager.add_event(event2);
        event_manager.add_event(event3);

        event_manager.delete_event(None, None, None);

        assert_eq!(event_manager.event_list.len(), 0);
    }
}