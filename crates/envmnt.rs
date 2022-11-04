// envmnt.rs: Establishes global environment variables [20220722-bar8tl]
pub mod config;
pub mod deflts;

use crate::settings::envmnt::config::ConfigTp;
use crate::settings::envmnt::deflts::DefltsTp;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Default)]
pub struct EnvmntTp {
  pub ackdr: String,
  pub acktp: String,
  pub xmldr: String,
  pub xmltp: String,
  pub dbfdr: String,
  pub dbfnm: String,
  pub dbfpt: String,
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

impl EnvmntTp {
  pub fn new_envmnt() -> EnvmntTp {
    let env = EnvmntTp { ..Default::default() };
    env
  }

  pub fn set_envmnt(&mut self, c: &ConfigTp, d: &DefltsTp) {
    self.ackdr = if c.progm.ackdr.len() > 0
      { c.progm.ackdr.clone() } else { d.deflt.acks_dir.clone()  };
    self.acktp = if c.progm.acktp.len() > 0
      { c.progm.acktp.clone() } else { d.deflt.acks_type.clone() };
    self.xmldr = if c.progm.xmldr.len() > 0
      { c.progm.xmldr.clone() } else { d.deflt.xml_dir.clone()   };
    self.xmltp = if c.progm.xmltp.len() > 0
      { c.progm.xmltp.clone() } else { d.deflt.xml_type.clone()  };
    self.dbfdr = if c.progm.dbfdr.len() > 0
      { c.progm.dbfdr.clone() } else { d.deflt.db_dir.clone()    };
    self.dbfnm = if c.progm.dbfnm.len() > 0
      { c.progm.dbfnm.clone() } else { d.deflt.db_name.clone()   };
    self.dbfpt = format!("{}{}", self.dbfdr, self.dbfnm);
    self.dtsys = Local::now().naive_local();
    self.dtcur = Local::now().naive_local();
    self.dtnul = NaiveDateTime::MIN;
    self.fdate = NaiveDate::from_ymd(1901, 1, 1).and_hms(0, 0, 0);
    self.tdate = Local::now().naive_local();
  }
}
