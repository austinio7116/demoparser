use super::read_bits::{Bitreader, DemoParserError};
use crate::{parser_settings::Parser, parser_thread_settings::ParserThread};
use csgoproto::{
    netmessages::{CSVCMsg_CreateStringTable, CSVCMsg_UpdateStringTable},
    networkbasetypes::CMsgPlayerInfo,
};
use protobuf::Message;
use snap::raw::Decoder;

#[derive(Clone, Debug)]
pub struct StringTable {
    name: String,
    user_data_size: i32,
    user_data_fixed: bool,
    #[allow(dead_code)]
    data: Vec<StringTableEntry>,
    flags: i32,
    var_bit_counts: bool,
}
#[derive(Clone, Debug)]
pub struct StringTableEntry {
    pub idx: i32,
    pub key: String,
    pub value: Vec<u8>,
}
#[derive(Clone, Debug)]
pub struct UserInfo {
    pub steamid: u64,
    pub name: String,
    pub userid: i32,
    pub is_hltv: bool,
}

impl Parser {
    pub fn update_string_table(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table: CSVCMsg_UpdateStringTable = Message::parse_from_bytes(&bytes).unwrap();
        match self.string_tables.get(table.table_id() as usize) {
            Some(st) => self.parse_string_table(
                table.string_data().to_vec(),
                table.num_changed_entries(),
                st.name.clone(),
                st.user_data_fixed,
                st.user_data_size,
                st.flags,
                st.var_bit_counts,
            )?,
            None => return Err(DemoParserError::StringTableNotFound),
        }
        Ok(())
    }

    pub fn parse_create_stringtable(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table: CSVCMsg_CreateStringTable = Message::parse_from_bytes(&bytes).unwrap();

        if !(table.name() == "instancebaseline" || table.name() == "userinfo") {
            return Ok(());
        }

        let bytes = match table.data_compressed() {
            true => snap::raw::Decoder::new().decompress_vec(table.string_data()).unwrap(),
            false => table.string_data().to_vec(),
        };
        self.parse_string_table(
            bytes,
            table.num_entries(),
            table.name().to_string(),
            table.user_data_fixed_size(),
            table.user_data_size(),
            table.flags(),
            table.using_varint_bitcounts(),
        )?;
        Ok(())
    }
    pub fn parse_string_table(
        &mut self,
        bytes: Vec<u8>,
        n_updates: i32,
        name: String,
        udf: bool,
        user_data_size: i32,
        flags: i32,
        variant_bit_count: bool,
    ) -> Result<(), DemoParserError> {
        let mut bitreader = Bitreader::new(&bytes);
        let mut idx = -1;
        let mut keys: Vec<String> = vec![];
        let mut items = vec![];

        for _upd in 0..n_updates {
            let mut key = "".to_owned();
            let mut value = vec![];

            // Increment index
            match bitreader.read_boolean()? {
                true => idx += 1,
                false => idx += (bitreader.read_varint()? + 1) as i32,
            };
            // Does the value have a key
            if bitreader.read_boolean()? {
                // Should we refer back to history (similar to LZ77)
                match bitreader.read_boolean()? {
                    // If no history then just read the data as one string
                    false => key = key.to_owned() + &bitreader.read_string()?,
                    // Refer to history
                    true => {
                        // How far into history we should look
                        let position = bitreader.read_nbits(5)?;
                        // How many bytes in a row, starting from distance ago, should be copied
                        let length = bitreader.read_nbits(5)?;

                        if position >= keys.len() as u32 {
                            key = key.to_owned() + &bitreader.read_string()?;
                        } else {
                            let s = &keys[position as usize];
                            if length > s.len() as u32 {
                                key = key.to_owned() + &s + &bitreader.read_string()?;
                            } else {
                                key = key.to_owned() + &s[0..length as usize] + &bitreader.read_string()?;
                            }
                        }
                    }
                }
                if keys.len() >= 32 {
                    keys.remove(0);
                }
                keys.push(key.clone());
                // Does the entry have a value
                if bitreader.read_boolean()? {
                    let bits: u32;
                    let mut is_compressed = false;

                    match udf {
                        true => bits = user_data_size as u32,
                        false => {
                            if (flags & 0x1) != 0 {
                                is_compressed = bitreader.read_boolean()?;
                            }
                            if variant_bit_count {
                                bits = bitreader.read_u_bit_var()? * 8;
                            } else {
                                bits = bitreader.read_nbits(17)? * 8;
                            }
                        }
                    }
                    value = bitreader.read_n_bytes((bits / 8) as usize)?;
                    value = if is_compressed {
                        Decoder::new().decompress_vec(&value).unwrap()
                    } else {
                        value
                    };
                }
                if name == "userinfo" {
                    if let Ok(player) = parse_userinfo(&value) {
                        self.stringtable_players.insert(player.steamid, player);
                    }
                }
                if name == "instancebaseline" {
                    match key.parse::<u32>() {
                        Ok(cls_id) => self.baselines.insert(cls_id, value.clone()),
                        Err(_e) => None,
                    };
                }
                items.push(StringTableEntry {
                    idx: idx,
                    key: key,
                    value: value,
                });
            }
        }
        self.string_tables.push(StringTable {
            data: items,
            name: name,
            user_data_size: user_data_size,
            user_data_fixed: udf,
            flags: flags,
            var_bit_counts: variant_bit_count,
        });
        Ok(())
    }
}
fn parse_userinfo(bytes: &[u8]) -> Result<UserInfo, DemoParserError> {
    let player = match CMsgPlayerInfo::parse_from_bytes(bytes) {
        Err(_e) => return Err(DemoParserError::MalformedMessage),
        Ok(player) => player,
    };
    Ok(UserInfo {
        is_hltv: player.ishltv(),
        steamid: player.xuid(),
        name: player.name().to_string(),
        userid: player.userid(),
    })
}

impl ParserThread {
    pub fn update_string_table(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table: CSVCMsg_UpdateStringTable = Message::parse_from_bytes(&bytes).unwrap();
        match self.string_tables.get(table.table_id() as usize) {
            Some(st) => self.parse_string_table(
                table.string_data().to_vec(),
                table.num_changed_entries(),
                st.name.clone(),
                st.user_data_fixed,
                st.user_data_size,
                st.flags,
                st.var_bit_counts,
            )?,
            None => return Ok(()),
        }
        Ok(())
    }

    pub fn parse_create_stringtable(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let table: CSVCMsg_CreateStringTable = Message::parse_from_bytes(&bytes).unwrap();
        let bytes = match table.data_compressed() {
            true => snap::raw::Decoder::new().decompress_vec(table.string_data()).unwrap(),
            false => table.string_data().to_vec(),
        };
        self.parse_string_table(
            bytes,
            table.num_entries(),
            table.name().to_string(),
            table.user_data_fixed_size(),
            table.user_data_size(),
            table.flags(),
            table.using_varint_bitcounts(),
        )?;
        Ok(())
    }
    pub fn parse_string_table(
        &mut self,
        bytes: Vec<u8>,
        n_updates: i32,
        name: String,
        udf: bool,
        user_data_size: i32,
        flags: i32,
        variant_bit_count: bool,
    ) -> Result<(), DemoParserError> {
        let mut bitreader = Bitreader::new(&bytes);
        let mut idx = -1;
        let mut keys: Vec<String> = vec![];
        let mut items = vec![];

        for _upd in 0..n_updates {
            let mut key = "".to_owned();
            let mut value = vec![];

            // Increment index
            match bitreader.read_boolean()? {
                true => idx += 1,
                false => idx += (bitreader.read_varint()? + 1) as i32,
            };
            // Does the value have a key
            if bitreader.read_boolean()? {
                // Should we refer back to history (similar to LZ77)
                match bitreader.read_boolean()? {
                    // If no history then just read the data as one string
                    false => key = key.to_owned() + &bitreader.read_string()?,
                    // Refer to history
                    true => {
                        // How far into history we should look
                        let position = bitreader.read_nbits(5)?;
                        // How many bytes in a row, starting from distance ago, should be copied
                        let length = bitreader.read_nbits(5)?;

                        if position >= keys.len() as u32 {
                            key = key.to_owned() + &bitreader.read_string()?;
                        } else {
                            let s = &keys[position as usize];
                            if length > s.len() as u32 {
                                key = key.to_owned() + &s + &bitreader.read_string()?;
                            } else {
                                key = key.to_owned() + &s[0..length as usize] + &bitreader.read_string()?;
                            }
                        }
                    }
                }
                if keys.len() >= 32 {
                    keys.remove(0);
                }
                keys.push(key.clone());
                // Does the entry have a value
                if bitreader.read_boolean()? {
                    let bits: u32;
                    let mut is_compressed = false;

                    match udf {
                        true => bits = user_data_size as u32,
                        false => {
                            if (flags & 0x1) != 0 {
                                is_compressed = bitreader.read_boolean()?;
                            }
                            if variant_bit_count {
                                bits = bitreader.read_u_bit_var()? * 8;
                            } else {
                                bits = bitreader.read_nbits(17)? * 8;
                            }
                        }
                    }
                    value = bitreader.read_n_bytes((bits / 8) as usize)?;
                    value = if is_compressed {
                        Decoder::new().decompress_vec(&value).unwrap()
                    } else {
                        value
                    };
                }
                if name == "userinfo" {
                    if let Ok(player) = parse_userinfo(&value) {
                        self.stringtable_players.insert(player.steamid, player);
                    }
                }
                if name == "instancebaseline" {
                    match key.parse::<u32>() {
                        Ok(cls_id) => self.baselines.insert(cls_id, value.clone()),
                        Err(_e) => None,
                    };
                }
                items.push(StringTableEntry {
                    idx: idx,
                    key: key,
                    value: value,
                });
            }
        }
        self.string_tables.push(StringTable {
            data: items,
            name: name,
            user_data_size: user_data_size,
            user_data_fixed: udf,
            flags: flags,
            var_bit_counts: variant_bit_count,
        });
        Ok(())
    }
}
