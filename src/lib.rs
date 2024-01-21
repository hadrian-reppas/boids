use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::web::WindowExtWebSys,
    window::WindowBuilder,
};

mod boid;
mod screen_state;
mod state;
mod vector;
mod vertex;

const NUM_BOIDS: u32 = 100;

#[wasm_bindgen(start)]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Warn).unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("body")?;
            let canvas = web_sys::Element::from(window.canvas());
            dst.append_child(&canvas).ok().map(|_| ())
        })
        .unwrap();

    let canvas = window.canvas();
    let (width, height) = (canvas.client_width() as u64, canvas.client_height() as u64);
    let screen_state = screen_state::ScreenState::new(window.scale_factor(), width, height);

    let mut state = state::State::new(window, 3 * NUM_BOIDS).await;
    let mut flock = boid::Flock::new(NUM_BOIDS as usize, width, height);

    event_loop.run(move |event, _, control_flow| {
        let canvas = state.window().canvas();
        screen_state.set_size(canvas.client_width() as u64, canvas.client_height() as u64);

        match event {
            Event::WindowEvent { event, window_id } if window_id == state.window().id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::CursorMoved { position, .. } => {
                        let factor = screen_state.scale_factor();
                        screen_state
                            .set_mouse((position.x / factor) as u64, (position.y / factor) as u64);
                    }
                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        screen_state.set_scale_factor(scale_factor);
                    }
                    WindowEvent::CursorLeft { .. } => {
                        screen_state.cursor_left();
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                let (width, height) = screen_state.size();
                flock.step(width, height);
                state.update_vertices(flock.vertices());
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.reconfigure_surface(),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => log::error!("{e:?}"),
                }
            }
            Event::MainEventsCleared => {
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
