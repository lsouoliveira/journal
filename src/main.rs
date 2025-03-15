mod api;
mod cli;

fn main() {
    let mut application = build_application();
    application.run()
}

fn build_application() -> cli::Application {
    let entries_service = api::EntriesService::new();
    let client = api::Client::new(entries_service);

    cli::Application::build(client)
}
