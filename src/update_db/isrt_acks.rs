// isrt_acks.rs - Record acks into the local acks db (2019-03-01 bar8tl)
use crate::update_db::updt_tables::AcuseReciboTp;
use rusqlite::Connection;

pub fn isrt_acks(cnn: &Connection, ifile: i64, ack: &AcuseReciboTp) {
  let mut err1 = String::new();
  let mut err2 = String::new();
  let notes    = String::new();
  if ack.error.len() >= 1 {
    err1 = ack.error[0].clone();
  }
  if ack.error.len() >= 2 {
    err2 = ack.error[1].clone();
  }
  cnn.execute("INSERT INTO acks VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)", (
    ifile,
    &ack.ruta.remitente,
    &ack.ruta.destinatario,
    &ack.documento.referenciaProveedor,
    &ack.documento.serie,
    &ack.documento.folioFiscal,
    &ack.documento.UUID,
    &ack.recepcion.fechahora,
    &ack.recepcion.estatus,
    err1, err2, notes
  )).expect("Error insering a new ACKS record.");
}
