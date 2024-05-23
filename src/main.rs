mod event_manager;
use crate::event_manager::event;
use chrono::NaiveDate;

fn main() {
    // New vector to push the events into
    let mut events: event_manager::EventManager = event_manager::EventManager::new();

    // Import events from CSV file
    if let Err(err) = events.import_csv("test-data.csv") {
        println!("Error importing CSV: {}", err);
    }

    // Add default event
    events.add_event(event::Event::new());

    // Iterate over the vector and print each event
    println!("\n");
    for event in &events {
        print!("{}\n", event);
    }
    
    // Deletes Microsoft events
    events.delete_event(None, None, Some("Microsoft"));
    // Deletes 2010-04-03 events
    events.delete_event(NaiveDate::from_ymd_opt(2010, 4, 3), None, None);
    // Deletes Nintendo Switch released event
    events.delete_event(None, Some("Nintendo Switch released"), None);
    // Deletes 1991-08-06;"World Wide Web goes live";"Internet"
    events.delete_event(NaiveDate::from_ymd_opt(1991, 8, 6), Some("World Wide Web goes live"), Some("Internet"));

    println!("\n");
    for event in &events {
        print!("{}\n", event);
    }
    println!("\n");

    // Export events to CSV file
    if let Err(err) = events.export_csv("data.csv") {
        println!("Error exporting CSV: {}", err);
    }
}
