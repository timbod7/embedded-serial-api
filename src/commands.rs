use crate::adl::protocol::{Protocol,Request};
use serde::{Serialize};
use serde::de::{DeserializeOwned};
use serde_json;
use heapless::{Vec, consts::*};
use std::fmt::Display;
use serialport::SerialPort;
use byteorder::ByteOrder;
use std::convert::TryInto;
use std::mem::size_of;
use fletcher::Fletcher16;


const STX : u8 = 2;
const ETX : u8 = 3;

pub fn execute_str(sport: &mut Box<dyn SerialPort>, name: &str, reqstr: &str) -> Result<(),String> {
    let protocol: Protocol = Protocol::new();
    match name {
        "getServo1" => execute(sport, &protocol.get_servo_1, reqstr),
        _ => Result::Err(format!("unknown command: {}", name)),
    }
}

fn execute<I,O>(sport: &mut Box<dyn SerialPort>, req: &Request<I,O>, reqstr: &str) -> Result<(),String>
  where I: DeserializeOwned + Serialize,
        O: DeserializeOwned + Serialize {

    let req_value : I = serde_json::from_str(reqstr).map_err(as_string)?;
    write_request(sport, req, &req_value)?;

    let resp_value : O = read_response(sport, req)?;
    format!("result = {}", serde_json::to_string(&resp_value).expect("Failed to serialize result to json"));
    Result::Ok(())
}

const REQ_HEADER_SIZE: usize = size_of::<u8>() + 2 * size_of::<u16>();
const REQ_TAIL_SIZE: usize = size_of::<u8>() + 2 * size_of::<u16>();
type MaxValueSize = U11;

fn write_request<I, O>(sport: &mut Box<dyn SerialPort>, req: &Request<I,O>, req_value: &I) -> Result<(),String>
where I: DeserializeOwned + Serialize {
    let req_bytes: Vec<u8, MaxValueSize> = postcard::to_vec(&req_value).map_err(as_string)?;

    let mut req_header: [u8;REQ_HEADER_SIZE] = [0;REQ_HEADER_SIZE];
    req_header[0] = STX;
    byteorder::LittleEndian::write_u16(&mut req_header[1..3], req.reqid);
    byteorder::LittleEndian::write_u16(&mut req_header[3..5], req_bytes.len().try_into().unwrap() );

    let mut req_tail: [u8;REQ_TAIL_SIZE] = [0;REQ_TAIL_SIZE];
    let mut checksum = Fletcher16::new();
    // We checksum the reqid and body
    checksum.update(req_header[1..3].try_into().unwrap());
    checksum.update(&req_bytes);
    byteorder::LittleEndian::write_u16(&mut req_tail[0..2], checksum.value());
    req_header[0] = ETX;

    sport.write_all(&req_header).map_err(as_string)?;
    sport.write_all(&req_bytes).map_err(as_string)?;
    sport.write_all(&req_tail).map_err(as_string)?;

    Result::Ok(())
}

const RESP_HEADER_SIZE: usize = size_of::<u8>() + size_of::<u16>();
const RESP_TAIL_SIZE: usize = size_of::<u8>() + size_of::<u16>();

fn read_response<I, O>(sport: &mut Box<dyn SerialPort>, _req: &Request<I,O>) -> Result<O,String>
  where O: DeserializeOwned {

    let mut req_header: [u8;RESP_HEADER_SIZE] = [0;RESP_HEADER_SIZE];
    sport.read_exact(&mut req_header).map_err(as_string)?;
    if req_header[0] != STX {
        return Result::Err(String::from("missing stx"))
    }

    let mut req_bytes: Vec<u8, MaxValueSize> = Vec::new();
    let size = byteorder::LittleEndian::read_u16(&req_header[1..3]);
    req_bytes.resize_default(size.try_into().unwrap());
    sport.read_exact(&mut req_bytes).map_err(as_string)?;

    let mut req_tail: [u8;RESP_TAIL_SIZE] = [0;RESP_TAIL_SIZE];
    sport.read_exact(&mut req_tail).map_err(as_string)?;
    if req_tail[2] != ETX {
        return Result::Err(String::from("missing etx"))
    }
    let mut checksum = Fletcher16::new();
    checksum.update(&req_bytes);
    if checksum.value() !=  byteorder::LittleEndian::read_u16(&req_header[0..2]) {
        return Result::Err(String::from("incorrect checksum"))
    }
    let (req_value,_) = postcard::take_from_bytes(&req_bytes).map_err(as_string)?;
    Result::Ok(req_value)
}

fn as_string<F:Display>(f:F) -> String {
    // better way?
    format!("{}", f)
}
