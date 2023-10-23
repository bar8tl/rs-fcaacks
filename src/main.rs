// main.rs - Program to test/start functions to generate Chrysler XML Invoices
// Acks Report (2019-03-01 bar8tl)
include!("header.rs");

fn main() {
  let stg = set_pgm_settings(CONFIG_FILENAME);
  let t = stg.clone();
  for p in t.prm.cmdpr {
    let mut s = stg.clone();
    set_run_settings(&mut s, &p);
    match p.optn.as_str() {
      CREATE_DB => create_tablelist(s.dbopt),
      INIT_COPY => init_files (s.ackdr, s.xmldr, s.acktp, s.xmltp),
      REFH_COPY => rfsh_files (s.dbopt, s.ackdr, s.xmldr, s.acktp, s.xmltp),
      UPDT_LOCL => updt_tables(s.dbopt, s.xmldr, s.xmltp),
      OUT_EXCEL => crea_excel (s.dbopt, s.outpt, s.fdate, s.tdate),
              _ => println!("Run option not valid"),
    };
  }
}
