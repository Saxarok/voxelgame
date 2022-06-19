use euclid::Size2D;

pub struct DepthBuffer {
    pub texture : wgpu::Texture,
    pub view    : wgpu::TextureView,
}

impl DepthBuffer {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn new(device: &wgpu::Device, size: Size2D<u32, u32>) -> Self {
        let size = wgpu::Extent3d { // 2.
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
                 | wgpu::TextureUsages::TEXTURE_BINDING,
        };

        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        return Self {
            texture,
            view
        };
    }
}