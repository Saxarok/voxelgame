
use cgmath::{Matrix4, Vector3, Point3, perspective, Rad, InnerSpace};
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
    pub position : Point3<f32>,
    pub yaw      : Rad<f32>,
    pub pitch    : Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(position: V, yaw: Y, pitch: P) -> Self {
        return Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        };
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        return Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        );
    }
}

#[derive(Debug)]
pub struct CameraUniform {
    pub view_proj: Matrix4::<f32>,
    pub buffer : wgpu::Buffer,
    
    bind_group        : wgpu::BindGroup,
    bind_group_layout : wgpu::BindGroupLayout,
}

impl CameraUniform {
    pub fn new(device: &wgpu::Device) -> Self {
        use cgmath::SquareMatrix;
        let view_proj = Matrix4::identity();

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[Into::<[[f32; 4]; 4]>::into(view_proj)]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });

        return Self {
            view_proj,
            buffer,
            bind_group,
            bind_group_layout
        };
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * projection.calc_matrix() * camera.calc_matrix()).into();
    }
}

impl Bindable for CameraUniform {
    fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return &self.bind_group_layout;
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
