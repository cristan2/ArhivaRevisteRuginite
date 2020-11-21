use crate::db::*;

pub mod db;
pub mod cats_test;

pub trait AsciiPrintable {
    fn to_printable_vec(&self) -> Vec<String>;
}

impl AsciiPrintable for Revista {
    fn to_printable_vec(&self) -> Vec<String> {
        unimplemented!()
    }
}

impl AsciiPrintable for Editie {
    fn to_printable_vec(&self) -> Vec<String> {
        unimplemented!()
    }
}