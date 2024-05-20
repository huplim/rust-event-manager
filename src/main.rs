mod event_manager;
use crate::event_manager::event;
use chrono::NaiveDate;

fn main() {
    // New vector to push the events into
    let mut events: event_manager::EventManager = event_manager::EventManager::new();

    if let Err(err) = events.import_csv("data.csv") {
        println!("Error importing CSV: {}", err);
    }

    events.add_event(event::Event::new());
    
    // Deletes Microsoft events
    events.delete_event(None, None, Some("Microsoft"));
    // Deletes 2010-04-03 events
    events.delete_event(NaiveDate::from_ymd_opt(2010, 4, 3), None, None);
    // Deletes Nintendo Switch released event
    events.delete_event(None, Some("Nintendo Switch released"), None);
    // Deletes 1991-08-06;"World Wide Web goes live";"Internet"
    events.delete_event(NaiveDate::from_ymd_opt(1991, 8, 6), Some("World Wide Web goes live"), Some("Internet"));

    // Iterate over the vector and print each event
    // &mut events if modifying is needed or &events if not
    for event in &mut events {
        // events.iter() or .iter_mut()
        print!("{}\n", event);
    }
}
