use bytemuck::{Zeroable, Pod};
use cgmath::{Vector3, Vector2};
use wgpu::util::DeviceExt;

use super::drawable::Drawable;

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos : Vector3<f32>,
    pub uv  : Vector2<f32>,
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride : mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode    : wgpu::VertexStepMode::Vertex,
            attributes   : &Self::ATTRIBUTES,
        }
    }
}

// Mesh
pub struct Mesh {
        buffer   : wgpu::Buffer,
    pub vertices : Vec<Vertex>
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: Vec<Vertex>) -> Self {
        let buffer = Mesh::make_buffer(device, &vertices);

        return Self {
            buffer,
            vertices,
        };
    }

    // TODO: opt for a safer approach and make vertices private?
    pub fn bake(&mut self, device: &wgpu::Device) {
        self.buffer = Mesh::make_buffer(device, &self.vertices);
    }

    pub fn update(&mut self, data: &[Vertex], queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }

    fn make_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        return device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
    }
}

impl Drawable for Mesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        let slice = self.buffer.slice(..);
        render_pass.set_vertex_buffer(0, slice);
        render_pass.draw(0 .. self.vertices.len() as u32, 0 .. 1);
    }
}

// Instanced Mesh
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

unsafe impl Zeroable for InstanceRaw { }
unsafe impl Pod for InstanceRaw { }

pub struct Instance {
    pub position : cgmath::Vector3<f32>,
    pub rotation : cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
        }
    }
}

pub struct InstancedMesh {
        buffer          : wgpu::Buffer,
        instance_buffer : wgpu::Buffer,

    pub vertices        : Vec<Vertex>,
    pub instances       : Vec<Instance>,
}

impl InstancedMesh {
    pub fn new(device: &wgpu::Device, vertices: Vec<Vertex>, instances: Vec<Instance>) -> Self {
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = InstancedMesh::make_buffer(device, &instance_data);
        let buffer = InstancedMesh::make_buffer(device, &vertices);

        return Self {
            buffer,
            instance_buffer,

            vertices,
            instances,
        };
    }

    pub fn bake(&mut self, device: &wgpu::Device) {
        self.buffer = Mesh::make_buffer(device, &self.vertices);
    }

    pub fn bake_instances(&mut self, device: &wgpu::Device) {
        let instance_data = self.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.instance_buffer = InstancedMesh::make_buffer(device, &instance_data);
    }

    pub fn update(&mut self, data: &[Vertex], queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }

    pub fn update_instances(&mut self, data: &[InstanceRaw], queue: &wgpu::Queue) {
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(data));
    }

    fn make_buffer<T: Pod + Zeroable>(device: &wgpu::Device, data: &[T]) -> wgpu::Buffer {
        return device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
    }
}

impl Drawable for InstancedMesh {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instances.len() > 0 {
            render_pass.set_vertex_buffer(0, self.buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(0 .. self.vertices.len() as u32, 0 .. self.instances.len() as u32);
        }
    }
}