mod event_manager;
use event_manager::event;
use chrono::NaiveDate;
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
        categories: Option<String>,
        #[clap(long = "exclude")]
        exclusion: bool,
    },
    #[clap(name = "add")]
    Add {
        #[clap(long)]
        date: Option<NaiveDate>,
        #[clap(short, long)]
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
    let file_path = format!("{}/.days/events.csv", home_path);
    if let Err(err) = events.import_csv(&file_path) {
        println!("Error importing CSV: {}", err);
    }

    // Execute commands based on CLI arguments
    let args = Cli::parse();
    match args.command {
        Command::List { date, before_date, after_date, today, categories, exclusion } => {
            let mut given_before_date = None;
            let mut given_after_date = None;
            let mut given_categories = Vec::new();

            if today {
                given_before_date = Some(chrono::Local::now().date_naive());
                given_after_date = Some(chrono::Local::now().date_naive());
            }
            else if let Some(date) = date {
                given_before_date = Some(date);
                given_after_date = Some(date);
            }
            else {
                if let Some(date) = before_date {
                    given_before_date = Some(date);
                }
                if let Some(date) = after_date {
                    given_after_date = Some(date);
                }
            }

            // Split the `categories` string into a vector of categories
            if let Some(categories_string) = categories {
                given_categories = categories_string
                    .split(',')
                    .map(|s| Some(s.to_string()))
                    .collect();
            }

            let print_event_indices = events.fetch_events(
                given_after_date,
                given_before_date,
                None, // No description filter
                given_categories,
                exclusion,
            );
            events.print_events(print_event_indices);
        }

        Command::Add { date, description, category } => {
            let given_date;

            // If no date is given, use the current date
            if let Some(date) = date {
                given_date = Some(date);
            }
            else {
                given_date = Some(chrono::Local::now().date_naive());
            }

            let new_event = event::Event::new_with_values(given_date.unwrap(), &description, &category);
            events.add_event(new_event);
            
            // Export events to CSV file
            let file_path = format!("{}/.days/events.csv", home_path);
            if let Err(err) = events.export_csv(&file_path) {
                println!("Error exporting CSV: {}", err);
            }
        }

        Command::Delete { dry_run, date, description, category } => {
            let mut given_date = None;
            let mut given_description = None;
            let mut given_categories = Vec::new();

            if let Some(date) = date {
                given_date = Some(date);
            }
            if let Some(category) = category {
                given_categories.push(Some(category));
            }
            if let Some(description) = description {
                given_description = Some(description);
            }

            let delete_event_indices = events.fetch_events(
                given_date,
                given_date,
                given_description,
                given_categories,
                false,
            );

            // Only print if dry_run is given
            if dry_run {
                events.print_events(delete_event_indices.clone());
            }
            else {
                events.delete_event(delete_event_indices.clone());
                // Export events to CSV file
                let file_path = format!("{}/.days/events.csv", home_path);
                if let Err(err) = events.export_csv(&file_path) {
                    println!("Error exporting CSV: {}", err);
                }
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
