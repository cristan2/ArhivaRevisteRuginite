use rusqlite::Result;
use arhiva_reviste_ruginite::db::*;

fn main() -> Result<()> {

    let db = DBConnection::open("arhiva_reviste_v7.0.db");

    let reviste = db.retrieve_toate_revistele();

    let rev_id = 1;
    let revista: Result<Revista> = db.retrieve_revista(&rev_id);

    for rev in reviste {
        println!("{:?}", rev.unwrap());
    }

    println!("{:?}", revista);

    let editii = db.retrieve_toate_editiile();


    for editie in editii {
        println!("{:?}", editie.unwrap());
    }

    Ok(())
}