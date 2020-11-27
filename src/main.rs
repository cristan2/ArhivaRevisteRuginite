use rusqlite::Result;
use arhiva_reviste_ruginite::db::*;

use prettytable::{Table, Row, Cell, Attr, color};
use arhiva_reviste_ruginite::AsciiPrintable;

fn main() -> Result<()> {

    let db = DBConnection::open("arhiva_reviste_v7.0.db");

    afisare_toate_revistele(&db);
    afisare_toate_editiile(&db);

    // citire si afisare revista dupa id
    // let rev_id = 1;
    // let revista: Result<Revista> = db.retrieve_revista(&rev_id);
    //
    // println!("{:?}", revista);

    Ok(())
}

fn afisare_toate_revistele(db: &DBConnection) {
    // citire si afisare toate revistele
    let reviste = db.retrieve_toate_revistele();

    // for rev in reviste {
    //     println!("{:?}", rev.unwrap());
    // }

    /* Printare rezultat in tabel */
    let mut table = Table::new();

    // adauga header tabel
    let header_cells = Revista::to_printable_header().into_iter()
        .map(|s|
            Cell::new(&s)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)))
        .collect();

    table.add_row(Row::new(header_cells));

    // adauga randuri tabel
    for rev in reviste {
        let fields_vec = rev.unwrap().to_printable_vec();
        let cells = fields_vec.into_iter().map(|s| Cell::new(&s)).collect();
        table.add_row(Row::new(cells));
    }

    // afisare tabel
    table.printstd();
}

fn afisare_toate_editiile(db: &DBConnection) {
    // citire si afisare toate editiile
    let editii = db.retrieve_toate_editiile();

    // for editie in editii {
    //     println!("{:?}", editie.unwrap());
    // }

    let mut table = Table::new();

    // adauga header tabel
    let header_cells = Editie::to_printable_header().into_iter()
        .map(|s|
            Cell::new(&s)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)))
            .collect();

    table.add_row(Row::new(header_cells));

    for editie in editii {
        let fields_vec = editie.unwrap().to_printable_vec();
        let cells = fields_vec.into_iter().map(|s| Cell::new(&s)).collect();
        table.add_row(Row::new(cells));
    }

    // afisare tabel
    table.printstd();
}