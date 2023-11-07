#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use ahash::AHashMap;
use memmap2::MmapOptions;
use napi::bindgen_prelude::*;
use napi::Error;
use parser::parser_settings::rm_user_friendly_names;
use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::parser_thread_settings::create_huffman_lookup_table;
use parser::variants::soa_to_aos;
use parser::variants::BytesVariant;
use parser::variants::OutputSerdeHelperStruct;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

#[napi]
pub fn parse_chat_messages(path_or_buf: Either<String, Buffer>) -> Result<Value> {
  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: Arc::new(bytes),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_events: vec![],
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let s = match serde_json::to_value(&output.chat_messages) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn list_game_events(path_or_buf: Either<String, Buffer>) -> Result<Value> {
  let bytes = resolve_byte_type(path_or_buf);

  let arc_huf = Arc::new(create_huffman_lookup_table());
  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: Arc::new(bytes),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_events: vec!["all".to_string()],
    parse_ents: false,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: false,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let v = Vec::from_iter(output.game_events_counter.iter());
  let s = match serde_json::to_value(v) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_grenades(path_or_buf: Either<String, Buffer>) -> Result<Value> {
  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: Arc::new(bytes),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_events: vec![],
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: true,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let s = match serde_json::to_value(&output.projectiles) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
#[napi]
pub fn parse_header(path_or_buf: Either<String, Buffer>) -> Result<Value> {
  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: Arc::new(bytes),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_events: vec![],
    parse_ents: false,
    wanted_ticks: vec![],
    parse_projectiles: true,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let _output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let mut hm: HashMap<String, String> = HashMap::default();
  hm.extend(parser.header);

  let s = match serde_json::to_value(&hm) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_event(
  path_or_buf: Either<String, Buffer>,
  event_name: String,
  player_extra: Option<Vec<String>>,
  other_extra: Option<Vec<String>>,
) -> Result<Value> {
  let player_props = match player_extra {
    Some(p) => p,
    None => vec![],
  };
  let other_props = match other_extra {
    Some(p) => p,
    None => vec![],
  };
  let real_names_player = match rm_user_friendly_names(&player_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let real_other_props = match rm_user_friendly_names(&other_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let mut real_name_to_og_name = AHashMap::default();
  for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }
  for (real_name, user_friendly_name) in real_other_props.iter().zip(&other_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }

  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: real_name_to_og_name,
    bytes: Arc::new(bytes),
    wanted_player_props: real_names_player.clone(),
    wanted_player_props_og_names: vec![],
    wanted_other_props: real_other_props,
    wanted_other_props_og_names: vec![],
    wanted_events: vec![event_name],
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let s = match serde_json::to_value(&output.game_events) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
#[napi]
pub fn parse_events(
  path_or_buf: Either<String, Buffer>,
  event_names: Option<Vec<String>>,
  player_extra: Option<Vec<String>>,
  other_extra: Option<Vec<String>>,
) -> Result<Value> {
  let event_names = match event_names {
    None => return Err(Error::new(Status::InvalidArg, "No events provided!")),
    Some(v) => v,
  };
  let player_props = match player_extra {
    Some(p) => p,
    None => vec![],
  };
  let other_props = match other_extra {
    Some(p) => p,
    None => vec![],
  };
  let real_names_player = match rm_user_friendly_names(&player_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let real_other_props = match rm_user_friendly_names(&other_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let mut real_name_to_og_name = AHashMap::default();
  for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }
  for (real_name, user_friendly_name) in real_other_props.iter().zip(&other_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }

  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: real_name_to_og_name,
    bytes: Arc::new(bytes),
    wanted_player_props: real_names_player.clone(),
    wanted_player_props_og_names: vec![],
    wanted_other_props: real_other_props.clone(),
    wanted_other_props_og_names: vec![],
    wanted_events: event_names,
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let s = match serde_json::to_value(&output.game_events) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_ticks(
  path_or_buf: Either<String, Buffer>,
  wanted_props: Vec<String>,
  wanted_ticks: Option<Vec<i32>>,
  struct_of_arrays: Option<bool>,
) -> Result<Value> {
  let mut real_names = match rm_user_friendly_names(&wanted_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());
  let mut real_name_to_og_name = AHashMap::default();

  for (real_name, user_friendly_name) in real_names.iter().zip(&wanted_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }
  let wanted_ticks = match wanted_ticks {
    Some(t) => t,
    None => vec![],
  };

  let settings = ParserInputs {
    real_name_to_og_name: real_name_to_og_name,
    bytes: Arc::new(bytes),
    wanted_player_props: real_names.clone(),
    wanted_player_props_og_names: wanted_props.clone(),
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_events: vec![],
    parse_ents: true,
    wanted_ticks: wanted_ticks,
    parse_projectiles: false,
    only_header: false,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  real_names.push("tick".to_owned());
  real_names.push("steamid".to_owned());
  real_names.push("name".to_owned());

  let mut prop_infos = output.prop_info.prop_infos.clone();
  prop_infos.sort_by_key(|x| x.prop_name.clone());
  real_names.sort();

  let helper = OutputSerdeHelperStruct {
    prop_infos: prop_infos,
    inner: output.df.into(),
  };

  let is_soa = match struct_of_arrays {
    Some(true) => true,
    _ => false,
  };

  if is_soa {
    let s = match serde_json::to_value(&helper) {
      Ok(s) => s,
      Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
    };
    return Ok(s);
  } else {
    let result = soa_to_aos(helper);
    let s = match serde_json::to_value(&result) {
      Ok(s) => s,
      Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
    };
    Ok(s)
  }
}

#[napi]
pub fn parse_player_info(path_or_buf: Either<String, Buffer>) -> Result<Value> {
  let bytes = resolve_byte_type(path_or_buf);
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: Arc::new(bytes),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_events: vec![],
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let s = match serde_json::to_value(&output.player_md) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

use napi::Either;
fn resolve_byte_type(path_or_buf: Either<String, Buffer>) -> BytesVariant {
  match path_or_buf {
    Either::A(path) => {
      let file = File::open(path.clone()).unwrap();
      let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
      BytesVariant::Mmap(mmap)
    }
    Either::B(buf) => BytesVariant::Vec(buf.into()),
  }
}
