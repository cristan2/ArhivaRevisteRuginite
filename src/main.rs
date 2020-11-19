use rusqlite::Result;
use arhiva_reviste_ruginite::db::*;

// DOC & EXEMPLE
// https://github.com/rusqlite/rusqlite
// https://crates.io/crates/rusqlite/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#create-a-sqlite-database

// https://github.com/Progressbar/heimdall-db
// https://qiita.com/kimagure/items/e24d7d6514a6a0dd2b48

fn main() -> Result<()> {

    let conn = DBConnection::open("arhiva_reviste_v7.0.db");

    let reviste = conn.query_reviste();

    for revista in reviste {
        println!("{:?}", revista.unwrap());
    }

    Ok(())
}