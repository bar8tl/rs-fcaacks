// set_pgm_settings.rs - Program level settings definition (2019-03-01 bar8tl)
use crate::settings::read_config_file::{ConfigTp, read_config_file};
use chrono::Local;
use chrono::{NaiveDate, NaiveDateTime};
use rblib::read_cmdline_args::{ParamsTp, read_cmdline_args};
use serde_json::from_str;

const DEFAULTS: &str = include!("_defaults.json");

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub cfg  : ConfigTp,
  pub dfl  : ConfigTp,
  pub ackdr: String,
  pub acktp: String,
  pub xmldr: String,
  pub xmltp: String,
  pub dbodr: String,
  pub dbonm: String,
  pub dbopt: String,
  pub outdr: String,
  pub outfl: String,
  pub outpt: String,
  pub filtr: String,
  pub fprm1: String,
  pub fprm2: String,
  pub objnm: String,
  pub found: i8,
  pub dtsys: NaiveDateTime,
  pub dtcur: NaiveDateTime,
  pub dtnul: NaiveDateTime,
  pub fdate: NaiveDateTime,
  pub tdate: NaiveDateTime
}

pub fn set_pgm_settings(fname: &str) -> SettingsTp {
  let mut s = SettingsTp { ..Default::default() };
  s.prm = read_cmdline_args();
  s.dfl = from_str(DEFAULTS).unwrap();
  s.cfg = read_config_file(fname);
  let c = &s.cfg;
  s.ackdr = if c.progm.ackdr.len() > 0
    { c.progm.ackdr.clone() } else { s.dfl.progm.ackdr.clone() };
  s.acktp = if c.progm.acktp.len() > 0
    { c.progm.acktp.clone() } else { s.dfl.progm.acktp.clone() };
  s.xmldr = if c.progm.xmldr.len() > 0
    { c.progm.xmldr.clone() } else { s.dfl.progm.xmldr.clone() };
  s.xmltp = if c.progm.xmltp.len() > 0
    { c.progm.xmltp.clone() } else { s.dfl.progm.xmltp.clone() };
  s.dbodr = if c.progm.dbodr.len() > 0
    { c.progm.dbodr.clone() } else { s.dfl.progm.dbodr.clone() };
  s.dbonm = if c.progm.dbonm.len() > 0
    { c.progm.dbonm.clone() } else { s.dfl.progm.dbonm.clone() };
  s.outdr = if c.progm.outdr.len() > 0
    { c.progm.outdr.clone() } else { s.dfl.progm.outdr.clone() };
  s.outfl = if c.progm.outfl.len() > 0
    { c.progm.outfl.clone() } else { s.dfl.progm.outfl.clone() };
  s.dbopt = format!("{}{}", s.dbodr, s.dbonm);
  s.outpt = format!("{}{}", s.outdr, s.outfl);
  s.dtsys = Local::now().naive_local();
  s.dtcur = Local::now().naive_local();
  s.dtnul = NaiveDateTime::MIN;
  s.fdate = NaiveDate::from_ymd_opt(1901, 1, 1).unwrap()
   .and_hms_opt(0, 0, 0).unwrap();
  s.tdate = Local::now().naive_local();
  return s;
}
