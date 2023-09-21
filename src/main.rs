//**********************************************************************************
// main.rs: Starts processes for Chrysler XML Invoice Acknowledgments - retrieval
// and report (2022-04-06 bar8tl)
//**********************************************************************************
mod copyacks;
mod dbase;
mod output;
mod settings;

fn main() {
  let optns = ["cdb", "ini", "ref", "upd", "out"];
  let funcs = [
    dbase   ::crea_tables, // Set/Reset DB tables
    copyacks::init_files,  // Initial Chrysler files copy into a local folder
    copyacks::rfsh_files,  // Keep local folder updated with newly received files
    dbase   ::updt_tables, // Maintain local acks DB updated
    output  ::crea_excel   // Produce output to excel file
  ];
  let stg = settings::SettingsTp::new_settings();
  let t = stg.clone();
  for p in t.prm.cmdpr {
    let mut s = stg.clone();
    s.set_runvars(&p);
    funcs[optns.iter().position(|&x| x == p.optn).unwrap()](s);
  }
}
