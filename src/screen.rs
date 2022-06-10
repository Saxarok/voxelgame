use winit::event::WindowEvent;

pub trait Screen {
    fn render(&self, surface: &wgpu::Surface, view: &wgpu::TextureView, queue: &wgpu::Queue, device: &wgpu::Device) { }
    fn update(&mut self, dt: instant::Duration) { }
    fn mouse(&mut self, delta: (f64, f64)) { }
    fn input(&mut self, event: &WindowEvent) { }
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { }
}