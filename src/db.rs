use rusqlite::{Connection, Result, Statement};
use rusqlite::NO_PARAMS;

// TODO generic API
// - error handling
// - parse row for Option<T>: null or empty string -> None
// - parse row for T (NOT NULL): empty string -> ???

#[derive(Debug)]
pub struct Revista {
    id: i32,
    nume: String,
    alias: Option<String>,
    status: Option<String>,
    tip: Option<String>,
    perioada: Option<String>,
    aparitii: Option<String>,
    descriere: Option<String>,
    link: Option<String>,
    observatii: Option<String>,
}

impl Revista {
    pub fn from_row(row: &rusqlite::Row) -> Revista {
        Revista {
            id: row.get("revista_id").unwrap(),
            nume: row.get("revista_nume").unwrap(),
            alias: row.get("revista_alias").ok(),
            status: row.get("status").ok(),
            tip: row.get("tip").ok(),
            perioada: row.get("perioada").ok(),
            aparitii: row.get("aparitii").ok(),
            descriere: row.get("descriere").ok(),
            link: row.get("link_oficial").ok(),
            observatii: row.get("observatii").ok(),
        }
    }
}

#[derive(Debug)]
pub struct Editie {
    editie_id: i32,
    revista_id: i32,
    tip: Option<String>,
    parinte_editie_id: Option<String>,
    numar: i32,
    an: i32,
    luna: i32,
    luna_sfarsit: Option<String>,
    pret: Option<i32>,
    nr_pagini: Option<i32>,
    disc_demo: Option<String>,
    joc_complet: Option<String>,
    redactor_sef: Option<String>,
    editie_link_oficial: Option<String>,
    editie_observatii: Option<String>,
    scan_info_nr_pg: Option<i32>,
    scan_info_pg_lipsa: Option<String>,
    scan_info_observatii: Option<String>,
    scan_info_credits: Option<String>
}

impl Editie {
    pub fn from_row(row: &rusqlite::Row) -> Editie {
        Editie {
            editie_id: row.get("editie_id").unwrap(),
            revista_id: row.get("revista_id").unwrap(),
            tip: row.get("tip").unwrap(),
            parinte_editie_id: row.get("parinte_editie_id").unwrap(),
            numar: row.get("numar").unwrap_or(99),  // error ???
            an: row.get("an").unwrap(),
            luna: row.get("luna").unwrap_or(99),  // de inlocuit
            luna_sfarsit: row.get("luna_sfarsit").ok(),
            pret: row.get("pret").ok(),   // in DB e NUMERIC  // error
            nr_pagini: row.get("nr_pagini").ok(),
            disc_demo: row.get("disc_demo").ok(),
            joc_complet: row.get("joc_complet").ok(),
            redactor_sef: row.get("redactor_sef").ok(),
            editie_link_oficial: row.get("editie_link_oficial").ok(),
            editie_observatii: row.get("editie_observatii").ok(),
            scan_info_nr_pg: row.get("scan_info_nr_pg").ok(),
            scan_info_pg_lipsa: row.get("scan_info_pg_lipsa").ok(),
            scan_info_observatii: row.get("scan_info_observatii").ok(),
            scan_info_credits: row.get("scan_info_credits").ok(),
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

    fn prepare_statement(&self, query_string: &str) -> rusqlite::Statement {
        self.connection.prepare(query_string).unwrap()
    }

    // TODO move queries to API trait

    /* --- Reviste --- */

    pub fn retrieve_toate_revistele(&self) -> Vec<Result<Revista>> {
        let mut stmt_reviste: Statement = self.prepare_statement(
            "
            SELECT * FROM reviste
            ");

        stmt_reviste
            .query_map(NO_PARAMS, |row| Ok(Revista::from_row(row)))
            .unwrap()
            .collect()
    }

    pub fn retrieve_revista(&self, revista_id: &i32) -> Result<Revista> {
        let mut stmt_revista: Statement = self.prepare_statement(
            "SELECT revista_id, revista_nume, aparitii
            FROM reviste
            WHERE revista_id = ?1"
        );

        let revista = stmt_revista
            .query_map(&[revista_id], |row| Ok(Revista::from_row(row)))?
            .next()
            .unwrap();

        revista
    }

    /* --- Editii --- */
    pub fn retrieve_toate_editiile(&self) -> Vec<Result<Editie>> {
        let mut stmt_editii: Statement = self.prepare_statement(
            "
            SELECT * FROM editii
            ");

        stmt_editii
            .query_map(NO_PARAMS, |row| Ok(Editie::from_row(row)))
            .unwrap()
            .collect()
    }
}
