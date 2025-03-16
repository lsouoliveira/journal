mod api;
mod cli;
mod db;
mod gui;
mod pager;

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if !Path::new(&db::db_path()).exists() {
        setup_database();
    }

    if args.contains(&"--gui".to_string()) {
        gui::Journal::new().run();
    } else {
        let mut cli_application = build_application();
        cli_application.run()
    }
}

fn build_application() -> cli::Application {
    let conn = db::connect();
    let entries_service = api::EntriesService::new(conn);
    let client = api::Client::new(entries_service);

    cli::Application::build(client)
}

fn setup_database() {
    let conn = db::connect();
    let query = "
        CREATE TABLE journal_entries (
            id TEXT PRIMARY KEY,
            message TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    ";
    conn.execute(query, ()).unwrap();
}
