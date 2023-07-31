use std::collections::HashMap;

use archipelago_rs::client::{ArchipelagoClient, ArchipelagoError};

use archipelago_rs::protocol::{
    ClientMessage, Connected, GameData, GetDataPackage, LocationChecks, LocationScouts, NetworkItem, ServerMessage, ConnectUpdate,
};
use itertools::Itertools;
use num_traits::ToPrimitive;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

use crate::game::scripting::tsc::text_script::ScriptMode;
use crate::game::scripting::tsc::text_script::TextScript;
use crate::game::scripting::tsc::text_script::TextScriptEncoding;
use crate::game::shared_game_state::SharedGameState;

use super::stage_data::ALL_LOCATIONS;

pub enum ArchipelagoState {
    Inactive,
    Disconnected,
    Connected,
    Active,
}

pub struct Archipelago {
    pub state: ArchipelagoState,
    runtime: Runtime,
    inbox: Option<mpsc::Receiver<ServerMessage>>,
    outbox: Option<mpsc::Sender<ClientMessage>>,
    pub server: String,
    pub slot_name: String,
    pub password: String,
    pub data_package: Option<HashMap<String, GameData>>,
    pub loc_info: Option<Vec<NetworkItem>>,
    pub connection_info: Option<Connected>,
}

impl Archipelago {
    pub fn new() -> Archipelago {
        Self {
            state: ArchipelagoState::Inactive,
            runtime: Builder::new_multi_thread().worker_threads(1).enable_all().build().unwrap(),
            inbox: None,
            outbox: None,
            server: String::new(),
            slot_name: String::new(),
            password: String::new(),
            data_package: None,
            loc_info: None,
            connection_info: None,
        }
    }
    pub fn connect(&mut self) -> Result<(), ArchipelagoError> {
        let mut url = String::new();
        url.push_str("ws://");
        url.push_str(&self.server);
        let (in_send, in_recv) = mpsc::channel(16);
        let (out_send, mut out_recv) = mpsc::channel(16);
        self.inbox = Some(in_recv);
        self.outbox = Some(out_send);

        log::info!("Attempting to connect");

        match self.runtime.block_on(ArchipelagoClient::new(&url)) {
            Ok(mut client) => {
                let games = Some(client.room_info().games.clone());
                match self.runtime.block_on(client.connect(
                    "Cave Story",
                    &self.slot_name,
                    Some(&self.password),
                    Some(0x000),
                    vec!["AP".to_string()],
                )) {
                    Ok(connect) => {
                        self.connection_info = Some(connect);
                        log::info!("Slot Info: {:?}",self.connection_info.as_ref().unwrap().slot_info);
                        match client.split() {
                            (mut s, mut r) => {
                                let in_rt = Builder::new_current_thread().enable_all().build().unwrap();

                                std::thread::spawn(move || {
                                    in_rt.block_on(async move {
                                        while match r.recv().await {
                                            Ok(try_packet) => {
                                                match try_packet {
                                                    Some(packet) => {
                                                        log::info!("Packet received: {packet:?}");
                                                        let _ = in_send.send(packet).await;
                                                    }
                                                    None => {}
                                                }
                                                true
                                            }
                                            Err(e) => {
                                                let mut response = false;
                                                match e {
                                                    ArchipelagoError::NonTextWebsocketResult(m) => {
                                                        log::info!("NonText packet received: {m:?}");
                                                        response = true;
                                                    }
                                                    _ => {
                                                        log::warn!("Inbox Error: {e}");
                                                    }
                                                }
                                                response
                                            }
                                        } {}
                                        log::warn!("Inbox has been lost!");
                                    });
                                });
                                let out_rt = Builder::new_current_thread().enable_all().build().unwrap();

                                std::thread::spawn(move || {
                                    out_rt.block_on(async move {
                                        while let Some(packet) = out_recv.recv().await {
                                            log::info!("Sending packet: {packet:?}");
                                            let _ = s.send(packet).await;
                                        }
                                        log::warn!("Outbox has been lost!");
                                    });
                                });
                            }
                        }
                        self.send(ClientMessage::GetDataPackage(GetDataPackage { games }));
                        self.send(ClientMessage::LocationScouts(LocationScouts {
                            locations: ALL_LOCATIONS.to_vec(),
                            create_as_hint: 0,
                        }));
                    }
                    Err(e) => {
                        log::warn!("Failed to connect to slot: {}", e.to_string());
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to connect to server: {}", e.to_string());
                return Err(e);
            }
        }
        self.state = ArchipelagoState::Connected;
        log::info!("Archipelago successfully connected!");
        Ok(())
    }
    pub fn is_ready(&self) -> bool {
        return self.data_package.is_some() && self.loc_info.is_some();
    }
    pub fn send(&mut self, packet: ClientMessage) {
        match &self.outbox {
            Some(outbox) => _ = self.runtime.block_on(outbox.send(packet)),
            None => log::warn!("Client wants to send but outbox is closed!"),
        }
    }
    pub fn startup(&mut self) {
        self.send(ClientMessage::ConnectUpdate(ConnectUpdate {items_handling:0x101, tags:vec![]}));
        self.state = ArchipelagoState::Active;
    }
    pub fn shutdown(&mut self) {
        self.send(ClientMessage::ConnectUpdate(ConnectUpdate {items_handling:0x000, tags:vec![]}));
        self.inbox = None;
        self.outbox = None;
        self.state = ArchipelagoState::Inactive;
    }
    pub fn tick(state: &mut SharedGameState) {
        match state.archipelago.inbox.as_mut() {
            Some(receiver) => match receiver.try_recv() {
                Ok(message) => {
                    handle_packet(state, message);
                }
                Err(e) => match e {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        state.archipelago.inbox = None;
                        state.archipelago.outbox = None;
                        state.archipelago.state = ArchipelagoState::Disconnected;
                    }
                },
            },
            None => (),
        }
    }
}

fn handle_packet(state: &mut SharedGameState, message: ServerMessage) {
    match message {
        ServerMessage::DataPackage(data_package) => {
            state.archipelago.data_package = Some(data_package.data.games);
        }
        // Used during generation
        ServerMessage::LocationInfo(loc_info) => {
            state.archipelago.loc_info = Some(loc_info.locations);
        }
        // Used during the game
        ServerMessage::ReceivedItems(recv_items) => {
            for item in recv_items.items {
                let mut command = String::new();
                let cs_item = item.item - 0xD00000;
                let item_name = state
                    .archipelago
                    .data_package
                    .as_ref()
                    .unwrap()
                    .get("Cave Story")
                    .unwrap()
                    .item_name_to_id
                    .iter()
                    .find_map(|(key, &val)| if val == item.item { Some(key) } else { None })
                    .unwrap();
                let cn_info = state.archipelago.connection_info.as_ref().unwrap();
                let player_name = cn_info
                    .players
                    .iter()
                    .find_map(|player| {
                        if player.team == cn_info.team && player.slot == item.player {
                            Some(&player.name)
                        } else {
                            None
                        }
                    })
                    .unwrap();
                if cs_item >= 0 && cs_item <= 500 {
                    match cs_item / 100 {
                        0 => {
                            //Inventory Item
                            command = format!(
                                "#9990\n<MSG<GIT10{:#02}<IT+00{:#02}Received {} from {}<WAI0160",
                                cs_item, cs_item, item_name, player_name
                            )
                        }
                        1 => {
                            //Weapon
                            if cs_item == 105 {
                                command =
                                    format!("#9990\n<MSGReceived {} from {}<WAI0080<EVE0030", item_name, player_name)
                            } else {
                                let ammo = match cs_item {
                                    104 => "0100",
                                    107 => "0100",
                                    _ => "0000",
                                };
                                command = format!(
                                    "#9990\n<MSG<GIT00{:#02}<AM+00{:#02}:{:#04}Received {} from {}<WAI0160",
                                    cs_item % 100,
                                    cs_item % 100,
                                    ammo,
                                    item_name,
                                    player_name
                                )
                            }
                        }
                        2 => {
                            //Health
                            command = format!(
                                "#9990\n<SOU0022<MSG<GIT1006<ML+000{}Received {} from {}<WAI0160",
                                cs_item % 200,
                                item_name,
                                player_name,
                            )
                        }
                        3 => {
                            command = format!("#9990\n<MSGReceived {} from {}<WAI0080<EVE0030", item_name, player_name)
                        }
                        4 => {
                            //Pickup
                        }
                        _ => (),
                    }
                } else {
                    // Other archipelago item
                }
                // For testing, any item gives you a life capsule
                match TextScript::compile(command.as_bytes(), true, TextScriptEncoding::UTF8) {
                    Ok(text_script) => {
                        state.textscript_vm.set_debug_script(text_script);
                        state.textscript_vm.set_mode(ScriptMode::Debug);
                        state.textscript_vm.start_script(9990);
                    }
                    Err(err) => {
                        log::warn!("Error occured during live TSC complilation: {}", err);
                    }
                };
            }
        }
        ServerMessage::Bounced(_bounced) => {
            // TODO: deathlink
        }
        // Everything else not implemented is printed to the console
        _ => {
            log::info!("Ignoring packet {message:?} from server")
        }
    }
}

pub fn handle_flag(state: &mut SharedGameState, id: usize, value: bool) {
    log::info!("Processing flag {} ({})", id, value);
    match &state.archipelago.data_package {
        Some(data_package) => {
            let id_int = (id + 0xD00000).to_i32().unwrap();
            if data_package.get("Cave Story").unwrap().location_name_to_id.values().contains(&id_int) {
                let locations = vec![id_int];
                _ = state.archipelago.send(ClientMessage::LocationChecks(LocationChecks { locations }));
            }
        }
        None => log::warn!("Client wants to send but data package is missing!"),
    }
}
