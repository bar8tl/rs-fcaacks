// config.rs: Reads config file and gets run parameters [20220722-bar8tl]
use serde::Deserialize;
use serde_json;
use std::fs::File;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProgramTp {
  pub ackdr: String,
  pub acktp: String,
  pub xmldr: String,
  pub xmltp: String,
  pub dbfdr: String,
  pub dbfnm: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RunTp {
  pub optcd: String,
  pub filtr: String,
  pub fprm1: String,
  pub fprm2: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigTp {
  pub progm: ProgramTp,
  pub run  : Vec<RunTp>
}

impl ConfigTp {
  pub fn new_config() -> ConfigTp {
    let cfg = ConfigTp{ ..Default::default() };
    cfg
  }

  pub fn get_config(&mut self, fname: &str) {
    let f = File::open(fname).unwrap();
    let cfg: ConfigTp = serde_json::from_reader(f)
      .expect("JSON not well-formed");
    self.progm = cfg.progm;
    self.run   = cfg.run;
  }
}
