// updt_tables.rs - Maintains local acks db updated with new acks received
// (2019-03-01)
#![allow(non_snake_case)]

use crate::update_db::get_lastack::get_lastack;
use crate::update_db::process_ack::process_ack;
use crate::update_db::note_corrections::note_corrections;
use rusqlite::Connection;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct RutaTp {
  pub remitente   : String,
  pub destinatario: String
}

#[derive(Debug, Deserialize)]
pub struct DocumentoTp {
  pub referenciaProveedor: String,
  pub serie              : String,
  pub folioFiscal        : String,
  pub UUID               : String
}

#[derive(Debug, Deserialize)]
pub struct RecepcionTp {
  pub fechahora: String,
  pub estatus  : String
}

#[derive(Debug, Deserialize)]
pub struct AcuseReciboTp {
  pub ruta     : RutaTp,
  pub documento: DocumentoTp,
  pub recepcion: RecepcionTp,
  #[serde(default)]
  pub error    : Vec<String>
}

pub fn updt_tables(dbopt: String, xmldr: String, xmltp: String) {
  let mut ticks = 0i64;
  let cnn = Connection::open(&dbopt).unwrap();
  let lsack = get_lastack(&cnn);
  let mut ilfil = lsack.clone();
  print!("Browsing FCA XML acknowledgments");
  for entry in fs::read_dir(&xmldr).unwrap() {
    let entry = entry.unwrap().path();
    let filid = Path::new(&entry).file_name().unwrap();
    let filen = Path::new(&filid).file_stem().unwrap().to_str().unwrap();
    let extsn = Path::new(&filid).extension().unwrap().to_str().unwrap();
    let ifile: i64 = filen.to_string().trim().parse().unwrap();
    if extsn == xmltp && ifile > lsack {
      process_ack(&cnn, &xmldr, filid.to_str().unwrap(), ifile, &mut ticks);
      if ifile > ilfil {
        ilfil = ifile;
      }
    }
  }
  cnn.execute("UPDATE last SET ackno=?1 WHERE recno='00';", [format!("{}", ilfil)])
    .expect("Error updating table LAST.");
  note_corrections(&cnn);
  println!("\nProcess completed.");
}
