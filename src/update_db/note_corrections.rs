// note_corrections.rs - Add correction notes into the db ack entries when the xml
// ack specify status 00 (2019-03-01 bar8tl)
use rusqlite::Connection;

#[derive(Debug, Clone, Default)]
pub struct AcknTp {
  pub ackno: i64
}

#[derive(Debug, Clone, Default)]
pub struct AckkTp {
  pub ackno: i64,
  pub serie: String,
  pub folio: String
}

pub fn note_corrections(cnn: &Connection) {
  let mut stmt1 = cnn.prepare(
    "SELECT ackno, serie, folio FROM acks WHERE stats<>\"00\" ORDER BY ackno")
    .unwrap();
  let failedack_iter = stmt1.query_map([], |row| { Ok(AckkTp {
      ackno: row.get(0).unwrap(),
      serie: row.get(1).unwrap(),
      folio: row.get(2).unwrap()
    })
  }).unwrap();
  for failedack in failedack_iter {
    let flack = failedack.unwrap().clone();
    let mut stmt2 = cnn.prepare(
      "SELECT ackno, serie, folio FROM acks WHERE stats=\"00\" AND serie=?1 AND
        folio=?2 AND ackno<>?3").unwrap();
    let matchack_iter = stmt2.query_map([flack.serie, flack.folio,
      flack.ackno.to_string()], |row| { Ok(AcknTp {
        ackno: row.get(0)?
      })
    }).unwrap();
    for matchack in matchack_iter {
      let mtack = matchack.unwrap().clone();
      let notes = format!("Corrected with ack# {}", mtack.ackno);
      cnn.execute("UPDATE acks SET notes=?1 WHERE ackno=?2",
        [notes, flack.ackno.to_string()]).expect("Error: Correction note failed.");
    }
  }
}
