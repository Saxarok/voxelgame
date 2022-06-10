use wgpu::include_wgsl;
use winit::{window::Window, event::{WindowEvent, KeyboardInput}};

pub struct State {
    pub surface  : wgpu::Surface,
    pub device   : wgpu::Device,
    pub queue    : wgpu::Queue,
    pub config   : wgpu::SurfaceConfiguration,
    pub size     : winit::dpi::PhysicalSize<u32>,

    pub pipeline : wgpu::RenderPipeline,
}

impl State {
    pub async fn new(window: &Window) -> Self {
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
        ).await.unwrap();

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
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(&include_wgsl!("../res/core.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        use wgpu::*;

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label  : Some("Render Pipeline"),
            layout : Some(&pipeline_layout),

            // Shaders
            vertex: VertexState {
                module      : &shader,
                entry_point : "vertex_main",
                buffers     : &[],
            },
            fragment: Some(FragmentState {
                module      : &shader,
                entry_point : "fragment_main",
                targets     : &[ColorTargetState {
                    format     : config.format,
                    blend      : Some(BlendState::REPLACE),
                    write_mask : ColorWrites::ALL,
                }],
            }),

            // Other
            primitive: PrimitiveState {
                topology           : PrimitiveTopology::TriangleList,
                strip_index_format : None,
                front_face         : FrontFace::Ccw,
                cull_mode          : Some(Face::Back),
                polygon_mode       : PolygonMode::Fill, // Other modes require Features::NON_FILL_POLYGON_MODE
                unclipped_depth    : false,             // Requires Features::DEPTH_CLIP_CONTROL
                conservative       : false,             // Requires Features::CONSERVATIVE_RASTERIZATION
            },
            depth_stencil : None,
            multiview     : None,
            multisample   : MultisampleState {
                count                     : 1,
                mask                      : !0,
                alpha_to_coverage_enabled : false,
            },
        });

        return Self {
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
        };
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, input: &KeyboardInput) {
    }

    pub fn update(&mut self) {
    }

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
            render_pass.draw(0..3, 0..1);
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}