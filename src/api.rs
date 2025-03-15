use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct EntriesService {}

struct Entry {
    id: Option<Uuid>,
    created_at: Option<DateTime<Utc>>,
    message: Option<String>,
}

pub struct EntryRead {
    id: Uuid,
    created_at: Option<DateTime<Utc>>,
    message: String,
}

pub struct EntryCreate {
    pub message: String,
}

pub struct EntryPagination {
    items: Vec<EntryRead>,
    total_items: u32,
    page: u32,
    total_pages: u32,
}

impl EntriesService {
    pub fn new() -> EntriesService {
        EntriesService {}
    }

    pub fn create_entry(&mut self, entry_create: EntryCreate) {
        println!("create entry")
    }

    pub fn list_entries(&mut self, page: u32) -> EntryPagination {
        EntryPagination {
            items: vec![],
            total_items: 0,
            page: 1,
            total_pages: 0,
        }
    }
}

pub struct Client {
    pub entries_service: EntriesService,
}

impl Client {
    pub fn new(entries_service: EntriesService) -> Client {
        Client { entries_service }
    }
}
