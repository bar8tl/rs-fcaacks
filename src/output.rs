//**********************************************************************************
// output.rs: Generates excel report and address in to an excel file
// (2022-04-06 bar8tl)
//**********************************************************************************
use crate::settings::SettingsTp;
use chrono::NaiveDate;
use rusqlite::Connection;
use rust_xlsxwriter::{Format, Workbook};

// toexcel - Produces output to excel file -----------------------------------------
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
pub struct ColTp<'a> {
  pub fld: &'a str,
  pub wdt: f64,
  pub dsc: &'a str
}

pub fn crea_excel(s: SettingsTp) {
  let conn = Connection::open(&s.dbopt).unwrap();
  let xlspt = s.outpt.clone();
  let mut ofile = Workbook::new(&xlspt);
  let blfmt = Format::new().set_bold();
  let wksht = ofile.add_worksheet();
  let colmn = vec![
    ColTp{fld:"ackno", wdt:10.29, dsc:"ack#"        },
    ColTp{fld:"issue", wdt:11.30, dsc:"remitente"   },
    ColTp{fld:"rceiv", wdt:12.00, dsc:"destinatario"},
    ColTp{fld:"invoi", wdt:13.14, dsc:"refProveedor"},
    ColTp{fld:"serie", wdt: 5.30, dsc:"serie"       },
    ColTp{fld:"folio", wdt: 7.00, dsc:"folio"       },
    ColTp{fld:"uuidn", wdt:41.00, dsc:"uuid"        },
    ColTp{fld:"dtime", wdt:18.90, dsc:"fechahora"   },
    ColTp{fld:"stats", wdt: 7.60, dsc:"estatus"     },
    ColTp{fld:"errn1", wdt:33.00, dsc:"error"       },
    ColTp{fld:"errn2", wdt:33.00, dsc:"error"       },
    ColTp{fld:"notas", wdt:10.00, dsc:"notas"       }
  ];
  let mut i: u16 = 0;
  for col in &colmn {
    wksht.set_column_width(i, col.wdt).expect("Setting column width error.");
    wksht.write_string(0, i, &col.dsc, &blfmt).expect("Error writing title.");
    i += 1;
  }
  let mut stmt = conn.prepare(
    "SELECT * FROM acks ORDER BY dtime, ackno").unwrap();
  let foundack_iter = stmt.query_map([], |row| { Ok(AcksTp {
      ackno: row.get(0).unwrap(),
      issue: row.get(1).unwrap(),
      rceiv: row.get(2).unwrap(),
      invoi: row.get(3).unwrap(),
      serie: row.get(4).unwrap(),
      folio: row.get(5).unwrap(),
      uuidn: row.get(6).unwrap(),
      dtime: row.get(7).unwrap(),
      stats: row.get(8).unwrap(),
      errn1: row.get(9).unwrap(),
      errn2: row.get(10).unwrap(),
      notas: row.get(11).unwrap()
    })
  }).expect("Error: No records found in table ACKS.");
  let mut j: u32 = 1;
  for foundack in foundack_iter {
    let acknm = foundack.unwrap().clone();
    let nyr: i32 = acknm.dtime[0..4].parse().unwrap();
    let nmn: u32 = acknm.dtime[5..7].parse().unwrap();
    let ndy: u32 = acknm.dtime[8..10].parse().unwrap();
    let wdate = NaiveDate::from_ymd_opt(nyr, nmn, ndy).unwrap()
      .and_hms_opt(0, 0, 0).unwrap();
    if wdate >= s.fdate && wdate <= s.tdate {
      wksht.write_string_only(j, 0, &(format!("{}",acknm.ackno).as_str())).unwrap();
      wksht.write_string_only(j, 1, &acknm.issue).unwrap();
      wksht.write_string_only(j, 2, &acknm.rceiv).unwrap();
      wksht.write_string_only(j, 3, &acknm.invoi).unwrap();
      wksht.write_string_only(j, 4, &acknm.serie).unwrap();
      wksht.write_string_only(j, 5, &acknm.folio).unwrap();
      wksht.write_string_only(j, 6, &acknm.uuidn).unwrap();
      wksht.write_string_only(j, 7, &acknm.dtime).unwrap();
      wksht.write_string_only(j, 8, &acknm.stats).unwrap();
      wksht.write_string_only(j, 9, &acknm.errn1).unwrap();
      wksht.write_string_only(j,10, &acknm.errn2).unwrap();
      wksht.write_string_only(j,11, &acknm.notas).unwrap();
      j += 1;
    }
  }
  ofile.close().expect("Error fetching output excel file.");
}
