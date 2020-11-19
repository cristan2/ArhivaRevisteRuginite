use rusqlite::Result;
use arhiva_reviste_ruginite::db::*;

fn main() -> Result<()> {

    let conn = DBConnection::open("arhiva_reviste_v7.0.db");

    let reviste = conn.retrieve_toate_revistele();

    let rev_id = 1;
    let revista: Result<Revista> = conn.retrieve_revista(&rev_id);

    for revista in reviste {
        println!("{:?}", revista.unwrap());
    }

    println!("{:?}", revista);


    Ok(())
}