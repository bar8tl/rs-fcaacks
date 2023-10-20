// crea_tablelist.rs - Sqlite3 DB tables creation for local acks storage
// (2019-03-01 bar8tl)
use rblib::create_sqlite3_tablelist::{TlistTp, create_sqlite3_tablelist};
use serde::Deserialize;
use serde_json;

const ITABLES: &str = include!("_sqlstmts.json");

#[derive(Debug, Clone, Default, Deserialize)]
struct SqlstTp {
  activ: String,
  table: String,
  sqlst: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ItablesTp {
  sqlst: Vec<SqlstTp>
}

pub fn create_tablelist(dbopt: String) {
  let it: ItablesTp = serde_json::from_str(ITABLES).unwrap();
  let mut tlist: Vec<TlistTp> = Vec::with_capacity(it.sqlst.len());
  for sql in &it.sqlst {
    if sql.activ.to_lowercase() == "yes"  {
      tlist.push(TlistTp {table: sql.table.clone(), sqlst: sql.sqlst.clone()});
    }
  }
  if tlist.len() > 0 {
    create_sqlite3_tablelist(&dbopt, &tlist);
  }
}
