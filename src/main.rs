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

    /*
    // All events from csv file
    println!("All events:");
    events.print_events(None, None, None, None);
    println!("\n");

    // Print events from 2000-01-01 to 2015-12-31
    println!("Events from 2000-01-01 to 2015-12-31:");
    events.print_events(NaiveDate::from_ymd_opt(2000, 1, 1), NaiveDate::from_ymd_opt(2015, 12, 31), None, None);

    // Deletes Microsoft events
    events.delete_event(None, None, None, Some("Microsoft"));
    // Deletes 2010-04-03 events
    events.delete_event(NaiveDate::from_ymd_opt(2010, 4, 3), None, None, None);
    // Deletes Nintendo Switch released event
    events.delete_event(None, None, Some("Nintendo Switch released"), None);
    // Deletes 1991-08-06;"World Wide Web goes live";"Internet"
    events.delete_event(NaiveDate::from_ymd_opt(1991, 8, 6), None, Some("World Wide Web goes live"), Some("Internet"));

    // All events after deletion
    println!("\nAll events after deletion:");
    events.print_events(None, None, None, None);
    println!("\n");

    // Print Intel events
    println!("Intel events:");
    events.print_events(None, None, None, Some("Intel"));
    println!("\n");

    // Print Google events
    println!("Google events:");
    events.print_events(None, None, None, Some("Google"));
    println!("\n");

    // Print 2007-06-29 events
    println!("2007-06-29 events:");
    events.print_events(NaiveDate::from_ymd_opt(2007, 6, 29), None, None, None);
    println!("\n");

    // Print "Facebook founded" events
    println!("\"Facebook founded\" events:");
    events.print_events(None, None, Some("Facebook founded"), None);
    println!("\n");

    */

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

    // Fetch and delete "Microsoft" events
    let fetched_event_indices = events.fetch_events(None, None, None, Some("Microsoft"));
    events.delete_event(fetched_event_indices);

    // Fetch and print "Google" events
    println!("Google events: ");
    let print_event_indices = events.fetch_events(None, None, None, Some("Google"));
    events.print_events(print_event_indices);
    println!("\n");

    // Fetch and delete 2007-06-29 events
    let fetched_event_indices = events.fetch_events(NaiveDate::from_ymd_opt(2007, 6, 29), None, None, None);
    events.delete_event(fetched_event_indices);

    // Export events to CSV file
    if let Err(err) = events.export_csv("data.csv") {
        println!("Error exporting CSV: {}", err);
    }
}
