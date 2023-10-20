// process_acks.rs - Convert acks from xml format to db records (2019-03-01 bar8tl)
use crate::update_db::isrt_acks::isrt_acks;
use crate::update_db::updt_tables::AcuseReciboTp;
use rusqlite::Connection;
use std::fs::File;

pub fn process_ack(cnn: &Connection, xmldr: &String, filid: &str, ifile: i64,
  ticks: &mut i64) {
  let xmlpt = format!("{}{}", xmldr, filid);
  let f = File::open(&xmlpt).expect(&format!("Cannot open file {}", xmlpt));
  let ack: AcuseReciboTp = serde_xml_rs::de::from_reader(f).unwrap();
  isrt_acks(&cnn, ifile, &ack);
  tick(ticks);
}

fn tick(ticks: &mut i64) {
  *ticks = *ticks + 1;
  if *ticks % 10i64 == 0 {
    print!(".");
  }
}
