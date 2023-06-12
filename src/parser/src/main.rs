use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::read_bits::Bitreader;
use std::fs;
use std::time::Instant;

fn main() {
    let wanted_props = vec!["CCSPlayerPawn.m_bSpotted".to_owned()];
    let demo_path = "/home/laiho/Documents/demos/cs2/s2.dem";
    let bytes = fs::read(demo_path).unwrap();

    let settings = ParserInputs {
        bytes: &bytes,
        wanted_player_props: wanted_props.clone(),
        wanted_player_props_og_names: wanted_props.clone(),
        wanted_event: Some("bomb_planted".to_string()),
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
    };
    let before = Instant::now();
    let mut parser = Parser::new(settings).unwrap();
    parser.start().unwrap();
    println!("{:2?}", before.elapsed());
    println!("{}", parser.qf_map.idx);
    // println!("{:?}", parser.qf_map.map);

    // IDS NEED TO BE UNIQ FOR PLAYER
    // println!("{:?}", parser.player_output_ids);
    // let x = &parser.output["CCSPlayerController.m_iPawnHealth"];
}
