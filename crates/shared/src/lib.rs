pub use wgpu;
pub use winit;
use winit::window::Window;

pub struct State {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let (adapter, format) = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter_map(|adapter| {
                if let Some(format) = surface.get_preferred_format(&adapter) {
                    Some((adapter, format))
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                // Some(&std::path::Path::new("target/trace")),
                None,
            )
            .await
            .unwrap();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &surface_config);

        State {
            device,
            queue,
            surface,
            surface_config,
        }
    }
}
