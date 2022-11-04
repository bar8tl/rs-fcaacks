// toexcel.rs: Produces output to excel file [20220722-bar8tl]
#![allow(unused)]

use crate::settings::params::ParameTp;
use crate::settings::SettingsTp;
use chrono::NaiveDate;
use rusqlite::{Connection, Result};
use rust_xlsxwriter::{Format, Workbook, XlsxError};

#[derive(Debug, Clone, Default)]
pub struct AcksTp {
  pub ackno: i64,
  pub issue: String,
  pub rceiv: String,
  pub invoi: String,
  pub serie: String,
  pub folio: String,
  pub uuidn: String,
  pub dtime: String,
  pub stats: String,
  pub errn1: String,
  pub errn2: String,
  pub notas: String
}

#[derive(Debug, Clone, Default)]
pub struct ColTp {
  pub fld: String,
  pub wdt: f64,
  pub dsc: String
}

#[derive(Debug, Default)]
pub struct ToexcelTp {}

impl ToexcelTp {
  pub fn new_toexcel() -> ToexcelTp {
    let d = ToexcelTp {};
    d
  }

  pub fn create_excel(&mut self, parm: ParameTp, mut s: SettingsTp) {
    s.set_runvars(parm);
    self.list_acks(&s).expect("General error during report");
  }

  pub fn list_acks(&mut self, s: &SettingsTp) -> Result<()> {
    let conn = Connection::open(&s.env.dbfpt)?;
    let xlspt = format!("{}\\zdcmex-rs.xlsx", s.env.dbfdr);
    let mut ofile = Workbook::new(&xlspt);
    let blfmt = Format::new().set_bold();
    let wksht = ofile.add_worksheet();
    let colmn = vec![
      ColTp{fld:"ackno".to_string(), wdt:10.29, dsc:"ack#".to_string()        },
      ColTp{fld:"issue".to_string(), wdt:11.30, dsc:"remitente".to_string()   },
      ColTp{fld:"rceiv".to_string(), wdt:12.00, dsc:"destinatario".to_string()},
      ColTp{fld:"invoi".to_string(), wdt:13.14, dsc:"refProveedor".to_string()},
      ColTp{fld:"serie".to_string(), wdt: 5.30, dsc:"serie".to_string()       },
      ColTp{fld:"folio".to_string(), wdt: 7.00, dsc:"folio".to_string()       },
      ColTp{fld:"uuidn".to_string(), wdt:41.00, dsc:"uuid".to_string()        },
      ColTp{fld:"dtime".to_string(), wdt:18.90, dsc:"fechahora".to_string()   },
      ColTp{fld:"stats".to_string(), wdt: 7.60, dsc:"estatus".to_string()     },
      ColTp{fld:"errn1".to_string(), wdt:33.00, dsc:"error".to_string()       },
      ColTp{fld:"errn2".to_string(), wdt:33.00, dsc:"error".to_string()       },
      ColTp{fld:"notas".to_string(), wdt:10.00, dsc:"notas".to_string()       }
    ];
    let mut i: u16 = 0;
    for col in &colmn {
      wksht.set_column_width(i, col.wdt).expect("error");
      wksht.write_string(0, i, &col.dsc, &blfmt).expect("error");
      i += 1;
    }
    let mut stmt = conn.prepare(
      "SELECT * FROM acks ORDER BY dtime, ackno")?;
    let foundack_iter = stmt.query_map([], |row| { Ok(AcksTp {
        ackno: row.get(0)?,
        issue: row.get(1)?,
        rceiv: row.get(2)?,
        invoi: row.get(3)?,
        serie: row.get(4)?,
        folio: row.get(5)?,
        uuidn: row.get(6)?,
        dtime: row.get(7)?,
        stats: row.get(8)?,
        errn1: row.get(9)?,
        errn2: row.get(10)?,
        notas: row.get(11)?
      })
    })?;
    let mut j: u32 = 1;
    for foundack in foundack_iter {
      let acknm = foundack.unwrap().clone();
      let nyr: i32 = acknm.dtime[0..4].parse().unwrap();
      let nmn: u32 = acknm.dtime[5..7].parse().unwrap();
      let ndy: u32 = acknm.dtime[8..10].parse().unwrap();
      let wdate = NaiveDate::from_ymd(nyr, nmn, ndy).and_hms(0, 0, 0);
      if wdate >= s.env.fdate && wdate <= s.env.tdate {
        wksht.write_string_only(j, 0, &acknm.ackno.to_string()).expect("error");
        wksht.write_string_only(j, 1, &acknm.issue.to_string()).expect("error");
        wksht.write_string_only(j, 2, &acknm.rceiv.to_string()).expect("error");
        wksht.write_string_only(j, 3, &acknm.invoi.to_string()).expect("error");
        wksht.write_string_only(j, 4, &acknm.serie.to_string()).expect("error");
        wksht.write_string_only(j, 5, &acknm.folio.to_string()).expect("error");
        wksht.write_string_only(j, 6, &acknm.uuidn.to_string()).expect("error");
        wksht.write_string_only(j, 7, &acknm.dtime.to_string()).expect("error");
        wksht.write_string_only(j, 8, &acknm.stats.to_string()).expect("error");
        wksht.write_string_only(j, 9, &acknm.errn1.to_string()).expect("error");
        wksht.write_string_only(j,10, &acknm.errn2.to_string()).expect("error");
        wksht.write_string_only(j,11, &acknm.notas.to_string()).expect("error");
        j += 1;
      }
    }
    ofile.close().expect("error");
    Ok(())
  }
}
