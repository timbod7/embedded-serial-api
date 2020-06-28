// @generated from adl module protocol

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone,Deserialize,Eq,Hash,PartialEq,Serialize)]
pub struct Request<I, O> {
  #[serde(default="Request::<I, O>::def_itype")]
  pub itype: std::marker::PhantomData<I>,

  #[serde(default="Request::<I, O>::def_otype")]
  pub otype: std::marker::PhantomData<O>,

  pub name: String,

  pub reqid: u16,
}

impl<I, O> Request<I, O> {
  pub fn new(name: String, reqid: u16) -> Request<I, O> {
    Request {
      itype: Request::<I, O>::def_itype(),
      otype: Request::<I, O>::def_otype(),
      name: name,
      reqid: reqid,
    }
  }

  pub fn def_itype() -> std::marker::PhantomData<I> {
    std::marker::PhantomData
  }

  pub fn def_otype() -> std::marker::PhantomData<O> {
    std::marker::PhantomData
  }
}

#[derive(Clone,Deserialize,Eq,Hash,PartialEq,Serialize)]
pub struct Protocol {
  #[serde(default="Protocol::def_get_led_1")]
  #[serde(rename="getLed1")]
  pub get_led_1: Request<(), bool>,

  #[serde(default="Protocol::def_set_led_1")]
  #[serde(rename="setLed1")]
  pub set_led_1: Request<bool, ()>,

  #[serde(default="Protocol::def_get_led_2")]
  #[serde(rename="getLed2")]
  pub get_led_2: Request<(), bool>,

  #[serde(default="Protocol::def_set_led_2")]
  #[serde(rename="setLed2")]
  pub set_led_2: Request<bool, ()>,
}

impl Protocol {
  pub fn new() -> Protocol {
    Protocol {
      get_led_1: Protocol::def_get_led_1(),
      set_led_1: Protocol::def_set_led_1(),
      get_led_2: Protocol::def_get_led_2(),
      set_led_2: Protocol::def_set_led_2(),
    }
  }

  pub fn def_get_led_1() -> Request<(), bool> {
    Request::<(), bool>{itype : std::marker::PhantomData, otype : std::marker::PhantomData, name : "getLed1".to_string(), reqid : 1_u16}
  }

  pub fn def_set_led_1() -> Request<bool, ()> {
    Request::<bool, ()>{itype : std::marker::PhantomData, otype : std::marker::PhantomData, name : "setLed1".to_string(), reqid : 2_u16}
  }

  pub fn def_get_led_2() -> Request<(), bool> {
    Request::<(), bool>{itype : std::marker::PhantomData, otype : std::marker::PhantomData, name : "getLed1".to_string(), reqid : 1_u16}
  }

  pub fn def_set_led_2() -> Request<bool, ()> {
    Request::<bool, ()>{itype : std::marker::PhantomData, otype : std::marker::PhantomData, name : "setLed1".to_string(), reqid : 2_u16}
  }
}
