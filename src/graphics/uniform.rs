use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};
use wgpu::util::DeviceExt;

use super::bindable::Bindable;

pub struct Uniform<T: Default + Clone + Pod + Zeroable> {
    buffer            : wgpu::Buffer,
    bind_group        : wgpu::BindGroup,
    bind_group_layout : wgpu::BindGroupLayout,
    _0                : PhantomData<T>,
}

impl<T: Default + Clone + Pod + Zeroable> Uniform<T> {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::bytes_of(&T::default()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
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
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
        });

        return Self {
            buffer,
            bind_group,
            bind_group_layout,
            _0: Default::default()
        };
    }

    pub fn update(&mut self, queue: &wgpu::Queue, value: &T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(value));
    }
}

impl<T: Default + Clone + Pod + Zeroable> Bindable for Uniform<T> {
    fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return &self.bind_group_layout;
    }
}