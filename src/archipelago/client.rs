use archipelago_rs::client::{ArchipelagoClient, ArchipelagoError};

use archipelago_rs::protocol::{ServerMessage, ClientMessage, RoomInfo, DataPackageObject};
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::runtime::Runtime;

use crate::game::scripting::tsc::text_script::ScriptMode;
use crate::game::scripting::tsc::text_script::TextScript;
use crate::game::scripting::tsc::text_script::TextScriptEncoding;
use crate::game::shared_game_state::SharedGameState;

pub enum ArchipelagoState {
    Disconnected,
    Connected,
}

pub struct Archipelago {
    state: ArchipelagoState,
    runtime: Runtime,
    inbox: Option<mpsc::Receiver<ServerMessage>>,
    outbox: Option<mpsc::Sender<ClientMessage>>,
    pub server: String,
    pub slot_name: String,
    pub password: String,
    room_info: Option<RoomInfo>,
    data_package: Option<DataPackageObject>,
}

impl Archipelago {
    pub fn new() -> Archipelago {
        Self {
            state: ArchipelagoState::Disconnected,
            runtime: Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap(),
            inbox: None,
            outbox: None,
            server: String::new(),
            slot_name: String::new(),
            password: String::new(),
            room_info: None,
            data_package: None
        }
    }
    pub fn connect(
        &mut self,
    ) -> Result<(),ArchipelagoError> {
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
                match self.runtime.block_on(client
                    .connect("Cave Story", &self.slot_name, Some(&self.password), Some(7), vec!["AP".to_string()])
                ){
                    Ok(_packet) => {
                        match client.split() {
                            (mut s, mut r) => {
                                let in_rt = Builder::new_current_thread()
                                    .enable_all()
                                    .build()
                                    .unwrap();

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
                                            },
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
                                        } {};
                                        log::warn!("Inbox has been lost!");
                                    });
                                });
                                let out_rt = Builder::new_current_thread()
                                    .enable_all()
                                    .build()
                                    .unwrap();

                                std::thread::spawn(move || {
                                    out_rt.block_on(async move {
                                        while let Some(packet) = out_recv.recv().await  {
                                            log::info!("Sending packet: {packet:?}");
                                            let _ = s.send(packet).await;
                                        }
                                        log::warn!("Outbox has been lost!");
                                    });
                                });
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to connect: {}", e.to_string());
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to connect: {}", e.to_string());
                return Err(e);
            }
        }
        self.state = ArchipelagoState::Connected;
        log::info!("Archipelago successfully connected!");
        Ok(())
    }
    pub fn location_checks(
        &mut self,
    ) -> Result<(),ArchipelagoError> {
        // send vector of location ids
        Ok(())
    }
    pub fn location_scouts(
        &mut self,
    ) -> Result<(),ArchipelagoError> {
        // create as hint is false for generation
        Ok(())
    }
    pub fn status_update(
        &mut self,
    ) -> Result<(),ArchipelagoError> {
        // unknown, ready, playing, goal
        Ok(())
    }
    pub fn get_data_package(
        &mut self,
    ) -> Result<(),ArchipelagoError> {

        Ok(())
    }
    pub fn bounce(
        &mut self,
    ) -> Result<(),ArchipelagoError> {
        Ok(())
    }
    pub fn tick(
        state: &mut SharedGameState
    ) {
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
                }
            }
            None => ()
        }
    }
}

fn handle_packet(
    state: &mut SharedGameState,
    message: ServerMessage
) {
    match message {
        // Used at the start of connection
        ServerMessage::RoomInfo(room_info) => {
            state.archipelago.room_info = Some(room_info);
        }
        ServerMessage::DataPackage(data_package) => {
            state.archipelago.data_package= Some(data_package.data);
        }
        // Used during generation
        ServerMessage::LocationInfo(_loc_info) => {

        }
        // Used during the game
        ServerMessage::ReceivedItems(recv_items) => {
            for _item in recv_items.items {
                // For testing, any item gives you a life capsule
                match TextScript::compile(format!("#9999\n<SOU0022<MSG<GIT1006Got Life Capsule.<WAI0160<ML+0003").as_bytes(), true, TextScriptEncoding::UTF8) {
                    Ok(text_script) => {
                        state.textscript_vm.set_debug_script(text_script);
                        state.textscript_vm.set_mode(ScriptMode::Debug);
                        state.textscript_vm.start_script(9999);
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

pub fn handle_flag(
    _state: &mut SharedGameState,
    id: usize,
    value: bool
) {
    log::warn!("Processing flag {} ({})", id, value);
}
