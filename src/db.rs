use rusqlite::{Connection, Result, Statement};
use rusqlite::NO_PARAMS;

// TODO
// - generic API
// - dependency proiect DB + generare din cod
// - error handling
// - parse row for Option<T>: null or empty string -> None
// - parse row for T (NOT NULL): empty string -> ???

fn empty_string() -> String { String::from("")}

#[derive(Debug)]
pub struct Revista {
    pub id: i32,
    pub nume: String,
    pub alias: Option<String>,
    pub status: Option<String>,
    pub tip: Option<String>,
    pub perioada: Option<String>,
    pub aparitii: Option<String>,
    pub descriere: Option<String>,
    pub link: Option<String>,
    pub observatii: Option<String>,
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

    pub fn get_nume_coloana() -> Vec<&'static str> {
        // TODO astea ar trebui sa stea in struct si folosite cumva si la parsarea unui row
        vec![
            "revista_id",
            "revista_nume",
            "revista_alias",
            "status",
            "tip",
            "perioada",
            "aparitii",
            "descriere",
            "link_oficial",
            "observatii"]
    }
}

#[derive(Debug)]
pub struct Editie {
    pub editie_id: i32,
    pub revista_id: i32,
    pub tip: Option<String>,
    pub parinte_editie_id: Option<String>,
    pub numar: Option<i32>, // in DB e NOT NULL, dar poate sa fie si string gol
    pub an: i32,
    pub luna: Option<i32>,  // in DB e NOT NULL, dar poate sa fie si string gol
    pub luna_sfarsit: Option<String>,
    pub pret: Option<i32>,  // in DB e NUMERIC
    pub nr_pagini: Option<i32>,
    pub disc_demo: Option<String>,
    pub joc_complet: Option<String>,
    pub redactor_sef: Option<String>,
    pub editie_link_oficial: Option<String>,
    pub editie_observatii: Option<String>,
    pub scan_info_nr_pg: Option<i32>,
    pub scan_info_pg_lipsa: Option<String>,
    pub scan_info_observatii: Option<String>,
    pub scan_info_credits: Option<String>
}

impl Editie {
    pub fn from_row(row: &rusqlite::Row) -> Editie {
        Editie {
            editie_id: row.get("editie_id").unwrap(),
            revista_id: row.get("revista_id").unwrap(),
            tip: row.get("tip").unwrap(),
            parinte_editie_id: row.get("parinte_editie_id").unwrap(),
            numar: row.get("numar").ok(),
            an: row.get("an").unwrap(),
            luna: row.get("luna").ok(),
            luna_sfarsit: row.get("luna_sfarsit").ok(),
            pret: row.get("pret").ok(),
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

    pub fn get_nume_coloana() -> Vec<&'static str> {
        vec![
            "editie_id",
            "revista_id",
            "tip",
            // "parinte_editie_id",
            "numar",
            "an",
            "luna",
            "luna_sfarsit",
            "pret",
            "nr_pagini",
            "disc_demo",
            "joc_complet",
            "redactor_sef",
            // "editie_link_oficial",
            "editie_observatii",
            // "scan_info_nr_pg",
            // "scan_info_pg_lipsa",
            // "scan_info_observatii",
            "scan_info_credits",]
    }
}

#[derive(Debug)]
pub struct Download {
    pub editie_id: i32,  // TODO trebuie asta?
    pub categorie: String,
    pub item: Option<i32>,
    pub link: Option<String>,
}

impl Download {
    pub fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            editie_id: row.get("editie_id").unwrap(),
            categorie: row.get("categorie").unwrap(),
            item: row.get("revista_alias").ok(),
            link: row.get("link").ok(),
        }
    }

    pub fn get_nume_coloana() -> Vec<&'static str> {
        // TODO astea ar trebui sa stea in struct si folosite cumva si la parsarea unui row
        vec![
            "editie_id",
            "categorie",
            "item",
            "link"
        ]
    }
}

#[derive(Debug)]
pub struct Articol {
    pub articol_id: i32,
    pub revista_id: i32,
    pub revista_nume: String,
    pub editie_id: i32,
    pub editie_an: i32,
    pub editie_luna: Option<i32>,  // in DB e NOT NULL, dar poate sa fie si string gol
    pub editie_luna_sfarsit: Option<String>,
    pub editie_numar: i32,
    pub pg_toc: Option<i32>,
    pub pg_count: Option<i32>,
    pub rubrica: Option<String>,
    pub titlu: Option<String>,
    pub joc_platforma: Option<String>,
    pub autor: Option<String>,
    pub nota: Option<String>,
}

impl Articol {
    pub fn from_row(row: &rusqlite::Row) -> Self {
        Self {
            articol_id: row.get("articol_id").unwrap(),
            revista_id: row.get("revista_id").unwrap(),
            revista_nume: row.get("revista_nume").unwrap(),
            editie_id: row.get("editie_id").unwrap(),
            editie_an: row.get("editie_an").unwrap(),
            editie_luna: row.get("editie_luna").ok(),
            editie_luna_sfarsit: row.get("editie_luna_sfarsit").ok(),
            editie_numar: row.get("editie_numar").unwrap(),
            pg_toc: row.get("pg_toc").ok(),
            pg_count: row.get("pg_count").ok(),
            rubrica: row.get("rubrica").ok(),
            titlu: row.get("titlu").ok(),
            joc_platforma: row.get("joc_platforma").ok(),
            autor: row.get("autor").ok(),
            nota: row.get("nota").ok(),
        }
    }

    pub fn get_nume_coloana() -> Vec<&'static str> {
        // TODO astea ar trebui sa stea in struct si folosite cumva si la parsarea unui row
        vec![
            // "articol_id",
            // "revista_id",
            "revista_nume",
            // "editie_id",
            "editie_an",
            "editie_luna",
            // "editie_luna_sfarsit",
            "editie_numar",
            // "pg_toc",
            // "pg_count",
            "rubrica",
            "titlu",
            // "joc_platforma",
            "autor",
            "nota",
        ]
    }
}

/* --- DB interface --- */

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

    /* --- Downloads --- */
    pub fn retrieve_toate_downloads(&self) -> Vec<Result<Download>> {
        let mut stmt_downloads = self.prepare_statement(
            "
            SELECT * FROM downloads
            "
        );

        stmt_downloads
            .query_map(NO_PARAMS, |row| Ok(Download::from_row(row)))
            .unwrap()
            .collect()
    }

    /* --- Articole --- */
    pub fn retrieve_toate_articole(&self) -> Vec<Result<Articol>> {
        let mut stmt_articole = self.prepare_statement(
            "
            SELECT * FROM articole
            "
        );

        stmt_articole
            .query_map(NO_PARAMS, |row| Ok(Articol::from_row(row)))
            .unwrap()
            .collect()
    }
}
