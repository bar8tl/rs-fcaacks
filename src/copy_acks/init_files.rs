// init_files.rs - Inital copy of ack files into a local folder (2019-03-01 bar8tl)
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn init_files(ackdr: String, xmldr: String, acktp: String, xmltp: String) {
  let ackar = format!("{}{}", ackdr, "archive\\");
  copy_files(&ackar, &xmldr, &acktp, &xmltp);
  copy_files(&ackdr, &xmldr, &acktp, &xmltp);
}

fn copy_files(ffold: &String, tfold: &String, fextn: &String, textn: &String) {
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
      let parm1  = format!("{}{}", ffold, filid.to_str().unwrap());
      let parm2  = format!("{}{}.{}", tfold, filnm, textn);
      let output = Command::new("cmd").args(&["/C", "copy",
        parm1.as_str(), parm2.as_str()]).output()
        .expect("Failed to execute system command");
      println!("{}", String::from_utf8(output.stdout).unwrap());
    }
  }
}
