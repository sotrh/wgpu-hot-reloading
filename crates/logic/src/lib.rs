use shared::wgpu;

#[no_mangle]
pub fn on_resize(state: &mut shared::State, width: u32, height: u32) {
    state.surface_config.width = width;
    state.surface_config.height = height;
    state.surface.configure(&state.device, &state.surface_config);
}

#[no_mangle]
pub fn update_state(state: &mut shared::State) {
}

#[no_mangle]
pub fn render_state(state: &mut shared::State) {
    match state.surface.get_current_texture() {
        Ok(output) => {
            let view = output.texture.create_view(&Default::default());
            let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });
            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });
            drop(rpass);
            state.queue.submit(Some(encoder.finish()));
            output.present();
        }
        Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
            todo!();
        }
        e => eprintln!("An error occurred: {:?}", e),
    }

}