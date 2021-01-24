// use rusqlite::Result;
use arhiva_reviste_ruginite::db::*;
use arhiva_reviste_ruginite::printer::{PrintableRow, make_table};
use rusqlite::Result;

fn main() /*-> Result<()>*/ {

    let db = DBConnection::open("arhiva_reviste_v6.db");

    afisare_toate_revistele(&db);
    // afisare_toate_editiile(&db);
    // afisare_toate_downloads(&db);
    // afisare_toate_articole(&db);

    // citire si afisare revista dupa id
    // let rev_id = 1;
    // let revista: Result<Revista> = db.retrieve_revista(&rev_id);
    //
    // println!("{:?}", revista);

    // let downloads = db.retrieve_toate_articole();
    // for dld in downloads {
    //     println!("{:?}", dld.unwrap());
    // }

    /*Ok(())*/
}

fn afisare_toate_revistele(db: &DBConnection) {
    let reviste = db.retrieve_toate_revistele();
    print_table(&reviste);
}

fn afisare_toate_editiile(db: &DBConnection) {
    let editii = db.retrieve_toate_editiile();
    print_table(&editii);
}

fn afisare_toate_downloads(db: &DBConnection) {
    let dlds = db.retrieve_toate_downloads();
    print_table(&dlds);
}

fn afisare_toate_articole(db: &DBConnection) {
    let articole = db.retrieve_toate_articole();
    print_table(&articole);
}

fn print_table<T>(rows: &Vec<Result<T>>) where T: PrintableRow {
    let tabel = make_table(rows);
    tabel.printstd();
    println!("Printed {:?} rows", rows.len());
}