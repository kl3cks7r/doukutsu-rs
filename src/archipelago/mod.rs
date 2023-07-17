use archipelago_rs::client::{ArchipelagoClient, ArchipelagoError};

use tokio::runtime::Builder;
use std::sync::Mutex;
use std::sync::Arc;
use tokio::task::JoinHandle as TokioJoinHandle;
use tokio::runtime::Runtime;

// pub enum ArchipelagoState {
//     Disconnected,
//     Connected,
// }

pub struct Archipelago {
    // state: ArchipelagoState,
    runtime: Runtime,
    client: Arc<Mutex<Option<ArchipelagoClient>>>,
    handles: Vec<TokioJoinHandle<Result<(),ArchipelagoError>>>,
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
            client: Arc::new(Mutex::new(None)),
            handles: Vec::with_capacity(10),
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

        log::info!("Attempting to connect");
        
        let mut client_lock = self.client.lock().unwrap();
        match self.runtime.block_on(ArchipelagoClient::new(&url)) {
            Ok(client) => {
                *client_lock = Some(client);
                log::info!("Archipelago successfully connected!");
            }
            Err(e) => {
                log::info!("Failed to connect: {}", e.to_string());
                return Err(e);
            }
        }

        match self.runtime.block_on(client_lock
            .as_mut()
            .unwrap()
            .connect("VVVVVV", &self.slot_name, Some(&self.password), Some(7), vec!["AP".to_string()])
        ){
            Ok(_) => {
                log::info!("Archipelago successfully connected!");
            }
            Err(e) => {
                log::info!("Failed to connect: {}", e.to_string());
                return Err(e);
            }
        }
        Ok(())
    }
    // pub fn action(
    //     &mut self,
    //     action: ArchipelagoAction,
    //     args: Option<Vec<String>>,
    // ) -> Result<(),ArchipelagoError> {
    //     match action {
    //         ArchipelagoAction::Connect => {
    //             let mut url = String::new();
    //             url.push_str("ws://");
    //             url.push_str(&self.server);

    //             log::info!("Attempting to connect");
                
    //             let mut client_lock = self.client.lock().unwrap();
    //             match self.runtime.block_on(ArchipelagoClient::new(&url)) {
    //                 Ok(client) => {
    //                     *client_lock = Some(client);
    //                     log::info!("Archipelago successfully connected!");
    //                 }
    //                 Err(e) => {
    //                     log::info!("Failed to connect: {}", e.to_string());
    //                     return Err(e);
    //                 }
    //             }

    //             match self.runtime.block_on(client_lock
    //                 .as_mut()
    //                 .unwrap()
    //                 .connect("VVVVVV", &self.slot_name, Some(&self.password), Some(7), vec!["AP".to_string()])
    //             ){
    //                 Ok(_) => {
    //                     log::info!("Archipelago successfully connected!");
    //                 }
    //                 Err(e) => {
    //                     log::info!("Failed to connect: {}", e.to_string());
    //                     return Err(e);
    //                 }
    //             }
    //             Ok(())
    //             // let _ = self.runtime.block_on(client_lock
    //             //     .as_mut()
    //             //     .unwrap()
    //             //     .say("Hello, world!")
    //             // );
    //             // println!("Sent Hello, world!");
    //         }
    //         ArchipelagoAction::LocationChecks => {
    //             // send vector of location ids
    //             Ok(())
    //         }
    //         ArchipelagoAction::LocationScouts => {
    //             // create as hint is false for generation
    //             Ok(())
    //         }
    //         ArchipelagoAction::StatusUpdate => {
    //             // unknown, ready, playing, goal
    //             Ok(())
    //         }
    //         ArchipelagoAction::GetDataPackage => {
    //             // 
    //             Ok(())
    //         }
    //         // ArchipelagoAction::Bounce => {
                
    //         // }
    //     }
    // }
}