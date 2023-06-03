use ahash::HashMap;
use serde::{ser::SerializeMap, Serialize};

use crate::game_events::EventField;

#[derive(Debug, Clone)]
pub enum Variant {
    Bool(bool),
    U32(u32),
    I32(i32),
    I16(i16),
    F32(f32),
    U64(u64),
    U8(u8),
    String(String),
    VecXY([f32; 2]),
    VecXYZ([f32; 3]),
    Vec(Vec<i32>),
    FloatVec32(Vec<f32>),
}
/*
Str(String),
Float(f32),
Long(i32),
Short(i16),
Byte(u8),
Bool(bool),
Uint64(u64),
I32(i32),
*/

#[derive(Debug, Clone, Serialize)]
pub enum VarVec {
    U32(Vec<Option<u32>>),
    Bool(Vec<Option<bool>>),
    U64(Vec<Option<u64>>),
    F32(Vec<Option<f32>>),
    I32(Vec<Option<i32>>),
    String(Vec<Option<String>>),
}

impl VarVec {
    pub fn new(item: &Variant) -> Self {
        match item {
            Variant::Bool(_) => VarVec::Bool(vec![]),
            Variant::I32(_) => VarVec::I32(vec![]),
            Variant::F32(_) => VarVec::F32(vec![]),
            Variant::String(_) => VarVec::String(vec![]),
            Variant::U64(_) => VarVec::U64(vec![]),
            Variant::U32(_) => VarVec::U32(vec![]),
            _ => panic!("Tried to create propcolumns from: {:?}", item),
        }
    }
}
impl Serialize for Variant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Variant::Bool(b) => serializer.serialize_bool(*b),
            Variant::F32(f) => serializer.serialize_f32(*f),
            Variant::I16(i) => serializer.serialize_i16(*i),
            Variant::I32(i) => serializer.serialize_i32(*i),
            Variant::String(s) => serializer.serialize_str(s),
            Variant::U32(u) => serializer.serialize_u32(*u),
            Variant::U64(u) => serializer.serialize_u64(*u),
            Variant::U8(u) => serializer.serialize_u8(*u),
            _ => panic!("cant ser: {:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputSerdeHelperStruct {
    pub inner: HashMap<String, PropColumn>,
}

impl Serialize for OutputSerdeHelperStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;

        for (k, v) in &self.inner {
            match &v.data {
                Some(VarVec::F32(val)) => {
                    map.serialize_entry(&k, val).unwrap();
                }
                Some(VarVec::I32(val)) => {
                    map.serialize_entry(&k, val).unwrap();
                }
                Some(VarVec::String(val)) => {
                    map.serialize_entry(&k, val).unwrap();
                }
                Some(VarVec::U64(val)) => {
                    map.serialize_entry(&k, val).unwrap();
                }
                Some(VarVec::Bool(val)) => {
                    map.serialize_entry(&k, val).unwrap();
                }
                Some(VarVec::U32(val)) => {
                    map.serialize_entry(&k, val).unwrap();
                }
                None => {}
            };
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct PropColumn {
    pub data: Option<VarVec>,
    num_nones: usize,
}
/*
pub fn to_serdeable(hm: &HashMap<String, PropColumn>) {
    for (k, v) in hm {
        match v {
            VarVec::F32(val) => {

            }
            VarVec::I32(val) => {

            }
            VarVec::String(val) => {

            }
            VarVec::U64(val) => {

            }
            VarVec::Bool(val) => {

            }
            VarVec::U32(val) => {

            }
        }
    }
}
*/

impl PropColumn {
    pub fn new() -> Self {
        PropColumn {
            data: None,
            num_nones: 0,
        }
    }
    #[inline(always)]
    pub fn push(&mut self, item: Option<Variant>) {
        match &item {
            // If we dont know what type the column is (prop has not been parsed yet)
            None => self.num_nones += 1,
            Some(p) => match &self.data {
                Some(_) => {}
                None => {
                    // First time a new prop is pushed we get the type for the vec and
                    // push the leading Nones we may have gotten before prop type was known.
                    let mut var_vec = VarVec::new(&p);
                    for _ in 0..self.num_nones {
                        var_vec.push_Variant(None);
                    }
                    self.data = Some(var_vec);
                }
            },
        }
        if let Some(v) = &mut self.data {
            v.push_Variant(item.clone());
        }
    }
}

impl VarVec {
    #[inline(always)]
    pub fn push_Variant(&mut self, item: Option<Variant>) {
        match item {
            Some(Variant::F32(p)) => match self {
                VarVec::F32(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(Variant::I32(p)) => match self {
                VarVec::I32(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(Variant::String(p)) => match self {
                VarVec::String(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a ? into a {:?} column", self);
                }
            },
            Some(Variant::U32(p)) => match self {
                VarVec::U32(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(Variant::U64(p)) => match self {
                VarVec::U64(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            Some(Variant::Bool(p)) => match self {
                VarVec::Bool(f) => f.push(Some(p)),
                _ => {
                    panic!("Tried to push a {:?} into a {:?} column", item, self);
                }
            },
            None => self.push_none(),
            _ => panic!("bad type for prop: {:?}", item),
        }
    }
    pub fn push_none(&mut self) {
        match self {
            VarVec::I32(f) => f.push(None),
            VarVec::F32(f) => f.push(None),
            VarVec::String(f) => f.push(None),
            VarVec::U32(f) => f.push(None),
            VarVec::U64(f) => f.push(None),
            VarVec::Bool(f) => f.push(None),
        }
    }
}
#[allow(dead_code)]
pub fn filter_to_vec<Wanted>(v: impl IntoIterator<Item = impl TryInto<Wanted>>) -> Vec<Wanted> {
    v.into_iter().filter_map(|x| x.try_into().ok()).collect()
}

pub fn eventdata_type_from_Variant(value: &Option<Variant>) -> i32 {
    match value {
        Some(Variant::String(_)) => 1,
        Some(Variant::F32(_)) => 2,
        Some(Variant::U32(_)) => 7,
        Some(Variant::I32(_)) => 4,
        Some(Variant::Bool(_)) => 6,
        None => 99,
        _ => panic!("Could not convert: {:?} into type", value),
    }
}
