// set_run_settings.rs - Option Run/Execution level setting definition
// (2019-03-01 bar8tl)
use crate::settings::set_pgm_settings::SettingsTp;
use chrono::{Datelike, Duration, NaiveDate};
use rblib::read_cmdline_args::ParameTp;

pub fn set_run_settings(s: &mut SettingsTp, p: &ParameTp) {
  for run in &s.dfl.run {
    if p.optn == run.optcd {
      if p.optn == "out" {
        if run.outdr.len() > 0 { s.outdr = run.outdr.clone(); }
        if run.outfl.len() > 0 { s.outfl = run.outfl.clone(); }
        if run.filtr.len() > 0 { s.filtr = run.filtr.clone(); }
        if run.fprm1.len() > 0 { s.fprm1 = run.fprm1.clone(); }
        if run.fprm2.len() > 0 { s.fprm2 = run.fprm2.clone(); }
        s.outpt = format!("{}{}", s.outdr, s.outfl);
      }
      break;
    }
  }
  for run in &s.cfg.run {
    if p.optn == run.optcd {
      if p.optn == "out" {
        if run.outdr.len() > 0 { s.outdr = run.outdr.clone(); }
        if run.outfl.len() > 0 { s.outfl = run.outfl.clone(); }
        if run.filtr.len() > 0 { s.filtr = run.filtr.clone(); }
        if run.fprm1.len() > 0 { s.fprm1 = run.fprm1.clone(); }
        if run.fprm2.len() > 0 { s.fprm2 = run.fprm2.clone(); }
        s.outpt = format!("{}{}", s.outdr, s.outfl);
      }
      break;
    }
  }
  if s.found && p.optn == "out" {
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
