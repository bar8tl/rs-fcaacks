// params.rs: Gets a list of command-line parameters [20220722-bar8tl]
use std::env;

#[derive(Debug, Clone, Default)]
pub struct ParameTp {
  pub optn: String,
  pub prm1: String,
  pub prm2: String
}

#[derive(Debug, Clone, Default)]
pub struct ParamsTp {
  pub cmdpr: Vec<ParameTp>,
  pub messg: String
}

impl ParamsTp {
  pub fn new_params() -> ParamsTp {
    let prm = ParamsTp { cmdpr: Vec::new(), messg: String::from("") };
    prm
  }

  pub fn scan_params(&mut self) {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
      self.messg = String::from("Run option missing");
      return;
    }
    let mut argn: i32 = 0;
    for curarg in args {
      if argn > 0 {
        if curarg[0..1] == "-".to_string() || curarg[0..1] == "/".to_string() {
          let mut optn: String = curarg[1..].trim().to_lowercase();
          let mut prm1: String = "".to_string();
          let mut prm2: String = "".to_string();
          if optn != "".to_string() {
            let idx = optn.find(":");
            if idx != None {
              let i = idx.unwrap();
              prm1 = optn[i + 1..].trim().to_string();
              optn = optn[..i].trim().to_string();
              let idx = prm1.find(":");
              if idx != None {
                let j = idx.unwrap();
                prm2 = prm1[j + 1..].trim().to_string();
                prm1 = prm1[..j].trim().to_string();
              }
            }
            self.cmdpr.push(ParameTp { optn, prm1, prm2 });
          }
        } else {
          self.messg = String::from("Run option missing");
        }
      }
      argn += 1;
    }
  }
}
