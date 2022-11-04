// crtdb.rs: Creates Sqlite DB and selected tables [20220722-bar8tl]
use crate::settings::params::ParameTp;
use crate::settings::SettingsTp;
use rusqlite::{Connection, Result};

#[derive(Debug, Default)]
pub struct CreateTp {}

impl CreateTp {
  pub fn new_create() -> CreateTp {
    let d = CreateTp {};
    d
  }

  pub fn create_tables(&mut self, parm: ParameTp, mut s: SettingsTp) {
    s.set_runvars(parm);
    self.crt_acks(&s).expect("acks table creation error");
    self.crt_last(&s).expect("last table creation error");
  }

  fn crt_acks(&mut self, s: &SettingsTp) -> Result<()> {
    let conn = Connection::open(&s.env.dbfpt)?;
    conn.execute("DROP TABLE IF EXISTS acks", [])?;
    conn.execute(
      "CREATE TABLE IF NOT EXISTS acks (ackno INTEGER PRIMARY KEY,
        issue TEXT, rceiv TEXT, invoi TEXT, serie TEXT, folio TEXT, uuid TEXT,
        dtime TEXT, stats TEXT, errn1 TEXT, errn2 TEXT, notes TEXT)", [])?;
    Ok(())
  }

  fn crt_last(&mut self, s: &SettingsTp) -> Result<()> {
    let conn = Connection::open(&s.env.dbfpt)?;
    conn.execute("DROP TABLE IF EXISTS last", [])?;
    conn.execute(
      "CREATE TABLE IF NOT EXISTS last (recno TEXT PRIMARY KEY, ackno INTEGER)",
        [])?;
    conn.execute("INSERT INTO last(recno, ackno) VALUES(\"00\", 0)", [])?;
    Ok(())
  }
}
