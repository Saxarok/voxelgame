mod network_player;
mod server;

use std::{net::{UdpSocket, SocketAddr}, collections::HashMap};
use anyhow::Result;
use log::{error, debug, info};
use network_player::NetworkPlayer;
use server::Server;
use uuid::Uuid as UUID;
use voxelgame::{game::{client::world::player::Player, net::proto::{ClientPacket, ServerPacket}}, utils};

fn main() -> Result<()> {
    utils::init_logger();
    let socket = UdpSocket::bind("127.0.0.1:16000")?;

    let mut server = Server::new();
    let mut buf = [0; 64 * 1024];
    loop {
        let (read, src) = socket.recv_from(&mut buf)?;
        let bytes = &buf[..read];

        if let Ok(packet) = bincode::deserialize::<ClientPacket>(&bytes) {
            match &packet {
                  ClientPacket::PlayerJoin  { .. }
                | ClientPacket::PlayerLeave { .. } => {
                    debug!("{:?}", packet);
                }

                _ => {}
            }

            match packet {
                ClientPacket::QueryPlayerList => {
                    let player_list: HashMap<_, _> = server.players.iter()
                        .map(|(uuid, net_player)| (uuid.clone(), net_player.player.clone()))
                        .collect();

                    let bytes = bincode::serialize(&player_list)?;
                    socket.send_to(&bytes, src)?;
                }

                ClientPacket::PlayerJoin { name } => {
                    let uuid = UUID::new_v4();
                    let token = UUID::new_v4();
                    let player = Player {
                        name     : name.clone(),
                        position : (0.0, 0.0, 0.0).into(),
                    };

                    server.players.insert(uuid, NetworkPlayer {
                        token   : token,
                        address : src,
                        player  : player.clone(),
                    });

                    info!("New connection: {}@{}", name, uuid);

                    // Send player the auth token
                    let bytes = [uuid.as_bytes().clone(), token.as_bytes().clone()].concat();
                    socket.send_to(&bytes, src)?;

                    // Broadcast to others
                    let player_join_packet = ServerPacket::PlayerJoin { uuid, player };
                    server.broadcast(&socket, uuid, &bincode::serialize(&player_join_packet)?);
                }

                ClientPacket::PlayerLeave { token, uuid } => {
                    if let Some(net_player) = server.players.get(&uuid) {
                        if net_player.token == token {
                            server.players.remove(&uuid);
                            
                            // Broadcast to others
                            let player_leave_packet = ServerPacket::PlayerLeave { uuid };
                            server.broadcast(&socket, uuid, &bincode::serialize(&player_leave_packet)?);
                        } else { error!("Incorrect player token"); }
                    } else { error!("No such player on the server"); }

                }

                ClientPacket::PlayerMove { token, uuid, position } => {
                    if let Some(net_player) = server.players.get_mut(&uuid) {
                        if net_player.token == token {
                            net_player.player.position = position;

                            let player_move_packet = ServerPacket::PlayerMove { uuid, position };
                            server.broadcast(&socket, uuid, &bincode::serialize(&player_move_packet)?);
                        } else { error!("Incorrect player token"); }
                    } else { error!("No such player on the server"); }

                }
            }
        } else { error!("Failed to parse incoming packet"); }
    }
}