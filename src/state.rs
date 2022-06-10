use anyhow::{Result, Context};
use cgmath::{vec3, vec2, Deg};
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{window::Window, event::{KeyboardInput, WindowEvent}};

use crate::graphics::{mesh::{Vertex, Mesh}, texture::Texture, camera::{Camera, CameraUniform, Projection}, controller::CameraController, utils, bindable::Bindable};

pub struct State {
    pub surface           : wgpu::Surface,
    pub device            : wgpu::Device,
    pub queue             : wgpu::Queue,
    pub config            : wgpu::SurfaceConfiguration,
    pub size              : winit::dpi::PhysicalSize<u32>,

    pub mesh              : Mesh,
    pub texture           : Texture,
    pub pipeline          : wgpu::RenderPipeline,

    pub projection        : Projection,
    pub camera            : Camera,
    pub camera_controller : CameraController,
    pub camera_uniform    : CameraUniform,
}

impl State {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.context("Failed to retrive an adapter")?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        // Camera
        let projection = Projection::new(config.width, config.height, Deg(90.0), 0.1, 100.0);
        let camera_controller = CameraController::new(4.0, 1.0);
        let camera = Camera::new((0.0, 1.0, 2.0), Deg(-90.0), Deg(-20.0));
        let mut camera_uniform = CameraUniform::new(&device);
        camera_uniform.update_view_proj(&camera, &projection);

        // Mesh
        let vertices = vec![
            Vertex { pos: vec3(1.0, 1.0, 0.0), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3(0.0, 1.0, 0.0), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3(0.0, 0.0, 0.0), uv: vec2(0.0, 0.0) },

            Vertex { pos: vec3(0.0, 0.0, 0.0), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3(1.0, 0.0, 0.0), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3(1.0, 1.0, 0.0), uv: vec2(1.0, 1.0) },
        ];

        let mesh = Mesh::new(&device, vertices);

        // Textures
        let data = include_bytes!("../res/test.png");
        let texture = Texture::from_bytes(&device, &queue, data, wgpu::FilterMode::Nearest, "test.png")?;

        // Shaders
        let shader = device.create_shader_module(&include_wgsl!("../res/core.wgsl"));
        let pipeline = utils::pipeline(&device, &shader, &config, &[
            texture.layout(),
            camera_uniform.layout(),
        ]);

        return Ok(Self {
            surface,
            device,
            queue,
            config,
            size,

            // Extra stuff
            mesh,
            texture,

            // Camera
            projection,
            camera,
            camera_controller,
            camera_uniform,

            // Pipeline
            pipeline,
        });
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn mouse(&mut self, delta: (f64, f64)) {
        self.camera_controller.process_mouse(delta.0, delta.1);
    }
    
    pub fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. }
                => { self.camera_controller.process_keyboard(*key, *state); }

            _ => {}
        }
    }

    pub fn update(&mut self, dt: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(&self.camera_uniform.buffer, 0, bytemuck::cast_slice(&[Into::<[[f32; 4]; 4]>::into(self.camera_uniform.view_proj)]));
    }

    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        utils::submit(&self.queue, &self.device, |encoder| {
            utils::render(encoder, &view, |mut render_pass| {
                render_pass.set_pipeline(&self.pipeline);
                self.texture.bind(&mut render_pass, 0);
                self.camera_uniform.bind(&mut render_pass, 1);
                self.mesh.draw(&mut render_pass);
            });
        });
    
        output.present();
    }
}