use cgmath::Deg;
use winit::event::{VirtualKeyCode, ElementState};

use crate::graphics::{camera::{Camera, Projection, calc_view_proj}, controller::CameraController, bindable::Bindable};
use crate::graphics::uniform::Uniform;

pub struct PlayerCamera {
    pub camera            : Camera,
    pub controller : CameraController,
    pub uniform    : Uniform,
}

impl PlayerCamera {
    pub fn new(device: &wgpu::Device) -> Self {
        let controller = CameraController::new(4.0, 1.0);
        let camera = Camera::new((0.0, 0.0, 0.0), Deg(0.0), Deg(0.0));
        let uniform = Uniform::new(&device);

        return Self {
            camera,
            controller,
            uniform,
        };
    }

    pub fn on_keyboard(&mut self, key: VirtualKeyCode, state: ElementState){
        self.controller.on_keyboard(key, state);
    }

    pub fn on_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.controller.on_mouse(mouse_dx, mouse_dy);
    }

    pub fn update(&mut self, projection: &Projection, queue: &wgpu::Queue, dt: instant::Duration) {
        self.controller.update_camera(&mut self.camera, dt);

        let view_proj = calc_view_proj(&self.camera, projection);
        self.uniform.update(queue, &view_proj.into());
    }
}

// TODO: probably can use some proc derive magic later
impl Bindable for PlayerCamera {
    fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        self.uniform.bind(render_pass, index);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return self.uniform.layout();
    }
}