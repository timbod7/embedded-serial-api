extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
  let app_m = App::new("serial-demo")
    .version("0.0")
    .about("Serial Protocol demo")
    .author("Tim Docker")
    .subcommand(SubCommand::with_name("get")
      .about("gets a value from the device")
      .arg(Arg::with_name("name")
          .help("the name of the parameter")
          .required(true)
        )
    )
    .subcommand(SubCommand::with_name("set")
    .about("sets a value on the device")
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
    ("get", Some(sub_m)) => {
      if let Some(name) = sub_m.value_of("name") {
        cmd_get(name);
      }
    },
    ("set", Some(sub_m)) => {
      if let (Some(name),Some(value)) = (sub_m.value_of("name"), sub_m.value_of("value")) {
        cmd_set(name, value);
      }
    },
    _ => {},
  }
}

fn cmd_get(name: &str) {
  println!("get!")

}

fn cmd_set(name: &str, value: &str) {
  println!("set!")
}


