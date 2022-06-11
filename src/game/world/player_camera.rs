use cgmath::Deg;
use winit::event::{VirtualKeyCode, ElementState};

use crate::graphics::{camera::{Camera, CameraUniform, Projection}, controller::CameraController, bindable::Bindable};

pub struct PlayerCamera {
    camera            : Camera,
    camera_controller : CameraController,
    camera_uniform    : CameraUniform,
}

impl PlayerCamera {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_controller = CameraController::new(4.0, 1.0);
        let camera = Camera::new((0.0, 0.0, 0.0), Deg(0.0), Deg(0.0));
        let camera_uniform = CameraUniform::new(&device);

        return Self {
            camera,
            camera_controller,
            camera_uniform,
        };
    }

    pub fn on_keyboard(&mut self, key: VirtualKeyCode, state: ElementState){
        self.camera_controller.on_keyboard(key, state);
    }

    pub fn on_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.camera_controller.on_mouse(mouse_dx, mouse_dy);
    }

    pub fn update(&mut self, projection: &Projection, queue: &wgpu::Queue, dt: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, projection);

        // TODO: come up with some unified shader uniforms API please
        queue.write_buffer(&self.camera_uniform.buffer, 0, bytemuck::cast_slice(&[Into::<[[f32; 4]; 4]>::into(self.camera_uniform.view_proj)]));
    }
}

// TODO: probably can use some proc derive magic later
impl Bindable for PlayerCamera {
    fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        self.camera_uniform.bind(render_pass, index);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return self.camera_uniform.layout();
    }
}