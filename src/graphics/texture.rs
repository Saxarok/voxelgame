use image::{GenericImageView, DynamicImage};
use wgpu::{Device, Queue, FilterMode};
use anyhow::Result;

pub struct Texture {
    pub sampler : wgpu::Sampler,
    pub texture : wgpu::Texture,
    pub view    : wgpu::TextureView,
}

impl Texture {
    pub fn from_bytes(device      : &Device,
                      queue       : &Queue,
                      bytes       : &[u8],
                      filter_mode : FilterMode,
                      label       : &str) -> Result<Self> {
        let img = image::load_from_memory(bytes)?.flipv();
        Self::from_image(device, queue, &img, filter_mode, Some(label))
    }

    pub fn from_image(device      : &Device,
                      queue       : &Queue,
                      img         : &DynamicImage,
                      filter_mode : FilterMode,
                      label       : Option<&str>) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count : 1,
                sample_count    : 1,
                dimension       : wgpu::TextureDimension::D2,
                format          : wgpu::TextureFormat::Rgba8UnormSrgb,
                usage           : wgpu::TextureUsages::TEXTURE_BINDING
                                | wgpu::TextureUsages::COPY_DST,
            }
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect    : wgpu::TextureAspect::All,
                texture   : &texture,
                mip_level : 0,
                origin    : wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset         : 0,
                bytes_per_row  : std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image : std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u : wgpu::AddressMode::ClampToEdge,
                address_mode_v : wgpu::AddressMode::ClampToEdge,
                address_mode_w : wgpu::AddressMode::ClampToEdge,
                mag_filter     : filter_mode,
                min_filter     : filter_mode,
                mipmap_filter  : wgpu::FilterMode::Nearest,
                .. Default::default()
            }
        );

        return Ok(Self { texture, view, sampler });
    }
}
