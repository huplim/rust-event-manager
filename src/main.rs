mod event_manager;
use crate::event_manager::event;
use chrono::{NaiveDate, Local};
use home::home_dir;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "list")]
    List {
        #[clap(long)]
        date: Option<NaiveDate>,
        #[clap(long)]
        before_date: Option<NaiveDate>,
        #[clap(long)]
        after_date: Option<NaiveDate>,
        #[clap(long)] // No value needed
        today: bool,
        #[clap(long)]
        category: Option<String>,
        #[clap(long)]
        description: Option<String>,
    },
    #[clap(name = "add")]
    Add {
        #[clap(long)]
        date: Option<NaiveDate>,
        #[clap(long)]
        description: String,
        #[clap(long)]
        category: String,
    },
    #[clap(name = "delete")]
    Delete {
        #[clap(long = "dry-run")] // Can't use hyphen in variable name
        dry_run: bool,
        #[clap(long)]
        date: Option<NaiveDate>,
        #[clap(long)]
        description: Option<String>,
        #[clap(long)]
        category: Option<String>,
    },
}

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

    let args = Cli::parse();
    match args.command {
        Command::List { date, before_date, after_date, today, category, description } => {
            println!("Listing events...");
            if today {
                let today = chrono::Local::today().naive_local();
                println!("Today is: {}", today);
            }
            else if let Some(date) = date {
                println!("Date: {}", date);
            }
            else {
                if let Some(date) = before_date {
                    println!("Before date: {}", date);
                }
                if let Some(date) = after_date {
                    println!("After date: {}", date);
                }
            }

            if let Some(category) = category {
                println!("Category: {}", category);
            }
            if let Some(description) = description {
                println!("Description: {}", description);
            }
        }

        Command::Add { date, description, category } => {
            println!("Adding event...");
            if let Some(date) = date {
                println!("Date: {}", date);
            }
            println!("Description: {}", description);
            println!("Category: {}", category);
        }

        Command::Delete { dry_run, date, description, category } => {
            println!("Deleting events...");
            if let Some(date) = date {
                println!("Date: {}", date);
            }
            if let Some(category) = category {
                println!("Category: {}", category);
            }
            if let Some(description) = description {
                println!("Description: {}", description);
            }
            if dry_run {
                println!("Dry run");
            }
        }
    }




    // Some old testing code
    /*
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
    */
}
