//**********************************************************************************
// settings.rs: Declare Pgm-level & Run-level settings (2022-04-06 bar8tl)
//**********************************************************************************
use crate::settings::config::ConfigTp;
use chrono::Local;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use rblib::params::{ParamsTp, ParameTp};

const ACKS_DIR     : &str =
  "\\\\bosch.com\\dfsrb\\DfsUS\\loc\\Mx\\ILM\\Projects\\CFA\\Edifcack\\";
const ACKS_TYPE    : &str = "txt";
const XML_DIR      : &str = "files\\";
const XML_TYPE     : &str = "xml";
const DB_DIR       : &str = ".\\";
const DB_NAME      : &str = "fcaacks.db";
const OUTPUT_DIR   : &str = ".\\";
const OUTPUT_NAME  : &str = "zdcmex-rs.xlsx";
const REPORT_FILTER: &str = "current";
const FILTER_PARM1 : &str = "year";
const FILTER_PARM2 : &str = "na";

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm  : ParamsTp,
  pub cfd  : ConfigTp,
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
      { c.progm.ackdr.clone() } else { ACKS_DIR.to_string()    };
    self.acktp = if c.progm.acktp.len() > 0
      { c.progm.acktp.clone() } else { ACKS_TYPE.to_string()   };
    self.xmldr = if c.progm.xmldr.len() > 0
      { c.progm.xmldr.clone() } else { XML_DIR.to_string()     };
    self.xmltp = if c.progm.xmltp.len() > 0
      { c.progm.xmltp.clone() } else { XML_TYPE.to_string()    };
    self.dbodr = if c.progm.dbodr.len() > 0
      { c.progm.dbodr.clone() } else { DB_DIR.to_string()      };
    self.dbonm = if c.progm.dbonm.len() > 0
      { c.progm.dbonm.clone() } else { DB_NAME.to_string()     };
    self.outdr = if c.progm.outdr.len() > 0
      { c.progm.outdr.clone() } else { OUTPUT_DIR.to_string()  };
    self.outfl = if c.progm.outfl.len() > 0
      { c.progm.outfl.clone() } else { OUTPUT_NAME.to_string() };
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
    for run in &self.cfd.run {
      if p.optn == run.optcd {
        self.found = true;
        if p.optn == "out" {
          if run.outdr.len() > 0 { self.outdr = run.outdr.clone(); }
          if run.outfl.len() > 0 { self.outfl = run.outfl.clone(); }
          self.outpt = format!("{}{}", self.outdr, self.outfl);
          self.filtr = if run.filtr.len() > 0
            { run.filtr.clone() } else { REPORT_FILTER.to_string() };
          self.fprm1 = if run.fprm1.len() > 0
            { run.fprm1.clone() } else { FILTER_PARM1.to_string()  };
          self.fprm2 = if run.fprm2.len() > 0
            { run.fprm2.clone() } else { FILTER_PARM2.to_string()  };
        }
        break;
      }
    }
    if self.found && p.optn == "out" {
      if self.filtr == "current" {
        self.tdate = self.dtsys.clone();
        let tdate = self.tdate.date();
        let tyear = tdate.year();
        let tmnth = tdate.month();
        let tday  = tdate.day();
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
        let dday: i64 = self.fprm1.trim().parse().unwrap();
        let tdate = self.tdate.date();
        let fdate = tdate - Duration::days(dday);
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
  use serde_json;
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
      let f = File::open(fname).unwrap();
      let cfg: ConfigTp = serde_json::from_reader(f)
        .expect("JSON not well-formed");
      self.progm = cfg.progm;
      self.run   = cfg.run;
    }
  }
}
