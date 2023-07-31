use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;

use archipelago_rs::protocol::NetworkItem;
use itertools::Itertools;
use num_traits::ToPrimitive;

use crate::archipelago::stage_data::{generate_tsc, Container};
use crate::framework::context::Context;
use crate::framework::filesystem;
use crate::game::map::NPCData;
use crate::game::scripting::tsc::encryption::{decrypt_tsc, encrypt_tsc};
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
        let mut buf = Vec::<u8>::new();
        load_vanilla_text_script(&mut buf, stage.map.to_string(), &state.constants.base_paths, ctx);
        let mut tsc_events: Vec<Vec<u8>> = Vec::new();
        for (_, group) in &buf.into_iter().group_by(|e| *e == b'#') {
            tsc_events.push(group.collect::<Vec<u8>>());
        }
        //let tsc_events = buf.split(|slice| *slice == b'#').collect::<Vec<&[u8]>>();
        let mut modded_npcs = Vec::<NPCData>::new();
        for mut npc in load_vanilla_npcs(stage.map.to_string(), &state.constants.base_paths, ctx) {
            if npc.npc_type == 21 {
                continue;
            }
            match flag_to_item.get(&npc.flag_num) {
                Some(item) => {
                    let player = cn_info
                        .players
                        .iter()
                        .find_map(|p| if p.team == cn_info.team && p.slot == item.player { Some(p) } else { None })
                        .unwrap();
                    let player_name = if cn_info.slot == item.player { None } else { Some(player.name.clone()) };
                    let item_name = data_package
                        .get(&cn_info.slot_info.get(&item.player.to_string()).unwrap().game)
                        .unwrap()
                        .item_name_to_id
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
                    //log::info!("Injected Event: {}", from_utf8(injected_tsc_event.as_slice()).unwrap());
                    let mut target_idx: Option<usize> = None;
                    for i in 0..tsc_events.len() {
                        match tsc_events[i].get_mut(0..4) {
                            Some(id) => {
                                if id == format!("{:#04}",npc.event_num).as_bytes() {
                                    target_idx = Some(i);
                                    break
                                }
                            }
                            None => ()
                        }
                    }
                    match target_idx {
                        Some(i) => {
                            log::info!("Event {:#04} has been injected!", npc.event_num);
                            tsc_events[i] = injected_tsc_event.to_vec();
                        }
                        None => {
                            log::warn!("Event {:#04} could not be injected!", npc.event_num);
                        }
                    }
                }
                None => (),
            }
            modded_npcs.push(npc);
        }
        let mut tsc = tsc_events.concat();
        save_modded_pxx(&mut tsc, &modded_npcs, &stage.map, ctx, state.save_slot);
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

fn save_modded_pxx(tsc: &mut Vec<u8>, npcs: &Vec<NPCData>, stage: &String, ctx: &mut Context, save: usize) {
    let stage_path: PathBuf = ["/StageAP", &save.to_string(), "/"].join("").into();
    let mut tsc_path: PathBuf = stage_path.clone();
    tsc_path.push(format!("{}.tsc", stage));
    let mut pxe_path: PathBuf = stage_path.clone();
    pxe_path.push(format!("{}.pxe", stage));

    if filesystem::create_dir(ctx, stage_path).is_err() {
        log::warn!("Failed to create StageAP directory structure.");
        return;
    }

    let mut tsc_file = match filesystem::create(ctx, tsc_path) {
        Ok(file) => file,
        Err(_) => {
            log::warn!("Failed to create tsc file.");
            return;
        }
    };

    let pxe_file = match filesystem::create(ctx, pxe_path) {
        Ok(file) => file,
        Err(_) => {
            log::warn!("Failed to create pxe file.");
            return;
        }
    };

    encrypt_tsc(tsc);
    if tsc_file.write_all(tsc).is_err() {
        log::warn!("Failed to write to tsc file.");
        return;
    }

    if NPCData::save_to(pxe_file, npcs).is_err() {
        log::warn!("Failed to write to pxe file.");
        return;
    }
}
