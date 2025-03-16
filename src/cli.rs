use crate::api;
use crate::pager;

use chrono::Local;
use clap::{arg, command, ArgMatches, Command};

pub struct Application {
    matches: ArgMatches,
    client: api::Client,
}

impl Application {
    pub fn new(matches: ArgMatches, client: api::Client) -> Application {
        Application { matches, client }
    }

    pub fn run(&mut self) {
        match self.matches.subcommand() {
            Some(("add", sub_matches)) => self.add_entry(sub_matches.clone()),
            _ => self.list_entries(),
        }
    }

    pub fn build(client: api::Client) -> Application {
        let matches = command!()
            .subcommand(
                Command::new("add")
                    .about("Adds a new entry")
                    .arg(arg!(<message> "Entry message")),
            )
            .get_matches();

        Application::new(matches, client)
    }

    fn add_entry(&mut self, matches: ArgMatches) {
        let message = matches.get_one::<String>("message").unwrap();

        let new_entry = api::EntryCreate {
            message: message.to_string(),
        };

        self.client.entries_service.create_entry(new_entry)
    }

    fn list_entries(&mut self) {
        let entries = self.client.entries_service.all();
        let mut output: String = String::new();

        for (index, entry) in entries.iter().enumerate() {
            if index > 0 {
                output += "\n\n";
            }

            output += &format_entry(&entry);
        }

        pager::start_pager(&output)
    }
}

fn format_entry(entry: &api::EntryRead) -> String {
    let output = format!(
        "\x1b[38;5;214mentry {}\x1b[0m\nDate: {}\n\n    {}",
        entry.id.to_string(),
        entry
            .created_at
            .with_timezone(&Local)
            .format("%a %b %d %H:%M:%S %Y %z"),
        entry.message
    );

    output
}
