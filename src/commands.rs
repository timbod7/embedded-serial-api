

use crate::adl::protocol::{Protocol,Request};
use serde::{Deserialize,Serialize};
use serde::de::{DeserializeOwned};
use serde_json;

pub fn execute_str(name: &str, reqstr: &str) -> Result<(),String> {
    let protocol: Protocol = Protocol::new();

    match name {
        "getServo1" => execute(protocol.get_servo_1, reqstr),
        _ => Result::Err(format!("unknown command: {}", name)),
    }
}

fn execute<I,O>(req: Request<I,O>, reqstr: &str) -> Result<(),String>
  where I: DeserializeOwned {
    let req_body: Result<I,serde_json::Error> = serde_json::from_str(reqstr);
    Result::Ok(())
}