// defaults.rs: Reads defaults file and gets default settings
// [20220722-bar8tl]
use serde::Deserialize;
use serde_json;
use std::fs::File;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DefltTp {
  pub acks_dir     : String,
  pub acks_type    : String,
  pub xml_dir      : String,
  pub xml_type     : String,
  pub db_dir       : String,
  pub db_name      : String,
  pub report_filter: String,
  pub filter_parm1 : String,
  pub filter_parm2 : String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DefltsTp {
  pub deflt: DefltTp,
}

impl DefltsTp {
  pub fn new_deflts() -> DefltsTp {
    let dfl = DefltsTp { ..Default::default() };
      dfl
  }

  pub fn get_deflts(&mut self, fname: &str) {
    let f = File::open(fname).unwrap();
    let dfl: DefltsTp = serde_json::from_reader(f)
      .expect("JSON not well-formed");
    self.deflt = dfl.deflt;
  }
}
