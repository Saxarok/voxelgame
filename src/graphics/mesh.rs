use cgmath::{Vector3, Vector2};
use wgpu::util::DeviceExt;

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos : Vector3<f32>,
    pub uv  : Vector2<f32>,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn describe<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Mesh {
    buffer   : wgpu::Buffer,
    vertices : Vec<Vertex>
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: Vec<Vertex>) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        return Self {
            buffer,
            vertices,
        };
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass::<'a>) {
        let slice = self.buffer.slice(..);
        render_pass.set_vertex_buffer(0, slice);
        render_pass.draw(0 .. self.vertices.len() as u32, 0 .. 1);
    }
}