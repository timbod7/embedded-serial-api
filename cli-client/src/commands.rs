use crate::adl::protocol::{Protocol, Request};
use byteorder::ByteOrder;
use fletcher::Fletcher16;
use heapless::{consts::*, Vec};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use serialport::SerialPort;
use std::convert::TryInto;
use std::error::Error;
use std::collections::HashMap;
use std::mem::size_of;

const STX: u8 = 2;
const ETX: u8 = 3;


type CmdResult<T> = Result<T, Box<dyn Error>>;
type SPort = Box<dyn SerialPort>;

type CmdMap =  HashMap<String, Box<dyn Fn(&mut SPort, &str) -> CmdResult<()>>>;

pub struct Commands {
    commands : CmdMap
}

impl Commands {
    pub fn new() -> Commands {
        let mut commands: CmdMap = HashMap::new();

        // TODO: It would be nice to abstract the following via a helper function,
        // but that is currently beyond my rust capabilities.
        let req =  Protocol::def_get_led_1();
        commands.insert(req.name.clone(), Box::new(move |sport, valuestr| {
            execute(sport, &req, valuestr)
        }));
        let req =  Protocol::def_get_servo_1();
        commands.insert(req.name.clone(), Box::new(move |sport, valuestr| {
            execute(sport, &req, valuestr)
        }));
        let req =  Protocol::def_get_servo_2();
        commands.insert(req.name.clone(), Box::new(move |sport, valuestr| {
            execute(sport, &req, valuestr)
        }));
        let req =  Protocol::def_set_led_1();
        commands.insert(req.name.clone(), Box::new(move |sport, valuestr| {
            execute(sport, &req, valuestr)
        }));
        let req =  Protocol::def_set_servo_1();
        commands.insert(req.name.clone(), Box::new(move |sport, valuestr| {
            execute(sport, &req, valuestr)
        }));
        let req =  Protocol::def_set_servo_2();
        commands.insert(req.name.clone(), Box::new(move |sport, valuestr| {
            execute(sport, &req, valuestr)
        }));
        Commands {
            commands: HashMap::new()
        }
    }

    pub fn execute_str(& self, sport: &mut SPort, name: &str, reqstr: &str) -> CmdResult<()> {
        match self.commands.get(name) {
            Option::None => Result::Err(app_error("Unknown command")),
            Option::Some(cmdf) => cmdf(sport, reqstr)
        }
    }
}


fn execute<I, O>(
    sport: &mut SPort,
    req: &Request<I, O>,
    reqstr: &str,
) -> CmdResult<()>
where
    I: DeserializeOwned + Serialize,
    O: DeserializeOwned + Serialize,
{
    let req_value: I = serde_json::from_str(reqstr)?;
    write_request(sport, &req, &req_value)?;

    let resp_value: O = read_response(sport, &req)?;
    format!("result = {}", serde_json::to_string(&resp_value)?);
    Result::Ok(())
}

const REQ_HEADER_SIZE: usize = size_of::<u8>() + 2 * size_of::<u16>();
const REQ_TAIL_SIZE: usize = size_of::<u8>() + 2 * size_of::<u16>();
type MaxValueSize = U11;

fn write_request<I, O>(
    sport: &mut SPort,
    req: &Request<I, O>,
    req_value: &I,
) -> CmdResult<()>
where
    I: DeserializeOwned + Serialize,
{
    let req_bytes: Vec<u8, MaxValueSize> = postcard::to_vec(&req_value)?;

    let mut req_header: [u8; REQ_HEADER_SIZE] = [0; REQ_HEADER_SIZE];
    req_header[0] = STX;
    byteorder::LittleEndian::write_u16(&mut req_header[1..3], req.reqid);
    byteorder::LittleEndian::write_u16(&mut req_header[3..5], req_bytes.len().try_into().unwrap());

    let mut req_tail: [u8; REQ_TAIL_SIZE] = [0; REQ_TAIL_SIZE];
    let mut checksum = Fletcher16::new();
    // We checksum the reqid and body
    checksum.update(req_header[1..3].try_into().unwrap());
    checksum.update(&req_bytes);
    byteorder::LittleEndian::write_u16(&mut req_tail[0..2], checksum.value());
    req_header[0] = ETX;

    sport.write_all(&req_header)?;
    sport.write_all(&req_bytes)?;
    sport.write_all(&req_tail)?;

    Result::Ok(())
}

const RESP_HEADER_SIZE: usize = size_of::<u8>() + size_of::<u16>();
const RESP_TAIL_SIZE: usize = size_of::<u8>() + size_of::<u16>();

fn read_response<I, O>(sport: &mut SPort, _req: &Request<I, O>) -> CmdResult<O>
where
    O: DeserializeOwned,
{
    let mut req_header: [u8; RESP_HEADER_SIZE] = [0; RESP_HEADER_SIZE];
    sport.read_exact(&mut req_header)?;
    if req_header[0] != STX {
        return Result::Err(app_error("missing stx"));
    }

    let mut req_bytes: Vec<u8, MaxValueSize> = Vec::new();
    let size = byteorder::LittleEndian::read_u16(&req_header[1..3]);
    req_bytes
        .resize_default(size.try_into()?)
        .expect("buffer too small");
    sport.read_exact(&mut req_bytes)?;

    let mut req_tail: [u8; RESP_TAIL_SIZE] = [0; RESP_TAIL_SIZE];
    sport.read_exact(&mut req_tail)?;
    if req_tail[2] != ETX {
        return Result::Err(app_error("missing etx"));
    }
    let mut checksum = Fletcher16::new();
    checksum.update(&req_bytes);
    if checksum.value() != byteorder::LittleEndian::read_u16(&req_header[0..2]) {
        return Result::Err(app_error("incorrect checksum"));
    }
    let (req_value, _) = postcard::take_from_bytes(&req_bytes)?;
    Result::Ok(req_value)
}

fn app_error(msg: &str) -> Box<dyn Error> {
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, msg))
}
