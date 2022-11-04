// settings.rs: Define FCA Acks report pgm-level & run-level settings
// [20220722-bar8tl]
mod envmnt;
pub mod params;

use crate::settings::envmnt::config::ConfigTp;
use crate::settings::envmnt::deflts::DefltsTp;
use crate::settings::envmnt::EnvmntTp;
use crate::settings::params::{ParameTp, ParamsTp};
use chrono::{Datelike, Duration, NaiveDate};

#[derive(Debug, Clone, Default)]
pub struct SettingsTp {
  pub prm: ParamsTp,
  pub cfd: ConfigTp,
  pub dfl: DefltsTp,
  pub env: EnvmntTp
}

impl SettingsTp {
  pub fn new_settings() -> SettingsTp {
    let mut stg = SettingsTp { ..Default::default() };
    stg.prm = ParamsTp::new_params();
    stg.cfd = ConfigTp::new_config();
    stg.dfl = DefltsTp::new_deflts();
    stg.env = EnvmntTp::new_envmnt();
    stg
  }

  pub fn set_settings(&mut self, cfnam: &str, dfnam: &str) {
    self.prm.scan_params();
    self.cfd.get_config(cfnam);
    self.dfl.get_deflts(dfnam);
    let c = &self.cfd;
    let d = &self.dfl;
    self.env.set_envmnt(c, d);
  }

  pub fn set_runvars(&mut self, p: ParameTp) {
    self.env.found = false;
    for run in &self.cfd.run {
      if p.optn == run.optcd {
        if run.filtr.len() > 0 { self.env.filtr = run.filtr.clone(); }
        if run.fprm1.len() > 0 { self.env.fprm1 = run.fprm1.clone(); }
        if run.fprm2.len() > 0 { self.env.fprm2 = run.fprm2.clone(); }
        self.env.found = true;
        break;
      }
    }
    if p.optn == "out" {
      if self.env.filtr == "current" {
        self.env.tdate = self.env.dtsys.clone();
        let tdate = self.env.tdate.date();
        let tyear = tdate.year();
        let tmnth = tdate.month();
        let tday  = tdate.day();
               if self.env.fprm1 == "year"  {
          self.env.fdate = NaiveDate::from_ymd(tyear, 1, 1)
            .and_hms(0, 0, 0);
        } else if self.env.fprm1 == "month" {
          self.env.fdate = NaiveDate::from_ymd(tyear, tmnth, 1)
            .and_hms(0, 0, 0);
        } else if self.env.fprm1 == "day"   {
          self.env.fdate = NaiveDate::from_ymd(tyear, tmnth, tday)
            .and_hms(0, 0, 0);
        }
      } else if self.env.filtr == "past" {
        self.env.tdate = self.env.dtsys.clone();
        let dday: i64 = self.env.fprm1.trim().parse().unwrap();
        let tdate = self.env.tdate.date();
        let fdate = tdate - Duration::days(dday);
        self.env.fdate = fdate.and_hms(0, 0, 0);
      }
    }
  }
}
