use std::collections::HashMap;
use std::io::Read;

use archipelago_rs::protocol::NetworkItem;
use num_traits::ToPrimitive;

use crate::archipelago::stage_data::{generate_tsc, Container};
use crate::framework::context::Context;
use crate::framework::filesystem;
use crate::game::map::NPCData;
use crate::game::scripting::tsc::encryption::decrypt_tsc;
use crate::game::shared_game_state::SharedGameState;

pub fn generate_world(state: &mut SharedGameState, ctx: &mut Context) {
    let data_package = state.archipelago.data_package.as_ref().unwrap();
    let cn_info = state.archipelago.connection_info.as_ref().unwrap();
    let loc_data = state.archipelago.loc_info.as_ref().unwrap();
    let mut flag_to_item = HashMap::<u16, &NetworkItem>::new();
    for item in loc_data {
        flag_to_item.insert((item.location - 0xD00000).to_u16().unwrap(), item);
    }
    for stage in &state.stages {
        log::info!("Stage: {}", stage.map.to_string());
        let buf = &mut Vec::<u8>::new();
        load_vanilla_text_script(buf, stage.map.to_string(), &state.constants.base_paths, ctx);
        let mut tsc_events = buf.split(|slice| *slice == b'#').collect::<Vec<&[u8]>>();
        for mut npc in load_vanilla_npcs(stage.map.to_string(), &state.constants.base_paths, ctx) {
            match flag_to_item.get(&npc.flag_num) {
                Some(item) => {
                    let player = cn_info
                        .players
                        .iter()
                        .find_map(|p| if p.team == cn_info.team && p.slot == item.player { Some(p) } else { None })
                        .unwrap();
                    let player_name = if cn_info.slot == item.player { None } else { Some(player.name.clone()) };
                    let item_name = data_package
                        .get(&cn_info.slot_info.get(&player.slot).unwrap().game)
                        .unwrap()
                        .location_name_to_id
                        .iter()
                        .find_map(|(key, &val)| if val == item.item { Some(key.clone()) } else { None })
                        .unwrap_or("Unknown Item".to_string());
                    npc.npc_type = 15; //For testing, everything is a chest
                    let injected_tsc_event = &mut Vec::<u8>::new();
                    generate_tsc(
                        injected_tsc_event,
                        npc.flag_num,
                        npc.event_num,
                        Container::Chest,
                        item.item,
                        item_name,
                        player_name,
                    );
                }
                None => (),
            }
        }
    }
}

fn load_vanilla_npcs(stage: String, roots: &Vec<String>, ctx: &mut Context) -> Vec<NPCData> {
    let pxe_file = filesystem::open_find(ctx, roots, ["Stage/", &stage, ".pxe"].join("")).unwrap();
    let npc_data = NPCData::load_from(pxe_file).unwrap();

    npc_data
}

fn load_vanilla_text_script(buf: &mut Vec<u8>, stage: String, roots: &Vec<String>, ctx: &mut Context) {
    let mut tsc_file = filesystem::open_find(ctx, roots, ["Stage/", &stage, ".tsc"].join("")).unwrap();
    let _ = tsc_file.read_to_end(buf);
    decrypt_tsc(buf);
}
