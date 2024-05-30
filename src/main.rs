mod event_manager;
use crate::event_manager::EventManager;
use event_manager::event::Event;
use chrono::NaiveDate;
use home::home_dir;
use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "days",
    version = "0.1.0",
    author = "Miika Hupli",
    about = "A simple CLI event manager",
)]
struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
enum Command {
    // List command has optional arguments:
    // date, before_date, after_date, today, categories, exclude
    #[clap(name = "list", about = "--date, --before-date, --after-date, --today, --categories, --exclude")]
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
    // Add command has required arguments: description, category
    // and an optional argument: date
    #[clap(name = "add", about = "--date, --description, --category")]
    Add {
        #[clap(long)]
        date: Option<NaiveDate>,
        #[clap(short, long)]
        description: String,
        #[clap(long)]
        category: String,
    },
    // Delete command has optional arguments: date, description, category
    // and an optional flag: dry-run, all
    #[clap(name = "delete", about = "--dry-run, --all, --date, --description, --category")]
    Delete {
        #[clap(long = "dry-run")] // Can't use hyphen in variable name
        dry_run: bool,
        #[clap(long)]
        all: bool,
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
    let mut events = EventManager::new();

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
                // If date is given, use that date for both before and after
                // This is because fetch_events works in a specific way
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

            let new_event = Event::new_with_values(given_date.unwrap(), &description, &category);
            events.add_event(new_event);

            // Export events to CSV file
            let file_path = format!("{}/.days/events.csv", home_path);
            if let Err(err) = events.export_csv(&file_path) {
                println!("Error exporting CSV: {}", err);
            }
        }

        Command::Delete { dry_run, date, description, category, all } => {
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

            // If all is not true, one other argument must be given
            if given_date.is_some() || given_description.is_some() || !given_categories.is_empty() || all {
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
    }
}
