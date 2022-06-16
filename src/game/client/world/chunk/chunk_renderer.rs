use crate::graphics::{drawable::Drawable, bindable::Bindable, atlas::Atlas};

use super::{chunk_mesh::ChunkMesh, chunk::{Chunk, BlockState}};

pub struct ChunkRenderer {
    pub texture_atlas : Atlas<BlockState>,
    pub chunk_meshes  : Vec<ChunkMesh>,
}

impl ChunkRenderer {
    pub fn new(texture_atlas: Atlas<BlockState>) -> Self {
        return Self {
            texture_atlas,
            chunk_meshes: vec![],
        };
    }

    // Might need to pass chunks instead of blocks later
    pub fn add(&mut self, device: &wgpu::Device, chunk: &Chunk) {
        self.chunk_meshes.push(ChunkMesh::new(device, &chunk.blocks, &self.texture_atlas))
    }
}

impl Drawable for ChunkRenderer {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.texture_atlas.bind(render_pass, 0);
        for mesh in &self.chunk_meshes {
            mesh.draw(render_pass);
        }
    }
}