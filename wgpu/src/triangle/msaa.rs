use crate::settings;

#[derive(Debug)]
pub struct Msaa {
    format: wgpu::TextureFormat,
    sample_count: u32,
    target: Option<Target>,
}

impl Msaa {
    pub fn new(
        format: wgpu::TextureFormat,
        antialiasing: settings::Antialiasing,
    ) -> Msaa {
        Msaa {
            format,
            sample_count: antialiasing.sample_count(),
            target: None,
        }
    }

    pub fn target(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> &wgpu::TextureView {
        match &mut self.target {
            None => {
                self.target = Some(Target::new(
                    device,
                    self.format,
                    self.sample_count,
                    width,
                    height,
                ));
            }
            Some(targets) => {
                if targets.width != width || targets.height != height {
                    self.target = Some(Target::new(
                        device,
                        self.format,
                        self.sample_count,
                        width,
                        height,
                    ));
                }
            }
        }

        let target = self.target.as_ref().unwrap();

        &target.view
    }
}

#[derive(Debug)]
struct Target {
    view: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl Target {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        sample_count: u32,
        width: u32,
        height: u32,
    ) -> Target {
        let extent = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let descriptor = &wgpu::TextureDescriptor {
            size: extent,
            mip_level_count: 1,
            sample_count,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };

        let view = device
            .create_texture(descriptor)
            .create_view(&wgpu::TextureViewDescriptor::default());

        Target {
            view,
            width,
            height,
        }
    }
}
