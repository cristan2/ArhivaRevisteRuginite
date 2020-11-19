use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

#[derive(Debug)]
pub struct Revista {
    id: u32,
    nume: String,
    alias: String,
    status: String,
    tip: String,
    perioada: String,
    aparitii: String,
    descriere: String,
    link: String,
    observatii: String,
}

impl Revista {
    pub fn from_row(row: &rusqlite::Row) -> Revista {
        Revista {
            id: row.get("revista_id").unwrap(),
            nume: row.get("revista_nume").unwrap_or("".to_string()),
            alias: row.get("revista_alias").unwrap_or("".to_string()),
            status: row.get("status").unwrap_or("".to_string()),
            tip: row.get("tip").unwrap_or("".to_string()),
            perioada: row.get("perioada").unwrap_or("".to_string()),
            aparitii: row.get("aparitii").unwrap_or("".to_string()),
            descriere: row.get("descriere").unwrap_or("".to_string()),
            link: row.get("link_oficial").unwrap_or("".to_string()),
            observatii: row.get("observatii").unwrap_or("".to_string()),
        }
    }
}

pub struct DBConnection {
    db_name: String,
    connection: rusqlite::Connection
}

impl DBConnection {
    pub fn open(nume_db: &str) -> DBConnection {
        DBConnection {
            db_name: nume_db.to_string(),
            connection: Connection::open(nume_db).unwrap()
        }
    }

    pub fn query_reviste(&self) -> Vec<Result<Revista>> {
        let mut stmt_revista = self.connection.prepare("SELECT * FROM reviste").unwrap();
        stmt_revista.query_map(NO_PARAMS, |row| Ok(Revista::from_row(row)))
            .unwrap()
            .collect()
    }
}
