// update.rs: Maintains local acks DB updated [20220722-bar8tl]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::settings::params::ParameTp;
use crate::settings::SettingsTp;
use rusqlite::{Connection, Result};
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct rutaTp {
  pub remitente   : String,
  pub destinatario: String
}

#[derive(Debug, Deserialize)]
pub struct documentoTp {
  pub referenciaProveedor: String,
  pub serie              : String,
  pub folioFiscal        : String,
  pub UUID               : String
}

#[derive(Debug, Deserialize)]
pub struct recepcionTp {
  pub fechahora: String,
  pub estatus  : String
}

#[derive(Debug, Deserialize)]
pub struct acuseReciboTp {
  pub ruta     : rutaTp,
  pub documento: documentoTp,
  pub recepcion: recepcionTp,
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

#[derive(Debug, Default)]
pub struct UpdateTp {
  pub ticks: i64
}

impl UpdateTp {
  pub fn new_update() -> UpdateTp {
    let d = UpdateTp { ticks: 0 };
    d
  }

  pub fn update_tables(&mut self, parm: ParameTp, mut s: SettingsTp) {
    s.set_runvars(parm);
    self.ticks = 0;
    self.bwse_newacks(&s).expect("General error in report generation");
  }

  fn bwse_newacks(&mut self, s: &SettingsTp) -> Result<()> {
    let conn = Connection::open(&s.env.dbfpt)?;
    let lsack = self.get_last(&conn);
    let mut ilfil = lsack.clone();
    for entry in fs::read_dir(&s.env.xmldr).unwrap() {
      let entry = entry.unwrap().path();
      let filid = Path::new(&entry).file_name().unwrap();
      let filen = Path::new(&filid).file_stem().unwrap().to_str().unwrap();
      let extsn = Path::new(&filid).extension().unwrap().to_str().unwrap();
      let ifile: i64 = filen.to_string().trim().parse().unwrap();
      if extsn == s.env.xmltp && ifile > lsack {
        self.process_ack(&conn, &s, filid.to_str().unwrap(), ifile);
        if ifile > ilfil {
          ilfil = ifile;
        }
      }
    }
    conn.execute(
      "UPDATE last SET ackno=?1 WHERE recno='00';", [format!("{}", ilfil)]
    )?;
    self.note_corrections(&conn).expect("Update acks error");
    println!("Browsing FCA XML acknowledgments");
    Ok(())
  }

  fn get_last(&mut self, conn: &Connection) -> i64 {
    let mut lsack = 0i64;
    conn.query_row("SELECT ackno FROM last WHERE recno=\"00\"", [], |row| {
      Ok(lsack = row.get(0).unwrap())
    }).expect("error: no control record in table \"last\"");
    lsack
  }

  fn process_ack(&mut self, conn: &Connection, s: &SettingsTp, filid: &str,
    ifile: i64) {
    let xmlpt = format!("{}{}", s.env.xmldr, filid);
    println!("{}", xmlpt);
    let f = File::open(&xmlpt).expect(&format!("Cannot open file {}", xmlpt));
    let ack: acuseReciboTp = serde_xml_rs::de::from_reader(f).unwrap();
    self.isrt_acks(&conn, ifile, &ack).expect("Insert acks error");
    self.TICK();
  }

  fn isrt_acks(&mut self, conn: &Connection, ifile: i64,
    ack: &acuseReciboTp) -> Result<()> {
    let mut err1 = String::from("");
    let mut err2 = String::from("");
    let notes    = String::from("");
    if ack.error.len() >= 1 {
      err1 = ack.error[0].clone();
    }
    if ack.error.len() >= 2 {
      err2 = ack.error[1].clone();
    }
    conn.execute(
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
    )?;
    Ok(())
  }

  fn note_corrections(&mut self, conn: &Connection) -> Result<()> {
    let mut stmt1 = conn.prepare(
      "SELECT ackno,serie,folio FROM acks WHERE stats<>\"00\" ORDER BY ackno")?;
    let failedack_iter = stmt1.query_map([], |row| { Ok(AckkTp {
        ackno: row.get(0)?,
        serie: row.get(1)?,
        folio: row.get(2)?
      })
    })?;
    for failedack in failedack_iter {
      let flack = failedack.unwrap().clone();
      let mut stmt2 = conn.prepare(
        "SELECT ackno,serie,folio FROM acks WHERE stats=\"00\" AND serie=?1 AND
          folio=?2 AND ackno<>?3")?;
      let matchack_iter = stmt2.query_map([flack.serie, flack.folio,
        flack.ackno.to_string()], |row| { Ok(AcknTp {
          ackno: row.get(0)?
        })
      })?;
      for matchack in matchack_iter {
        let mtack = matchack.unwrap().clone();
        let notes = format!("Corrected with ack# {}", mtack.ackno);
        conn.execute("UPDATE acks SET notes=?1 WHERE ackno=?2",
          [notes, flack.ackno.to_string()])?;
      }
    }
    Ok(())
  }

  fn TICK(&mut self) {
    self.ticks += 1;
    if self.ticks % 10 == 0 {
      print!(".");
    }
  }
}
