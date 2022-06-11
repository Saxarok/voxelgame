use std::rc::Rc;

use anyhow::{Result, Context};
use winit::{window::Window, event::WindowEvent};

use crate::{screen::Screen};

pub struct State {
    pub surface : wgpu::Surface,
    pub device  : wgpu::Device,
    pub config  : wgpu::SurfaceConfiguration,
    pub size    : winit::dpi::PhysicalSize<u32>,
    pub queue   : Rc<wgpu::Queue>, // TODO: are you sure this must be Rc?

    pub screen_stack : Vec<Box<dyn Screen>>,
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
                label: None,
                features: wgpu::Features::empty(), // WebGL doesn't support all of wgpu's features
                limits: if cfg!(target_arch = "wasm32") { wgpu::Limits::downlevel_webgl2_defaults() }
                        else                            { wgpu::Limits::default()                   },
            },
            None,
        ).await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        }; surface.configure(&device, &config);

        let screen_stack = Vec::<Box<dyn Screen>>::new();

        return Ok(Self {
            surface,
            device,
            queue: Rc::new(queue),
            config,
            size,

            screen_stack,
        });
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            for screen in &mut self.screen_stack {
                screen.resize(new_size);
            }
        }
    }

    pub fn mouse(&mut self, delta: (f64, f64)) {
        for screen in &mut self.screen_stack {
            screen.mouse(delta);
        }
    }
    
    pub fn input(&mut self, event: &WindowEvent) {
        for screen in &mut self.screen_stack {
            screen.input(event);
        }
    }

    pub fn update(&mut self, dt: instant::Duration) {
        for screen in &mut self.screen_stack {
            screen.update(dt);
        }
    }

    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        for screen in &self.screen_stack {
            screen.render(&view, &self.queue, &self.device);
        }
    
        output.present();
    }
}