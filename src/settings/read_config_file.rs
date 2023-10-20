// read_config_file.rs - Upload external configuration json file with user defined
// settings (2019-03-01 bar8tl)
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProgmTp {
  #[serde(default)]
  pub ackdr: String,
  #[serde(default)]
  pub acktp: String,
  #[serde(default)]
  pub xmldr: String,
  #[serde(default)]
  pub xmltp: String,
  #[serde(default)]
  pub dbodr: String,
  #[serde(default)]
  pub dbonm: String,
  #[serde(default)]
  pub outdr: String,
  #[serde(default)]
  pub outfl: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RunTp {
  #[serde(default)]
  pub optcd: String,
  #[serde(default)]
  pub outdr: String,
  #[serde(default)]
  pub outfl: String,
  #[serde(default)]
  pub filtr: String,
  #[serde(default)]
  pub fprm1: String,
  #[serde(default)]
  pub fprm2: String
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigTp {
  #[serde(default)]
  pub progm: ProgmTp,
  #[serde(default)]
  pub run  : Vec<RunTp>
}

pub fn read_config_file(fname: &str) -> ConfigTp {
  let mut cfg: ConfigTp = Default::default();
  match File::open(fname) {
    Ok(f)  => { cfg = from_reader(f).expect("JSON not well-formed"); },
    Err(_) => {},
  };
  return cfg;
}
