
use cgmath::{Matrix4, Vector3, perspective, Rad, InnerSpace};
use wgpu::util::DeviceExt;

use super::bindable::Bindable;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Camera {
    pub position : Vector3<f32>,
    pub yaw      : Rad<f32>,
    pub pitch    : Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Vector3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(position: V, yaw: Y, pitch: P) -> Self {
        return Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        };
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        return Matrix4::look_to_rh( // This is sad.
            (self.position.x, self.position.y, self.position.z).into(),
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        );
    }
}

pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        return Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        };
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        return perspective(self.fovy, self.aspect, self.znear, self.zfar);
    }
}

pub fn calc_view_proj(camera: &Camera, projection: &Projection) -> Matrix4<f32>{
    return OPENGL_TO_WGPU_MATRIX * projection.calc_matrix() * camera.calc_matrix();
}