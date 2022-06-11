use std::rc::Rc;

use cgmath::{Deg, vec2, vec3};
use anyhow::Result;
use wgpu::include_wgsl;
use winit::event::{WindowEvent, KeyboardInput};
use crate::{graphics::{mesh::{Mesh, Vertex}, texture::Texture, camera::Projection, utils, bindable::Bindable, drawable::Drawable, depth_buffer::DepthBuffer}, screen::Screen, game::world::{player_camera::PlayerCamera, chunk::Chunk}};

pub struct WorldScreen {
    pub chunk             : Chunk,
    pub texture           : Texture,
    pub pipeline          : wgpu::RenderPipeline,
    pub queue             : Rc<wgpu::Queue>,

    pub projection   : Projection,
    pub camera       : PlayerCamera,
    pub depth_buffer : DepthBuffer,
}

impl WorldScreen {
    pub fn new(device: &wgpu::Device, queue: Rc<wgpu::Queue>, config: &wgpu::SurfaceConfiguration) -> Result<Self> {
        // Camera
        let projection = Projection::new(config.width, config.height, Deg(90.0), 0.1, 100.0);
        let camera = PlayerCamera::new(device);

        // Rendering
        let depth_buffer = DepthBuffer::new(device, config);

        // Mesh
        let chunk = Chunk::new(device);

        // Textures
        let data = include_bytes!("../../../res/test.png");
        let texture = Texture::from_bytes(&device, &queue, data, wgpu::FilterMode::Nearest, "test.png")?;

        // Shaders
        let shader = device.create_shader_module(&include_wgsl!("../../../res/core.wgsl"));
        let pipeline = utils::pipeline(&device, &shader, &config, &[
            texture.layout(),
            camera.layout(),
        ]);

        return Ok(Self {
            chunk,
            texture,
            pipeline,
            queue,

            projection,
            camera,
            depth_buffer,
        });
    }
}

impl Screen for WorldScreen {
    fn render(&self, surface: &wgpu::Surface, view: &wgpu::TextureView, queue: &wgpu::Queue, device: &wgpu::Device) {
        utils::submit(&queue, device, |encoder| {
            utils::render(encoder, &view, Some(&self.depth_buffer.view), |mut render_pass| {
                render_pass.set_pipeline(&self.pipeline);
                self.texture.bind(&mut render_pass, 0);
                self.camera.bind(&mut render_pass, 1);
                self.chunk.draw(&mut render_pass);
            });
        });
    }

    fn update(&mut self, dt: instant::Duration) {
        self.camera.update(&self.projection, &self.queue, dt);
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
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.projection.resize(new_size.width, new_size.height);
        // TODO: Resize depth buffer here
    }
}