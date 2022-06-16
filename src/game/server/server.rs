use std::{collections::HashMap, io, net::UdpSocket};

use log::debug;
use uuid::Uuid as UUID;

use crate::network_player::NetworkPlayer;

pub struct Server {
    pub players : HashMap<UUID, NetworkPlayer>,
}

impl Server {
    pub fn new() -> Self {
        let players = HashMap::<UUID, NetworkPlayer>::new();
        
        return Self {
            players
        };
    }

    pub fn broadcast(&self, socket: &UdpSocket, uuid: UUID, bytes: &[u8]) {
        for player in &self.players {
            if *player.0 != uuid {
                let address = player.1.address;
                socket.send_to(bytes, address).unwrap();
            }
            
        }
    }
}