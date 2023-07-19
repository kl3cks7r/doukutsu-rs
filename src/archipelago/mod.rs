use archipelago_rs::client::ArchipelagoClientSender;
use archipelago_rs::client::{ArchipelagoClient, ArchipelagoError};

use archipelago_rs::protocol::{ServerMessage, ClientMessage};
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::runtime::Runtime;

use crate::game::scripting::tsc::text_script::ScriptMode;
use crate::game::scripting::tsc::text_script::TextScript;
use crate::game::scripting::tsc::text_script::TextScriptEncoding;
use crate::game::shared_game_state::SharedGameState;

// pub enum ArchipelagoState {
//     Disconnected,
//     Connected,
// }

pub struct Archipelago {
    // state: ArchipelagoState,
    runtime: Runtime,
    inbox: Option<mpsc::Receiver<ServerMessage>>,
    outbox: Option<mpsc::Sender<ClientMessage>>,
    pub server: String,
    pub slot_name: String,
    pub password: String,
}

impl Archipelago {
    pub fn new() -> Archipelago {
        Self {
            // state: ArchipelagoState::Disconnected,
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
                    .connect("VVVVVV", &self.slot_name, Some(&self.password), Some(7), vec!["AP".to_string()])
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
                                                        log::info!("Got packet: {packet:?}");
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
                                            send_packet(&mut s, packet).await;
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
    pub fn process_flag(
        &mut self,
        id: usize,
        value: bool
    ) {
        log::warn!("Processing flag {} ({})", id, value);
    }
    pub fn tick(
        state: &mut SharedGameState
    ) {
        // match TextScript::compile(format!("#9999\n<SOU0022<MSG<GIT1006Got Life Capsule.<WAI0160<ML+0003").as_bytes(), true, TextScriptEncoding::UTF8) {
        //     Ok(text_script) => {
        //         state.textscript_vm.set_debug_script(text_script);
        //         state.textscript_vm.set_mode(ScriptMode::Debug);
        //         state.textscript_vm.start_script(9999);
        //     }
        //     Err(err) => {
        //         log::warn!("Error occured during live TSC complilation: {}", err);
        //     }
        // };
    }
}

async fn send_packet(
    sender: &mut ArchipelagoClientSender,
    packet: ClientMessage
) {
    println!("Sending packet {packet:?}");
}