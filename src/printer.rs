use crate::db::{Revista, Editie, Downloads};
use prettytable::{Table, Cell, Attr, Row, color};
use rusqlite::Result;

pub trait PrintableRow {
    fn get_printable_header() -> Vec<String>;
    fn get_printable_row(&self) -> Vec<String>;
}

fn empty_string() -> String { String::from("")}

impl PrintableRow for Revista {
    fn get_printable_header() -> Vec<String> {
        Self::get_nume_coloana().into_iter().map(|s| String::from(s)).collect()
    }

    fn get_printable_row(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.nume.clone(),
            self.alias.clone().unwrap_or(empty_string()),
            self.status.clone().unwrap_or(empty_string()),
            self.tip.clone().unwrap_or(empty_string()),
            self.perioada.clone().unwrap_or(empty_string()),
            self.aparitii.clone().unwrap_or(empty_string()),
            self.descriere.clone().unwrap_or(empty_string()),
            self.link.clone().unwrap_or(empty_string()),
            self.observatii.clone().unwrap_or(empty_string()),
        ]
    }
}

impl PrintableRow for Editie {

    fn get_printable_header() -> Vec<String> {
        Self::get_nume_coloana().into_iter().map(|s| String::from(s)).collect()
    }

    fn get_printable_row(&self) -> Vec<String> {
        let i32_to_string = |n: i32| n.to_string();
        vec![
            self.editie_id.clone().to_string(),
            self.revista_id.clone().to_string(),
            self.tip.clone().unwrap_or(empty_string()),
            // self.parinte_editie_id.clone().unwrap_or(empty_string()),
            self.numar.clone().map_or(empty_string(), i32_to_string),
            self.an.clone().to_string(),
            self.luna.clone().map_or(empty_string(), i32_to_string),
            self.luna_sfarsit.clone().unwrap_or(empty_string()),
            self.pret.clone().map_or(empty_string(), i32_to_string),
            self.nr_pagini.clone().map_or(empty_string(), i32_to_string),
            self.disc_demo.clone().unwrap_or(empty_string()),
            self.joc_complet.clone().unwrap_or(empty_string()),
            self.redactor_sef.clone().unwrap_or(empty_string()),
            // self.editie_link_oficial.clone().unwrap_or(empty_string()),
            // TODO truncate e solutie temporara pentru text lung, de gasit alta solutie sau folosit comfy-table
            self.editie_observatii.clone().map_or(empty_string(), |mut s| {
                s.truncate(52);
                s
            }),
            // self.scan_info_nr_pg.clone().map_or(empty_string(), |nr| nr.to_string()),
            // self.scan_info_pg_lipsa.clone().unwrap_or(empty_string()),
            // self.scan_info_observatii.clone().unwrap_or(empty_string()),
            self.scan_info_credits.clone().unwrap_or(empty_string()),
        ]
    }
}

impl PrintableRow for Downloads {
    fn get_printable_header() -> Vec<String> {
        Self::get_nume_coloana().into_iter().map(|s| String::from(s)).collect()
    }

    fn get_printable_row(&self) -> Vec<String> {
        let i32_to_string = |n: i32| n.to_string();
        vec![
            self.editie_id.to_string(),
            self.categorie.clone(),
            self.item.clone().map_or(empty_string(), i32_to_string),
            self.link.clone().unwrap_or(empty_string()),
        ]
    }
}

pub fn make_table<T>(printable_rows: &Vec<Result<T>>) -> Table
    where T: PrintableRow {

    let mut table = Table::new();

    // adauga header tabel
    let header_cells = T::get_printable_header().into_iter()
        .map(|s|
            Cell::new(&s)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)))
        .collect();

    table.add_row(Row::new(header_cells));

    // adauga randuri tabel
    for row in printable_rows {
        let fields_vec = row.as_ref().unwrap().get_printable_row();
        let cells = fields_vec.into_iter().map(|s| Cell::new(&s)).collect();
        table.add_row(Row::new(cells));
    }
    table
}
