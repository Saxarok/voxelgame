use std::{rc::Rc, net::{SocketAddr, UdpSocket}, env, mem::size_of, collections::HashMap};

use crate::{
    game::{client::world::{player_camera::PlayerCamera, chunk::{chunk::{Chunk, BlockState}, chunk_renderer::ChunkRenderer, chunk_mesh::block_face}, player::Player}, net::proto::{ClientPacket, ServerPacket}},
    graphics::{bindable::Bindable, camera::Projection, depth_buffer::DepthBuffer, utils::{self, Side}, atlas::Atlas, drawable::Drawable, uniform::Uniform, mesh::Mesh },
    screen::Screen,
};
use anyhow::Result;
use cgmath::{Deg, Matrix4, SquareMatrix};
use euclid::Box2D;
use log::{info, error};
use rand::Rng;
use uuid::Uuid as UUID;
use wgpu::include_wgsl;
use winit::event::{KeyboardInput, WindowEvent};

pub struct WorldScreen {
    pub chunk_renderer : ChunkRenderer,
    pub chunk          : Chunk,
    
    pub socket         : UdpSocket,

    pub player         : Player,
    pub player_uuid    : UUID,
    pub player_token   : UUID,
    pub player_list    : HashMap<UUID, Player>,
    pub player_mesh    : Mesh,

    pub pipeline       : wgpu::RenderPipeline,
    pub queue          : Rc<wgpu::Queue>,

    pub projection     : Projection,
    pub camera         : PlayerCamera,
    pub transform      : Uniform,
    pub depth_buffer   : DepthBuffer,
}

impl WorldScreen {
    pub fn new(device: &wgpu::Device, queue: Rc<wgpu::Queue>, config: &wgpu::SurfaceConfiguration) -> Result<Self> {
        // Camera
        let projection = Projection::new(config.width, config.height, Deg(90.0), 0.1, 100.0);
        let camera = PlayerCamera::new(device);
        let transform = Uniform::new(device);
        transform.update(&queue, &Matrix4::<f32>::identity().into());

        // Setup player mesh
        let player_mesh = Mesh::new(device, [
            block_face(Side::Top,    0, 0, 0, Box2D::new((0.0, 0.0).into(), (1.0, 1.0).into())),
            block_face(Side::Bottom, 0, 0, 0, Box2D::new((0.0, 0.0).into(), (1.0, 1.0).into())),
            block_face(Side::Right,  0, 0, 0, Box2D::new((0.0, 0.0).into(), (1.0, 1.0).into())),
            block_face(Side::Left,   0, 0, 0, Box2D::new((0.0, 0.0).into(), (1.0, 1.0).into())),
            block_face(Side::Front,  0, 0, 0, Box2D::new((0.0, 0.0).into(), (1.0, 1.0).into())),
            block_face(Side::Back,   0, 0, 0, Box2D::new((0.0, 0.0).into(), (1.0, 1.0).into())),
        ].concat());

        // Rendering
        let depth_buffer = DepthBuffer::new(device, config);

        // Chunks, btw it should be about time we start using resource stores...
        let image_test = image::load_from_memory(include_bytes!("../../../../res/test.png"))?.flipv();
        let image_panel = image::load_from_memory(include_bytes!("../../../../res/panel.png"))?.flipv();
        let texture_atlas = Atlas::new(&[
            (BlockState::TEST , image_test ),
            (BlockState::PANEL, image_panel)
        ], device, &queue, None);

        let chunk = Chunk::new();
        let mut chunk_renderer = ChunkRenderer::new(texture_atlas);
        chunk_renderer.add(device, &chunk);

        // Shaders
        let shader = device.create_shader_module(&include_wgsl!("../../../../res/core.wgsl"));
        let pipeline = utils::pipeline(&device, &shader, &config, &[
            &chunk_renderer.texture_atlas.layout(), // TODO: move pipeline into ChunkRenderer
            camera.layout(),
            transform.layout(),
        ]);

        // networking
        let mut generator = rand::thread_rng(); 
        let player = Player {
            name     : env::var("NAME").unwrap_or(format!("player{}", generator.gen::<u16>())),
            position : (0.0, 0.0, 0.0).into(),
        };

        let port = generator.gen();
        let socket = UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();
        socket.connect("127.0.0.1:16000").unwrap();

        // Query player list
        let query_player_list_packet = bincode::serialize(&ClientPacket::QueryPlayerList).unwrap();
        socket.send(&query_player_list_packet).unwrap();

        let mut player_list_data = [0; 16384];
        let player_list_data_read = socket.recv(&mut player_list_data).unwrap();
        let player_list = bincode::deserialize(&player_list_data[..player_list_data_read]).unwrap();

        // Send PlayerJoin
        let join_packet = bincode::serialize(&ClientPacket::PlayerJoin {
            name     : player.name.clone(),
        }).unwrap();

        socket.send(&join_packet).unwrap();

        // Obtain player token (auth) and UUID
        let mut player_data = [0; size_of::<UUID>() * 2];
        if socket.recv(&mut player_data)? != size_of::<UUID>() * 2 {
            panic!("Corrupted PlayerJoin packet response");
        }

        let player_uuid = UUID::from_bytes(unsafe { player_data[..16].try_into().unwrap_unchecked() });
        let player_token = UUID::from_bytes(unsafe { player_data[16..].try_into().unwrap_unchecked() });
        info!("Player UUID:\t{}", &player_uuid);
        info!("Player token:\t{}", &player_token);

        socket.set_nonblocking(true).unwrap();
        
        return Ok(Self {
            chunk_renderer,
            chunk,

            socket,

            player,
            player_uuid,
            player_token,
            player_list,
            player_mesh,

            pipeline,
            queue,

            projection,
            camera,
            transform,
            depth_buffer,
        });
    }
}

impl Drop for WorldScreen {
    fn drop(&mut self) {
        let bytes = bincode::serialize(&ClientPacket::PlayerLeave {
            token : self.player_token,
            uuid  : self.player_uuid
        }).unwrap();

        self.socket.send(&bytes).unwrap();
    }
}

impl Screen for WorldScreen {
    fn render(&self, view: &wgpu::TextureView, queue: &wgpu::Queue, device: &wgpu::Device) {
        utils::submit(&queue, device, |encoder| {
            utils::render(encoder, &view, Some(&self.depth_buffer.view), |mut render_pass| {
                render_pass.set_pipeline(&self.pipeline);
                self.camera.bind(&mut render_pass, 1);
                self.transform.bind(&mut render_pass, 2);

                let model = Matrix4::<f32>::identity();
                self.transform.update(&queue, &model.into());
                self.chunk_renderer.draw(&mut render_pass);

                for player in &self.player_list {
                    let model = Matrix4::<f32>::from_translation(player.1.position);
                    self.transform.update(&queue, &model.into());
                    self.player_mesh.draw(&mut render_pass);
                }
            });
        });
    }

    fn update(&mut self, dt: instant::Duration) {
        self.camera.update(&self.projection, &self.queue, dt);
        
        let mut buffer = [0; 16384];
        if let Ok(read) = self.socket.recv(&mut buffer) {
            if let Ok(packet) = bincode::deserialize::<ServerPacket>(&buffer[..read]) {
                match packet {
                    ServerPacket::PlayerJoin { uuid, player } => {
                        self.player_list.insert(uuid, player);
                    }

                    ServerPacket::PlayerLeave { uuid } => {
                        self.player_list.remove(&uuid);
                    }

                    ServerPacket::PlayerMove { uuid, position } => {
                        if let Some(player) = self.player_list.get_mut(&uuid) {
                            player.position = position;
                        } else { error!("Invalid server packet: no player with UUID: {}", uuid); }
                    }
                }
            } else { error!("Invalid server packet: corrupt data"); }
        }

    }

    fn mouse(&mut self, delta: (f64, f64)) {
        self.camera.on_mouse(delta.0, delta.1);
    }

    fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. }
                => { self.camera.on_keyboard(*key, *state); }

            _ => {}
        }

        let bytes = bincode::serialize(&ClientPacket::PlayerMove {
            token    : self.player_token,
            uuid     : self.player_uuid,
            position : self.camera.camera.position,
        }).unwrap();
        
        self.socket.send(&bytes).unwrap();
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.projection.resize(new_size.width, new_size.height);
        // TODO: Resize depth buffer here
    }
}
