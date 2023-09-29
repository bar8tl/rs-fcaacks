//**********************************************************************************
// settings.rs: Declare Pgm-level & Run-level settings (2022-04-06 bar8tl)
//**********************************************************************************
use crate::settings::config::ConfigTp;
use chrono::Local;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use rblib::params::{ParamsTp, ParameTp};

const DEFAULTS: &str = include!("defaults.json");

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub cfd  : ConfigTp,
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
  pub found: bool,
  pub dtsys: NaiveDateTime,
  pub dtcur: NaiveDateTime,
  pub dtnul: NaiveDateTime,
  pub fdate: NaiveDateTime,
  pub tdate: NaiveDateTime
}

impl SettingsTp {
  pub fn new_settings() -> SettingsTp {
    let mut stg = SettingsTp { ..Default::default() };
    stg.dfl = serde_json::from_str(DEFAULTS).unwrap();
    stg.prm = ParamsTp::new_params();
    stg.cfd = ConfigTp::new_config();
    stg.set_settings("_config.json");
    stg
  }

  pub fn set_settings(&mut self, cfnam: &str) {
    self.prm.scan_params();
    self.cfd.get_config(cfnam);
    let c = &self.cfd;
    self.ackdr = if c.progm.ackdr.len() > 0
      { c.progm.ackdr.clone() } else { self.dfl.progm.ackdr.clone() };
    self.acktp = if c.progm.acktp.len() > 0
      { c.progm.acktp.clone() } else { self.dfl.progm.acktp.clone() };
    self.xmldr = if c.progm.xmldr.len() > 0
      { c.progm.xmldr.clone() } else { self.dfl.progm.xmldr.clone() };
    self.xmltp = if c.progm.xmltp.len() > 0
      { c.progm.xmltp.clone() } else { self.dfl.progm.xmltp.clone() };
    self.dbodr = if c.progm.dbodr.len() > 0
      { c.progm.dbodr.clone() } else { self.dfl.progm.dbodr.clone() };
    self.dbonm = if c.progm.dbonm.len() > 0
      { c.progm.dbonm.clone() } else { self.dfl.progm.dbonm.clone() };
    self.outdr = if c.progm.outdr.len() > 0
      { c.progm.outdr.clone() } else { self.dfl.progm.outdr.clone() };
    self.outfl = if c.progm.outfl.len() > 0
      { c.progm.outfl.clone() } else { self.dfl.progm.outfl.clone() };
    self.dbopt = format!("{}{}", self.dbodr, self.dbonm);
    self.outpt = format!("{}{}", self.outdr, self.outfl);
    self.dtsys = Local::now().naive_local();
    self.dtcur = Local::now().naive_local();
    self.dtnul = NaiveDateTime::MIN;
    self.fdate = NaiveDate::from_ymd_opt(1901, 1, 1).unwrap()
     .and_hms_opt(0, 0, 0).unwrap();
    self.tdate = Local::now().naive_local();
  }

  pub fn set_runvars(&mut self, p: &ParameTp) {
    self.found = false;
    for run in &self.dfl.run {
      if p.optn == run.optcd {
        if p.optn == "out" {
          self.found = true;
          if run.outdr.len() > 0 { self.outdr = run.outdr.clone(); }
          if run.outfl.len() > 0 { self.outfl = run.outfl.clone(); }
          if run.filtr.len() > 0 { self.filtr = run.filtr.clone(); }
          if run.fprm1.len() > 0 { self.fprm1 = run.fprm1.clone(); }
          if run.fprm2.len() > 0 { self.fprm2 = run.fprm2.clone(); }
          self.outpt = format!("{}{}", self.outdr, self.outfl);
        }
        break;
      }
    }
    for run in &self.cfd.run {
      if p.optn == run.optcd {
        if p.optn == "out" {
          self.found = true;
          if run.outdr.len() > 0 { self.outdr = run.outdr.clone(); }
          if run.outfl.len() > 0 { self.outfl = run.outfl.clone(); }
          if run.filtr.len() > 0 { self.filtr = run.filtr.clone(); }
          if run.fprm1.len() > 0 { self.fprm1 = run.fprm1.clone(); }
          if run.fprm2.len() > 0 { self.fprm2 = run.fprm2.clone(); }
          self.outpt = format!("{}{}", self.outdr, self.outfl);
        }
        break;
      }
    }
    if self.found && p.optn == "out" {
      if self.filtr == "current" {
        self.tdate = self.dtsys.clone();
        let  tdate = self.tdate.date();
        let  tyear = tdate.year();
        let  tmnth = tdate.month();
        let  tday  = tdate.day();
               if self.fprm1 == "year"  {
          self.fdate = NaiveDate::from_ymd_opt(tyear, 1, 1).unwrap()
            .and_hms_opt(0, 0, 0).unwrap();
        } else if self.fprm1 == "month" {
          self.fdate = NaiveDate::from_ymd_opt(tyear, tmnth, 1).unwrap()
            .and_hms_opt(0, 0, 0).unwrap();
        } else if self.fprm1 == "day"   {
          self.fdate = NaiveDate::from_ymd_opt(tyear, tmnth, tday).unwrap()
            .and_hms_opt(0, 0, 0).unwrap();
        }
      } else if self.filtr == "past" {
        self.tdate = self.dtsys.clone();
        let  dday: i64 = self.fprm1.trim().parse().unwrap();
        let  tdate = self.tdate.date();
        let  fdate = tdate - Duration::days(dday);
        self.fdate = fdate.and_hms_opt(0, 0, 0).unwrap();
      }
    }
  }
}

//**********************************************************************************
// config.rs: Reads config file
//**********************************************************************************
mod config {
  use serde::Deserialize;
  use serde_json::from_reader;
  use std::fs::File;

  #[derive(Debug, Clone, Default, Deserialize)]
  pub struct ProgramTp {
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
    pub progm: ProgramTp,
    #[serde(default)]
    pub run  : Vec<RunTp>
  }

  impl ConfigTp {
    pub fn new_config() -> ConfigTp {
      let cfg = ConfigTp{ ..Default::default() };
      cfg
    }

    pub fn get_config(&mut self, fname: &str) {
      let mut cfg: ConfigTp = Default::default();
      match File::open(fname) {
        Ok(f)  => { cfg = from_reader(f).expect("Error Deserializing JSON"); },
        Err(_) => {},
      };
      self.progm = cfg.progm;
      self.run   = cfg.run;
    }
  }
}
