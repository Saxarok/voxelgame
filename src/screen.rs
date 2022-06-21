use winit::event::WindowEvent;

// TODO: provide a screen manager in order to guarantee correct event propogation and abstract it from other code
#[allow(unused_variables)]
pub trait Screen {
    fn render(&mut self, view: &wgpu::TextureView, queue: &wgpu::Queue, device: &wgpu::Device) { }
    fn update(&mut self, now: instant::Instant) { }
    
    // If `false` is returned, events will not be propogated further
    fn mouse(&mut self, delta: (f64, f64)) -> bool { true }
    fn input(&mut self, event: &WindowEvent) -> bool { true }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { }

    /// If `true` is returned, only the `update()` method is expectecd to be called.
    /// Otherwise, engine is expected to call all other methods when events occur. 
    /// Screens can implement this to conditionally prevent event handling.
    fn is_hidden(&mut self) -> bool { false }
}