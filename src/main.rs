mod event_manager;
use crate::event_manager::event;
use chrono::NaiveDate;
use home::home_dir;

fn main() {
    // New vector to push the events into
    let mut events: event_manager::EventManager = event_manager::EventManager::new();

    // Import events from CSV file
    let home_path = home_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
    let file_path = format!("{}/.days/test-events.csv", home_path);
    if let Err(err) = events.import_csv(&file_path) {
        println!("Error importing CSV: {}", err);
    }

    // Add default event
    events.add_event(event::Event::new());

    // Print all events
    println!("All events: ");
    let print_event_indices = events.fetch_events(None, None, None, None);
    events.print_events(print_event_indices);
    println!("\n");

    // Fetch and print 1990-01-01 to 1995-12-31 events
    println!("Events from 1990-01-01 to 1995-12-31: ");
    let print_event_indices = events.fetch_events(NaiveDate::from_ymd_opt(1990, 1, 1), NaiveDate::from_ymd_opt(1995, 12, 31), None, None);
    events.print_events(print_event_indices);
    println!("\n");

    // Fetch and print events that start with "Window"
    println!("Events that start with 'Window': ");
    let print_event_indices = events.fetch_events(None, None, Some("Window"), None);
    events.print_events(print_event_indices);
    println!("\n");

    // Fetch and print events that start with "W"
    println!("Events that start with 'W': ");
    let print_event_indices = events.fetch_events(None, None, Some("W"), None);
    events.print_events(print_event_indices);
    println!("\n");

    // Fetch and delete "Microsoft" events
    println!("Deleting Microsoft events: ");
    let fetched_event_indices = events.fetch_events(None, None, None, Some("Microsoft"));
    events.delete_event(fetched_event_indices);
    println!("\n");

    // Fetch and delete 2007-06-29 events
    println!("Deleting 2007-06-29 events: ");
    let fetched_event_indices = events.fetch_events(NaiveDate::from_ymd_opt(2007, 6, 29), None, None, None);
    events.delete_event(fetched_event_indices);
    println!("\n");

    // Fetch and print "Google" events
    println!("Google events: ");
    let print_event_indices = events.fetch_events(None, None, None, Some("Google"));
    events.print_events(print_event_indices);
    println!("\n");

    // Print all events
    println!("All events: ");
    let print_event_indices = events.fetch_events(None, None, None, None);
    events.print_events(print_event_indices);
    println!("\n");

    // Export events to CSV file
    let file_path = format!("{}/.days/events.csv", home_path);
    if let Err(err) = events.export_csv(&file_path) {
        println!("Error exporting CSV: {}", err);
    }
}
