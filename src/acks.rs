//**********************************************************************************
// acks.rs: Inital/Periodic copy of Chrysler ack files into a local folder
// (2022-04-06 bar8tl)
//**********************************************************************************
#![allow(unused)]

use crate::settings::SettingsTp;
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::process::Command;

// inicopy - Inital copy of FCA ack files into a local folder ----------------------
pub fn init_files(s: SettingsTp) {
  copy_files(format!("{}{}", s.ackdr, "archive\\"), &s.xmldr, &s.acktp, &s.xmltp);
  copy_files(s.ackdr, &s.xmldr, &s.acktp, &s.xmltp);
}

fn copy_files(ffold: String, tfold: &String, fextn: &String, textn: &String) {
  for entry in fs::read_dir(&ffold).unwrap() {
    let entry = entry.unwrap().path();
    if entry.is_dir() {
      continue;
    }
    let filid = Path::new(&entry).file_name().unwrap();
    let filen = Path::new(&filid).file_stem().unwrap();
    let extsn = Path::new(&filid).extension().unwrap();
    let tk: Vec<&str> = filen.to_str().unwrap().split('_').collect();
    let filnm = tk[2];
    let _ifile: i32 = filnm.trim().parse().unwrap();
    if extsn.to_str().unwrap().to_lowercase() == fextn.to_lowercase() {
      println!("> copying file: {}", filid.to_str().unwrap());
      let parm1 = format!("{}{}", ffold, filid.to_str().unwrap());
      let parm2 = format!("{}{}.{}", tfold, filnm, textn);
      let output = Command::new("cmd").args(&["/C", "copy",
        parm1.as_str(), parm2.as_str()]).output()
        .expect("Failed to execute system command");
      println!("{}", String::from_utf8(output.stdout).unwrap());
    }
  }
}

// refresh - Keeps FCA-Acks local folder updated with all received FCA acks --------
pub fn rfsh_files(s: SettingsTp) {
  let cnn = Connection::open(&s.dbopt).unwrap();
  for entry in fs::read_dir(&s.ackdr).unwrap() {
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
    if extsn.to_str().unwrap().to_lowercase() == s.acktp.to_lowercase() {
      match cnn.query_row("SELECT ackno FROM acks WHERE ackno=?1",
        [ifile], |row| { Ok(()) }) {
        Ok(rows) => {}
        Err(query_returned_no_rows) => {
          println!("> copying file {}", filid.to_str().unwrap());
          let parm1 = format!("{}{}", s.ackdr, filid.to_str().unwrap());
          let parm2 = format!("{}{}.{}", s.xmldr, filnm, s.xmltp);
          let output = Command::new("cmd").args(&["/C", "copy",
            parm1.as_str(), parm2.as_str()]).output()
            .expect("Failed to execute system command");
          println!("{}", String::from_utf8(output.stdout).unwrap());
        }
      }
    }
  }
}
