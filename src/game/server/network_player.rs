use std::net::SocketAddr;

use uuid::Uuid as UUID;
use voxelgame::game::client::world::player::Player;

pub struct NetworkPlayer {
    pub token   : UUID,
    pub address : SocketAddr,
    pub player  : Player,
    
}