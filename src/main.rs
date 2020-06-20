extern crate clap;

use clap::{Arg, App, SubCommand};

mod adl;
mod commands;

fn main() {
  let app_m = App::new("serial-demo")
    .version("0.0")
    .about("Serial Protocol demo")
    .author("Tim Docker")
    .subcommand(SubCommand::with_name("cmd")
      .about("send a command to the device")
      .arg(Arg::with_name("name")
          .help("the name of the parameter")
          .required(true)
        )
      .arg(Arg::with_name("value")
        .help("the json formatted value")
        .required(true)
      )
    )
  .get_matches();

  match app_m.subcommand() {
    ("cmd", Some(sub_m)) => {
      if let (Some(name),Some(value)) = (sub_m.value_of("name"), sub_m.value_of("value")) {
        match commands::execute_str(name, value) {
          Result::Ok(_)  => (),
          Result::Err(msg)  => println!("Error: {}", msg),
        }
      }
    },
    _ => {},
  }
}
