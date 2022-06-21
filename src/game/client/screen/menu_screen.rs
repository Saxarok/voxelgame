use anyhow::Result;
use winit::event::WindowEvent;
use crate::{screen::Screen, egui::EGUI};

pub struct MenuScreen {
    pub egui      : EGUI,
    pub is_hidden : bool,
}

impl MenuScreen {
    pub fn new(device: &wgpu::Device, surface_format: &wgpu::SurfaceConfiguration) -> Result<Self> {
        return Ok(Self {
            egui      : EGUI::new(device, surface_format)?,
            is_hidden : true,
        });
    }
}

impl Screen for MenuScreen {
    fn is_hidden(&mut self) -> bool { self.is_hidden }

    fn render(&mut self, view: &wgpu::TextureView, queue: &wgpu::Queue, device: &wgpu::Device) {
        self.egui.render(view, queue, device, |ctx| {
            egui::Window::new("My Window").show(ctx, |ui| {
                ui.label("Hello World!");
            });
        });
    }

    // Prevent input from being passed to the next screen
    fn mouse(&mut self, _delta: (f64, f64)) -> bool { false }
    fn input(&mut self, event: &WindowEvent) -> bool {
        self.egui.input(event);

        return false;
    }

    // Gracefully handle resizes with egui
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.egui.resize(new_size);
    }
}