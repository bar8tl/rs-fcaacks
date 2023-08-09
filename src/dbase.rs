//**********************************************************************************
// dbase.rs: Acks DB tables creation and maintenence (2022-04-06 bar8tl)
//**********************************************************************************
#![allow(non_snake_case)]

use crate::settings::SettingsTp;
use rblib::db::{TlistTp, reset_tables};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::fs::File;
use std::path::Path;

// Set/Reset DB tables -------------------------------------------------------------
const ITABLES: &str = r#"{
  "sqlst": [
    {"activ": true, "table": "acks", "sqlst": "CREATE TABLE IF NOT EXISTS acks (ackno INTEGER PRIMARY KEY, issue TEXT, rceiv TEXT, invoi TEXT, serie TEXT, folio TEXT, uuid TEXT, dtime TEXT, stats TEXT, errn1 TEXT, errn2 TEXT, notes TEXT);"},
    {"activ": true, "table": "last", "sqlst": "CREATE TABLE IF NOT EXISTS last (recno TEXT PRIMARY KEY, ackno INTEGER);"}
  ]
}"#;

#[derive(Debug, Clone, Default, Deserialize)]
struct SqlstTp {
  activ: bool,
  table: String,
  sqlst: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ItablesTp {
  sqlst: Vec<SqlstTp>
}

pub fn crea_tables(s: SettingsTp) {
  let it: ItablesTp = serde_json::from_str(ITABLES).unwrap();
  let mut tlist: Vec<TlistTp> = Vec::with_capacity(it.sqlst.len());
  for sql in &it.sqlst {
    if sql.activ {
      tlist.push(TlistTp {table: sql.table.clone(), sqlst: sql.sqlst.clone()});
    }
  }
  reset_tables(&s.dbopt, &tlist);
}

// Maintains local acks DB updated with new Acks received --------------------------
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

#[derive(Debug, Clone, Default)]
pub struct AcknTp {
  pub ackno: i64
}

#[derive(Debug, Clone, Default)]
pub struct AckkTp {
  pub ackno: i64,
  pub serie: String,
  pub folio: String
}

pub fn updt_tables(s: SettingsTp) {
  let mut ticks = 0i64;
  let cnn = Connection::open(&s.dbopt).unwrap();
  let lsack = get_lastack(&cnn);
  let mut ilfil = lsack.clone();
  print!("Browsing FCA XML acknowledgments");
  for entry in fs::read_dir(&s.xmldr).unwrap() {
    let entry = entry.unwrap().path();
    let filid = Path::new(&entry).file_name().unwrap();
    let filen = Path::new(&filid).file_stem().unwrap().to_str().unwrap();
    let extsn = Path::new(&filid).extension().unwrap().to_str().unwrap();
    let ifile: i64 = filen.to_string().trim().parse().unwrap();
    if extsn == s.xmltp && ifile > lsack {
      process_ack(&cnn, &s, filid.to_str().unwrap(), ifile, &mut ticks);
      if ifile > ilfil {
        ilfil = ifile;
      }
    }
  }
  cnn.execute("UPDATE last SET ackno=?1 WHERE recno='00';", [format!("{}", ilfil)])
    .expect("Error updating table LAST.");
  note_corrections(&cnn);
  println!("\nProcess is completed.");
}

fn get_lastack(cnn: &Connection) -> i64 {
  let mut lsack = 0i64;
  cnn.query_row("SELECT ackno FROM last WHERE recno=\"00\"", [], |row| {
    Ok(lsack = row.get(0).unwrap())})
    .expect("Error: Control record in table LAST not found.");
  return lsack;
}

fn process_ack(cnn: &Connection, s: &SettingsTp, filid: &str, ifile: i64,
  ticks: &mut i64) {
  let xmlpt = format!("{}{}", s.xmldr, filid);
  let f = File::open(&xmlpt).expect(&format!("Cannot open file {}", xmlpt));
  let ack: AcuseReciboTp = serde_xml_rs::de::from_reader(f).unwrap();
  isrt_acks(&cnn, ifile, &ack);
  TICK(ticks);
}

fn isrt_acks(cnn: &Connection, ifile: i64, ack: &AcuseReciboTp) {
  let mut err1 = String::new();
  let mut err2 = String::new();
  let notes    = String::new();
  if ack.error.len() >= 1 {
    err1 = ack.error[0].clone();
  }
  if ack.error.len() >= 2 {
    err2 = ack.error[1].clone();
  }
  cnn.execute(
    "INSERT INTO acks VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)", (
      ifile,
      &ack.ruta.remitente,
      &ack.ruta.destinatario,
      &ack.documento.referenciaProveedor,
      &ack.documento.serie,
      &ack.documento.folioFiscal,
      &ack.documento.UUID,
      &ack.recepcion.fechahora,
      &ack.recepcion.estatus,
      err1,
      err2,
      notes
    )
  ).expect("Error insering a new ACKS record.");
}

fn note_corrections(cnn: &Connection) {
  let mut stmt1 = cnn.prepare(
    "SELECT ackno, serie, folio FROM acks WHERE stats<>\"00\" ORDER BY ackno")
    .unwrap();
  let failedack_iter = stmt1.query_map([], |row| { Ok(AckkTp {
      ackno: row.get(0).unwrap(),
      serie: row.get(1).unwrap(),
      folio: row.get(2).unwrap()
    })
  }).unwrap();
  for failedack in failedack_iter {
    let flack = failedack.unwrap().clone();
    let mut stmt2 = cnn.prepare(
      "SELECT ackno, serie, folio FROM acks WHERE stats=\"00\" AND serie=?1 AND
        folio=?2 AND ackno<>?3").unwrap();
    let matchack_iter = stmt2.query_map([flack.serie, flack.folio,
      flack.ackno.to_string()], |row| { Ok(AcknTp {
        ackno: row.get(0)?
      })
    }).unwrap();
    for matchack in matchack_iter {
      let mtack = matchack.unwrap().clone();
      let notes = format!("Corrected with ack# {}", mtack.ackno);
      cnn.execute("UPDATE acks SET notes=?1 WHERE ackno=?2",
        [notes, flack.ackno.to_string()]).expect("Error: Correction note failed.");
    }
  }
}

fn TICK(ticks: &mut i64) {
  *ticks = *ticks + 1;
  if *ticks % 10i64 == 0 {
    print!(".");
  }
}
