// refresh.rs: Keeps FCA-Acks local folder updated with all received FCA acks
// [20220722-bar8tl]
#![allow(unused)]

use crate::settings::params::ParameTp;
use crate::settings::SettingsTp;
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default)]
pub struct RefreshTp {}

impl RefreshTp {
  pub fn new_refresh() -> RefreshTp {
    let d = RefreshTp {};
    d
  }

  pub fn refresh_files(&mut self, parm: ParameTp, mut s: SettingsTp) {
    s.set_runvars(parm);
    self.bwse_dir(&s).expect("xml-files browsing error");
  }

  fn bwse_dir(&mut self, s: &SettingsTp) -> Result<()> {
    let conn = Connection::open(&s.env.dbfpt)?;
    for entry in fs::read_dir(&s.env.ackdr).unwrap() {
      let entry = entry.unwrap().path();
      if entry.is_dir() {
        continue;
      }
      let filid = Path::new(&entry).file_name().unwrap();
      let filen = Path::new(&filid).file_stem().unwrap();
      let extsn = Path::new(&filid).extension().unwrap();
      let tk: Vec<&str> = filen.to_str().unwrap().split('_').collect();
      let filnm = tk[2];
      let ifile: i32 = filnm.trim().parse().unwrap();
      if extsn.to_str().unwrap().to_lowercase() == s.env.acktp.to_lowercase() {
        match conn.query_row("SELECT ackno FROM acks WHERE ackno=?1",
          [ifile], |row| { Ok(()) }) {
          Ok(rows) => {}
          Err(query_returned_no_rows) => {
            println!("> copying file {}", filid.to_str().unwrap());
            let args = format!("{}{} {}{}.{}", s.env.ackdr,
              filid.to_str().unwrap(), s.env.xmldr, filnm, s.env.xmltp);
            Command::new("cmd").args(["/C", "copy ", &args]).output()
              .expect("Failed to execute system command");
          }
        }
      }
    }
    Ok(())
  }
}
