use cgmath::Point3;
use serde::{Serialize, Deserialize};
use uuid::Uuid as UUID;

use crate::game::client::world::player::Player;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientPacket {
    QueryPlayerList,
    PlayerJoin {
        name     : String,
    },
    PlayerLeave {
        token    : UUID,
        uuid     : UUID,
    },
    PlayerMove {
        token    : UUID,
        uuid     : UUID,
        position : Point3<f32>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPacket {
    PlayerJoin {
        uuid     : UUID,
        player   : Player,
    },
    PlayerLeave {
        uuid     : UUID,
    },
    PlayerMove {
        uuid     : UUID,
        position : Point3<f32>,
    },
}