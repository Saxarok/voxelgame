use cgmath::{vec3, vec2};

use crate::graphics::{mesh::{Mesh, Vertex}, drawable::Drawable};

type BlockState = bool;
type ChunkData = [BlockState; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
const CHUNK_SIZE: usize = 8;

pub struct Chunk {
    mesh: Mesh,
    data: ChunkData,
}

impl Drawable for Chunk {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.mesh.draw(render_pass);
    }
}

impl Chunk {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut data = [false; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];
        for i in 0 .. CHUNK_SIZE {
            for j in 0 .. CHUNK_SIZE / 2 {
                for k in 0 .. CHUNK_SIZE {
                    data[Self::index::<CHUNK_SIZE>(i, j, k)] = true;
                }
            }
        }

        let vertices = Self::mesh_simple(&data);
        let mesh = Mesh::new(device, vertices);

        return Self {
            mesh,
            data,
        };
    }

    fn mesh_simple(data: &ChunkData) -> Vec<Vertex> {
        let mut vertices = vec![];
        for i in 0 .. CHUNK_SIZE {
            for j in 0 .. CHUNK_SIZE {
                for k in 0 .. CHUNK_SIZE {
                    if data[Self::index::<CHUNK_SIZE>(i, j, k)] {
                        vertices.extend_from_slice(&[
                            // Top (Y+)
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                            
                            // Bottom (Y-)
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                            
                            // Right (Z+)
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 0.0) },
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 1.0) },
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 0.0) },
                            
                            // Left (Z-)
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 1.0) },
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
                            
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
                            
                            // Front (X+)
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 1.0) },
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 1.0) },
                            
                            Vertex { pos: vec3((1 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 1.0) },
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(0.0, 0.0) },
                            Vertex { pos: vec3((1 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(1.0, 0.0) },

                            // Back (X-)
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 0.0) },
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (1 + k) as f32), uv: vec2(1.0, 1.0) },
                            Vertex { pos: vec3((0 + i) as f32, (1 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 1.0) },
                            Vertex { pos: vec3((0 + i) as f32, (0 + j) as f32, (0 + k) as f32), uv: vec2(0.0, 0.0) },
                        ]);
                    }

                }

            }

        }
        
        return vertices;
    }

    fn index<const MAX: usize>(x: usize, y: usize, z: usize) -> usize {
        return (z * MAX * MAX) + (y * MAX) + x;
    }
}