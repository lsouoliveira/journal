use crate::api;
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
        println!("list entries")
    }
}
