// inicopy.rs: Inital copy of FCA ack files into a local folder
// [20220722-bar8tl]
use crate::settings::params::ParameTp;
use crate::settings::SettingsTp;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Default)]
pub struct InicopyTp {}

impl InicopyTp {
  pub fn new_inicopy() -> InicopyTp {
    let d = InicopyTp {};
    d
  }

  pub fn initial_copy(&mut self, parm: ParameTp, mut s: SettingsTp) {
    s.set_runvars(parm);
    self.copy_files(format!("{}{}", s.env.ackdr, "archive\\"),
      &s.env.xmldr, &s.env.acktp, &s.env.xmltp);
    self.copy_files(s.env.ackdr,
      &s.env.xmldr, &s.env.acktp, &s.env.xmltp);
  }

  fn copy_files(&mut self, ffold: String, tfold: &String, fextn: &String,
    textn: &String) {
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
        println!("> copying file {}", filid.to_str().unwrap());
        let args = format!("{}{} {}{}.{}", ffold, filid.to_str().unwrap(),
          tfold, filnm, textn);
        Command::new("cmd").args(["/C", "copy ", &args]).output()
          .expect("Failed to execute system command");
      }
    }
  }
}
