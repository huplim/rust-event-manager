pub mod event;
use csv::Error;
use std::fs::File;

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

    pub fn import_csv(&mut self, file_name: &str) -> Result<(), Error> {
        let file = File::open(file_name)?;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
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
}

impl Iterator for EventManager {
    type Item = event::Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.event_list.pop()
    }
}