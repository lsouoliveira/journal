use crate::api;
use crate::date_parser;
use crate::pager;

use chrono::Local;
use clap::{arg, command, Arg, ArgMatches, Command};

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
            Some(("clear", _)) => self.clear(),
            Some(("delete", sub_matches)) => self.delete_entry(sub_matches.clone()),
            _ => self.list_entries(),
        }
    }

    pub fn build(client: api::Client) -> Application {
        let matches = command!()
            .arg(Arg::new("since").short('s').long("since"))
            .subcommand(
                Command::new("add")
                    .about("Adds a new entry")
                    .arg(arg!(<message> "Entry message")),
            )
            .subcommand(Command::new("clear").about("Deletes all entries"))
            .subcommand(
                Command::new("delete")
                    .about("Deletes an entry")
                    .arg(arg!(<id> "Entry ID")),
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

    fn clear(&mut self) {
        self.client.entries_service.destroy_all()
    }

    fn list_entries(&mut self) {
        let params = self.build_list_entries_params();

        let entries = self.client.entries_service.all(params);
        let mut output: String = String::new();

        for (index, entry) in entries.iter().enumerate() {
            if index > 0 {
                output += "\n\n";
            }

            output += &format_entry(&entry);
        }

        pager::start_pager(&output)
    }

    fn delete_entry(&mut self, matches: ArgMatches) {
        let id = matches.get_one::<String>("id").unwrap();
        let entry_id = uuid::Uuid::parse_str(&id).unwrap();
        let entry_destroy = api::EntryDestroy { id: entry_id };

        self.client.entries_service.delete_entry(entry_destroy);

        println!("Deleted entry {}", id);
    }

    fn build_list_entries_params(&mut self) -> api::ListEntriesParams {
        let mut params = api::ListEntriesParams::default();

        if let Some(since) = self.matches.get_one::<String>("since") {
            params.since = match date_parser::parse_date(&since) {
                Ok(date) => Some(date),
                _ => None,
            }
        }

        params
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
