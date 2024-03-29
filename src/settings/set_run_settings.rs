// set_run_settings.rs - Option Run/Execution level setting definition
// (2019-03-01 bar8tl)
use crate::settings::set_pgm_settings::SettingsTp;
use crate::settings::read_config_file::RunTp;
use chrono::{Datelike, Duration, NaiveDate};
use rblib::read_cmdline_args::ParameTp;

// Run options
pub const CREATE_DB: &str = "cdb";
pub const INIT_COPY: &str = "ini";
pub const REFH_COPY: &str = "ref";
pub const UPDT_LOCL: &str = "upd";
pub const OUT_EXCEL: &str = "out";

pub fn set_run_settings(s: &mut SettingsTp, p: &ParameTp) {
/* For now, none of optns will require object name. Thus this paragraph is commented
  if p.prm1.len() > 0 {
    s.objnm = p.prm1.clone();
  } else {
    panic!("Error: Not possible to determine Object name");
  } */
  s.found = 0;
  for run in s.dfl.run.clone() {
    set_optn_settings(&p.optn, &run, s);
    if s.found > 0 {
      break;
    }
  }
  for run in s.cfg.run.clone() {
    set_optn_settings(&p.optn, &run, s);
    if s.found > 0 {
      break;
    }
  }
  if s.found > 0 && p.optn == OUT_EXCEL {
    if s.filtr == "current" {
      s.tdate = s.dtsys.clone();
      let tdate = s.tdate.date();
      let tyear = tdate.year();
      let tmnth = tdate.month();
      let tday  = tdate.day();
             if s.fprm1 == "year"  {
        s.fdate = NaiveDate::from_ymd_opt(tyear, 1, 1).unwrap()
         .and_hms_opt(0, 0, 0).unwrap();
      } else if s.fprm1 == "month" {
        s.fdate = NaiveDate::from_ymd_opt(tyear, tmnth, 1).unwrap()
         .and_hms_opt(0, 0, 0).unwrap();
      } else if s.fprm1 == "day"   {
        s.fdate = NaiveDate::from_ymd_opt(tyear, tmnth, tday).unwrap()
         .and_hms_opt(0, 0, 0).unwrap();
      }
    } else if s.filtr == "past" {
      s.tdate = s.dtsys.clone();
      let dday: i64 = s.fprm1.trim().parse().unwrap();
      let tdate = s.tdate.date();
      let fdate = tdate - Duration::days(dday);
      s.fdate = fdate.and_hms_opt(0, 0, 0).unwrap();
    }
  }
}

fn set_optn_settings(optn: &String, run: &RunTp, s: &mut SettingsTp) {
  if optn == &run.optcd /* && p.prm1 == run.objnm */ {
    if optn == OUT_EXCEL {
      if run.outdr.len() > 0 { s.outdr = run.outdr.clone(); }
      if run.outfl.len() > 0 { s.outfl = run.outfl.clone(); }
      if run.filtr.len() > 0 { s.filtr = run.filtr.clone(); }
      if run.fprm1.len() > 0 { s.fprm1 = run.fprm1.clone(); }
      if run.fprm2.len() > 0 { s.fprm2 = run.fprm2.clone(); }
      s.outpt = format!("{}{}", s.outdr, s.outfl);
    }
  }
  s.found += 1;
}
