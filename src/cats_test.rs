use std::collections::HashMap;
use rusqlite::{Connection, Result, NO_PARAMS, Statement};

#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

pub fn cats_test() {
    let conn_cats = Connection::open("../cats.db").unwrap();

    create_cats_db(&conn_cats);
    insert_cats_data(&conn_cats);
    let cats = read_cats_data(&conn_cats);
    println!("{:?}", &cats);

    for cat in cats {
        println!("Found cat {:?}", cat);
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
