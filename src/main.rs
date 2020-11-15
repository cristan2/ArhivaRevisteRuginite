use rusqlite::{Connection, Result, Statement, MappedRows};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;

// DOC & EXEMPLE
// https://github.com/rusqlite/rusqlite
// https://crates.io/crates/rusqlite/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#create-a-sqlite-database

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

#[derive(Debug)]
struct Revista {
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
    fn from_row(row: &rusqlite::Row) -> Revista {
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

fn create_cats_db(conn_cats: &Connection) -> Result<usize> {
    conn_cats.execute(
        "create table if not exists cat_colors (
            id integer primary key,
            name text not null unique
        )",
        NO_PARAMS,
    )?;
    conn_cats.execute(
        "create table if not exists cats (
            id integer primary key,
            name text not null,
            color_id integer not null references cat_colors(id)
        )",
        NO_PARAMS,
    )
}

fn insert_cats_data(conn_cats: &Connection) {

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn_cats.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        );

        let last_id: String = conn_cats.last_insert_rowid().to_string();

        for cat in catnames {
            conn_cats.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            );
        };
    };
}

fn read_cats_data(conn_cats: &Connection) -> Vec<Result<Cat>> {
    let mut stmt: Statement = conn_cats.prepare(
        "SELECT c.name, cc.name from cats as c
         INNER JOIN cat_colors as cc
         ON cc.id = c.color_id;",
    ).unwrap();

    stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })
        .unwrap()
        // https://stackoverflow.com/questions/42594102/return-type-for-rusqlite-mappedrows
        .collect()
}

fn main() -> Result<()> {
    let conn_cats = Connection::open("cats.db")?;

    create_cats_db(&conn_cats);
    insert_cats_data(&conn_cats);
    let cats = read_cats_data(&conn_cats);
    println!("{:?}", &cats);

    // for cat in cats {
    //     println!("Found cat {:?}", cat);
    // }

    // let conn = Connection::open("arhiva_reviste_v7.0.db")?;
    //
    // let mut stmt_revista = conn.prepare("SELECT * FROM reviste").unwrap();
    //
    // let reviste = stmt_revista.query_map(NO_PARAMS, |row| {Ok(Revista::from_row(row))})?;
    //
    // for revista in reviste {
    //     println!("{:?}", revista.unwrap());
    // }

    Ok(())
}