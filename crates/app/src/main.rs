use hot_reload::HotReloadLib;
use shared::winit::event::WindowEvent;
use shared::winit::event_loop::ControlFlow;
use shared::winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

struct App {
    state: shared::State,
    logic: HotReloadLib,
}

impl App {
    fn new(base_path: &str, state: shared::State) -> Self {
        Self {
            state,
            logic: HotReloadLib::new(base_path, "logic"),
        }
    }

    fn update(&mut self) {
        self.logic.update();
        self.logic
            .load_symbol::<fn(&mut shared::State)>("update_state")(&mut self.state);
    }

    fn render(&mut self) {
        self.logic
            .load_symbol::<fn(&mut shared::State)>("render_state")(&mut self.state);
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.logic
            .load_symbol::<fn(&mut shared::State, u32, u32)>("on_resize")(&mut self.state, width, height);
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wgpu Hot Reloading")
        .build(&event_loop)
        .expect("Window should be created");

    let state = pollster::block_on(shared::State::new(&window));
    let mut app = App::new("./target/debug", state);

    event_loop.run(move |event, _, flow| match event {
        Event::WindowEvent { window_id: id, event } if id == window.id() => {
            match event {
                WindowEvent::CloseRequested => *flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    app.resize(size.width, size.width);
                }
                _ => (),
            }
        }
        Event::MainEventsCleared => {
            app.update();
        }
        Event::RedrawRequested(id) if id == window.id() => {
            app.render();
        }
        Event::RedrawEventsCleared => {
            window.request_redraw();
        }
        _ => (),
    })
}
