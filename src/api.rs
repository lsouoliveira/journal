use chrono::{DateTime, Local, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct EntriesService {
    conn: rusqlite::Connection,
}

struct Entry {
    id: Option<Uuid>,
    message: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct EntryRead {
    pub id: Uuid,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct EntryCreate {
    pub message: String,
}

pub struct EntryDestroy {
    pub id: Uuid,
}

pub struct EntryPagination {
    pub items: Vec<EntryRead>,
    pub total_items: u32,
    pub page: u32,
    pub total_pages: u32,
}

pub struct ListEntriesParams {
    pub since: Option<DateTime<Local>>,
}

impl Default for ListEntriesParams {
    fn default() -> ListEntriesParams {
        ListEntriesParams { since: None }
    }
}

impl EntriesService {
    pub fn new(conn: rusqlite::Connection) -> EntriesService {
        EntriesService { conn }
    }

    pub fn create_entry(&mut self, entry_create: EntryCreate) {
        let new_entry = Entry {
            id: Some(Uuid::new_v4()),
            message: Some(entry_create.message),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        let query = "
            INSERT INTO journal_entries (id, message, created_at, updated_at) VALUES (?, ?, ?, ?)
        ";

        self.conn
            .execute(
                query,
                (
                    &new_entry.id.unwrap().to_string(),
                    &new_entry.message.unwrap().to_string(),
                    &new_entry.created_at.unwrap().to_rfc3339(),
                    &new_entry.updated_at.unwrap().to_rfc3339(),
                ),
            )
            .unwrap();
    }

    pub fn list_entries(&mut self, page: u32, limit: u32) -> EntryPagination {
        if page <= 0 {
            panic!("Page should be positive: {}", page);
        }

        if limit <= 0 {
            panic!("Limit should be positive: {}", limit);
        }

        let mut result: Vec<EntryRead> = vec![];
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, message, created_at, updated_at FROM journal_entries ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .unwrap();

        let entries = stmt
            .query_map([limit, (page - 1) * limit], |row| {
                let id: String = row.get(0).unwrap();

                Ok(EntryRead {
                    id: Uuid::parse_str(&id).unwrap(),
                    message: row.get(1).unwrap(),
                    created_at: row.get(2).unwrap(),
                    updated_at: row.get(3).unwrap(),
                })
            })
            .unwrap();

        for entry in entries {
            result.push(entry.unwrap());
        }

        let mut stmt = self
            .conn
            .prepare("SELECT COUNT(*) FROM journal_entries")
            .unwrap();

        let total_items: u32 = stmt.query_row([], |row| Ok(row.get(0).unwrap())).unwrap();

        EntryPagination {
            items: result,
            total_items,
            page,
            total_pages: total_items / limit,
        }
    }

    pub fn all(&mut self, params: ListEntriesParams) -> Vec<EntryRead> {
        let mut result: Vec<EntryRead> = vec![];
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, message, created_at, updated_at 
                FROM journal_entries 
                WHERE (?1 IS NULL or datetime(created_at) >= datetime(?1)) ORDER BY created_at DESC
            ",
            )
            .unwrap();

        let since = if let Some(since) = params.since {
            Some(since.to_utc())
        } else {
            None
        };

        let entries = stmt
            .query_map([since], |row| {
                let id: String = row.get(0).unwrap();
                Ok(EntryRead {
                    id: Uuid::parse_str(&id).unwrap(),
                    message: row.get(1).unwrap(),
                    created_at: row.get(2).unwrap(),
                    updated_at: row.get(3).unwrap(),
                })
            })
            .unwrap();

        for entry in entries {
            result.push(entry.unwrap());
        }

        result
    }

    pub fn delete_entry(&mut self, entry_destroy: EntryDestroy) {
        let query = "
            DELETE FROM journal_entries WHERE id = ?;
        ";

        self.conn
            .execute(query, [entry_destroy.id.to_string()])
            .unwrap();
    }

    pub fn destroy_all(&mut self) {
        let query = "
            DELETE FROM journal_entries;
        ";

        self.conn.execute(query, ()).unwrap();
    }
}

#[derive(Debug)]
pub struct Client {
    pub entries_service: EntriesService,
}

impl Client {
    pub fn new(entries_service: EntriesService) -> Client {
        Client { entries_service }
    }
}
