mod event_manager;
use crate::event_manager::event;

fn main() {
    // New vector to push the events into
    let mut events: event_manager::EventManager = event_manager::EventManager::new();

    if let Err(err) = events.import_csv("data.csv") {
        println!("Error importing CSV: {}", err);
    }

    events.add_event(event::Event::new());

    // Iterate over the vector and print each event
    // &mut events if modifying is needed or &events if not
    for event in &mut events {
        // events.iter() or .iter_mut()
        print!("{}\n", event);
    }
}
