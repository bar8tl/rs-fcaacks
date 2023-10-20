// get_lastack.rs - Determine the ack# of last ack stored into the local db
// (2019-03-01 bar8tl)
use rusqlite::Connection;

pub fn get_lastack(cnn: &Connection) -> i64 {
  let mut lsack = 0i64;
  cnn.query_row("SELECT ackno FROM last WHERE recno=\"00\"", [], |row| {
    Ok(lsack = row.get(0).unwrap())})
    .expect("Error: Control record in table LAST not found.");
  return lsack;
}
