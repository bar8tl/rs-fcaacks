// rfsh_files.rs - Keeps acks local folder updated with all received acks
// (2019-03-01 bar8tl)
#![allow(unused)]

use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn rfsh_files(dbopt: String, ackdr: String, xmldr: String, acktp: String,
  xmltp: String) {
  let cnn = Connection::open(dbopt).unwrap();
  for entry in fs::read_dir(&ackdr).unwrap() {
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
    if extsn.to_str().unwrap().to_lowercase() == acktp.to_lowercase() {
      match cnn.query_row("SELECT ackno FROM acks WHERE ackno=?1",
        [ifile], |row| { Ok(()) }) {
        Ok(rows) => {}
        Err(query_returned_no_rows) => {
          println!("> copying file {}", filid.to_str().unwrap());
          let parm1  = format!("{}{}", &ackdr, filid.to_str().unwrap());
          let parm2  = format!("{}{}.{}", xmldr, filnm, xmltp);
          let output = Command::new("cmd").args(&["/C", "copy",
            parm1.as_str(), parm2.as_str()]).output()
            .expect("Failed to execute system command");
          println!("{}", String::from_utf8(output.stdout).unwrap());
        }
      }
    }
  }
}
