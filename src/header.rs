// header.rs - References to function modules being used by the program fcaacks
// (2019-03-01 bar8tl)
mod copy_acks;
mod db_creation;
mod output;
mod settings;
mod update_db;

use copy_acks::init_files::init_files;
use copy_acks::rfsh_files::rfsh_files;
use db_creation::create_tablelist::create_tablelist;
use output::crea_excel::crea_excel;
use settings::set_pgm_settings::set_pgm_settings;
use settings::set_run_settings::set_run_settings;
use update_db::updt_tables::updt_tables;

const CONFIG_FILENAME: &str = "_config.json";
