use ahash::AHashMap;
use arrow2::array::*;
use arrow2::datatypes::DataType;
use arrow2::datatypes::Field;
use arrow2::datatypes::PhysicalType;
use arrow2::offset::Offsets;
use arrow2::offset::OffsetsBuffer;
use arrow_data::{ArrayData, ArrayDataBuilder};
use memmap2::MmapOptions;
use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::parser_thread_settings::create_huffman_lookup_table;
use parser::variants::BytesVariant;
use std::fs;
use std::fs::File;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let wanted_props = vec!["inventory".to_string()];
    let before = Instant::now();
    let dir = fs::read_dir("/home/laiho/Documents/demos/cs2/test3/").unwrap();
    let mut c = 0;
    let huf = create_huffman_lookup_table();

    for path in dir {
        c += 1;

        let before = Instant::now();

        if c > 100 {
            break;
        }

        let file = File::open(path.unwrap().path()).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        mmap.advise(memmap2::Advice::HugePage).unwrap();

        let settings = ParserInputs {
            real_name_to_og_name: AHashMap::default(),
            bytes: Arc::new(BytesVariant::Mmap(mmap)),
            wanted_player_props: wanted_props.clone(),
            wanted_player_props_og_names: wanted_props.clone(),
            wanted_events: vec!["player_blind".to_string()],
            //wanted_event: None,
            wanted_other_props: vec![
                "CCSTeam.m_iScore".to_string(),
                "CCSTeam.m_szTeamname".to_string(),
                "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
            ],
            wanted_other_props_og_names: vec![
                "score".to_string(),
                "name".to_string(),
                "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
            ],
            parse_ents: true,
            wanted_ticks: vec![],
            parse_projectiles: false,
            only_header: false,
            count_props: false,
            only_convars: false,
            huffman_lookup_table: Arc::new(huf.clone()),
        };

        let mut ds = Parser::new(settings);
        let d = ds.parse_demo().unwrap();
        println!("TOTAL {:?}", before.elapsed());
    }
    println!("TOTAL {:?}", before.elapsed());
}
