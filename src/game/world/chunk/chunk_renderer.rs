use crate::graphics::{texture::Texture, drawable::Drawable, bindable::Bindable};

use super::{chunk_mesh::ChunkMesh, chunk::Chunk};

pub struct ChunkRenderer {
    pub texture_atlas : Texture,
    pub chunk_meshes  : Vec<ChunkMesh>,
}

impl ChunkRenderer {
    pub fn new(texture_atlas: Texture) -> Self {
        return Self {
            texture_atlas,
            chunk_meshes: vec![],
        };
    }

    // Might need to pass chunks instead of blocks later
    pub fn add(&mut self, device: &wgpu::Device, chunk: &Chunk) {
        self.chunk_meshes.push(ChunkMesh::new(device, &chunk.blocks))
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