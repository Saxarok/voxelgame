use anyhow::{Result, Context};
use cgmath::{vec3, vec2};
use wgpu::{include_wgsl, util::DeviceExt};
use winit::{window::Window, event::{KeyboardInput, WindowEvent, MouseButton, ElementState}};

use crate::graphics::{mesh::{Vertex, Mesh}, texture::Texture};

pub struct State {
    pub surface    : wgpu::Surface,
    pub device     : wgpu::Device,
    pub queue      : wgpu::Queue,
    pub config     : wgpu::SurfaceConfiguration,
    pub size       : winit::dpi::PhysicalSize<u32>,

    pub mesh       : Mesh,
    pub texture    : Texture,
    pub bind_group : wgpu::BindGroup,
    pub pipeline   : wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: &Window) -> Result<Self> {
        let size = window.inner_size();

        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.context("Failed to retrive an adapter")?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        // Textures
        let data = include_bytes!("../res/test.png");
        let texture = Texture::from_bytes(&device, &queue, data, wgpu::FilterMode::Nearest, "test.png")?;

        let texture_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding    : 0,
                    visibility : wgpu::ShaderStages::FRAGMENT,
                    ty         : wgpu::BindingType::Texture {
                        multisampled   : false,
                        view_dimension : wgpu::TextureViewDimension::D2,
                        sample_type    : wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count      : None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding    : 1,
                    visibility : wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the corresponding Texture entry above.
                    ty         : wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count      : None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding  : 0,
                        resource : wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding  : 1,
                        resource : wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        // Shaders
        let shader = device.create_shader_module(&include_wgsl!("../res/core.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts   : &[&texture_bind_group_layout],
            push_constant_ranges : &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label  : Some("Render Pipeline"),
            layout : Some(&pipeline_layout),

            // Shaders
            vertex: wgpu::VertexState {
                module      : &shader,
                entry_point : "vertex_main",
                buffers     : &[
                    Vertex::describe(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module      : &shader,
                entry_point : "fragment_main",
                targets     : &[wgpu::ColorTargetState {
                    format     : config.format,
                    blend      : Some(wgpu::BlendState::REPLACE),
                    write_mask : wgpu::ColorWrites::ALL,
                }],
            }),

            // Other
            primitive: wgpu::PrimitiveState {
                topology           : wgpu::PrimitiveTopology::TriangleList,
                strip_index_format : None,
                front_face         : wgpu::FrontFace::Ccw,
                cull_mode          : Some(wgpu::Face::Back),
                polygon_mode       : wgpu::PolygonMode::Fill, // Other modes require Features::NON_FILL_POLYGON_MODE
                unclipped_depth    : false,                   // Requires Features::DEPTH_CLIP_CONTROL
                conservative       : false,                   // Requires Features::CONSERVATIVE_RASTERIZATION
            },
            depth_stencil : None,
            multiview     : None,
            multisample   : wgpu::MultisampleState {
                count                     : 1,
                mask                      : !0,
                alpha_to_coverage_enabled : false,
            },
        });

        // Mesh
        let vertices = vec![
            Vertex { pos: vec3( 0.5,  0.5, 0.0), uv: vec2(1.0, 1.0) },
            Vertex { pos: vec3(-0.5,  0.5, 0.0), uv: vec2(0.0, 1.0) },
            Vertex { pos: vec3(-0.5, -0.5, 0.0), uv: vec2(0.0, 0.0) },

            Vertex { pos: vec3(-0.5, -0.5, 0.0), uv: vec2(0.0, 0.0) },
            Vertex { pos: vec3( 0.5, -0.5, 0.0), uv: vec2(1.0, 0.0) },
            Vertex { pos: vec3( 0.5,  0.5, 0.0), uv: vec2(1.0, 1.0) },
        ];

        let mesh = Mesh::new(&device, vertices);

        return Ok(Self {
            surface,
            device,
            queue,
            config,
            size,

            // Extra stuff
            mesh,
            texture,
            bind_group,

            // Pipeline
            pipeline,
        });
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn mouse(&mut self, delta: (f64, f64)) {}
    pub fn input(&mut self, event: &WindowEvent) { }
    pub fn update(&mut self, dt: instant::Duration) { }
    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // [[location(0)]] in the fragment shader
                    wgpu::RenderPassColorAttachment {
                        resolve_target: None,
                        view: &view,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.01,
                                    g: 0.01,
                                    b: 0.01,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
        
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            self.mesh.draw(&mut render_pass);
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}