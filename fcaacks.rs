// fcaacks.rs: Starts processes for FCA XML Invoice Acknowledgments
// (retrieval and report generation) [20220722-bar8tl]
mod create;
mod inicopy;
mod refresh;
mod update;
mod toexcel;
mod settings;

fn main() {
  let mut stg = settings::SettingsTp::new_settings();
  stg.set_settings("_config-rs.json", "_deflts-rs.json");
  let t = stg.clone();
  for parm in t.prm.cmdpr {
    let s = stg.clone();
           if parm.optn == "crt" {
      let mut crt = create::CreateTp::new_create();
      crt.create_tables(parm, s);
    } else if parm.optn == "ini" {
      let mut ini = inicopy::InicopyTp::new_inicopy();
      ini.initial_copy(parm, s);
    } else if parm.optn == "ref" {
      let mut rfh = refresh::RefreshTp::new_refresh();
      rfh.refresh_files(parm, s);
    } else if parm.optn == "upd" {
      let mut upd = update::UpdateTp::new_update();
      upd.update_tables(parm, s);
    } else if parm.optn == "out" {
      let mut out = toexcel::ToexcelTp::new_toexcel();
      out.create_excel(parm, s);
    } else {
      println!("Run option not valid");
    }
  }
}
