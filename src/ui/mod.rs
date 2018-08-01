mod client;
mod action;
mod action_list;
mod tabs;
mod payload;
mod item_state;
mod id_state;
mod renderer;
mod json_inspector;

use std::sync::mpsc::{Receiver, TryRecvError};
use std::io::Error;
use ::state::{State, Status};
use ::actions::{Message, Action};
use conrod::backend::winit;
use conrod::backend::glium;
use conrod::backend::glium::glium::{glutin, texture, Display};
use conrod::backend::glium::glium::glutin::{EventsLoop, Event, WindowBuilder, ContextBuilder};
use conrod::{image, UiBuilder};
use self::id_state::IdState;
use self::renderer::Renderer;

widget_ids! {
    pub struct Ids {
        tab,
        canvas,
        client_canvases[],
        title,
        text,
        client_texts[],
        button,
        canvas_scrollbar,
        action_lists[],
        action_buttons[],
        payload_texts[],
        item_state_texts[],
    }
}

pub fn run(rx: Receiver<State>) {
    let mut state = rx.recv().expect("Ui failed to receive application state");
    let mut render_state = init().expect("Ui failed to init renderer");

    loop {
        match rx.try_recv() {
            Ok(new_state) => state = new_state,
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => panic!("ui disconnected from main thread"),
        };

        match state.status {
            Status::ShuttingDown => break,
            _ => (),
        };

        render_state = render(&state, render_state);
    }
}

fn init() -> Result<(Renderer, Ids), Error> {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;
    const FONT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");

    let events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vision Quest")
        .with_dimensions(WIDTH, HEIGHT)
        ;

    let context = ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4)
        ;

    let display = Display::new(window, context, &events_loop).expect("Failed to create ui display");
    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    ui.fonts.insert_from_file(FONT_PATH).expect("Failed to insert font");

    let ids = Ids::new(ui.widget_id_generator());
    let renderer = glium::Renderer::new(&display).expect("Failed to create ui renderer");
    let image_map = image::Map::<texture::Texture2d>::new();

    Ok((Renderer {
        renderer: renderer,
        display: display,
        image_map: image_map,
        ui: ui,
        events_loop: events_loop,
        events: Vec::new(),
    }, ids))
}

fn render(state: &State, (mut renderer, ids) : (Renderer, Ids)) -> (Renderer, Ids) {
    let mut events = Vec::new();
    renderer.events_loop.poll_events(|event| events.push(event));

    if events.is_empty() {
        renderer.events_loop.run_forever(|event| {
            events.push(event);
            glutin::ControlFlow::Break
        });
    }

    let mut id_state = IdState::new(ids);

    for event in events.drain(..) {
        handle_ui_event(event.clone(), &state);

        let input = match winit::convert_event(event, &renderer.display) {
            None => continue,
            Some(input) => input,
        };

        id_state.reset();

        renderer.ui.handle_event(input);
        let ui_cell = &mut renderer.set_widgets();

        id_state.generate_client_widget_ids(state.client_amount(), ui_cell);
        id_state = tabs::render(id_state, ui_cell, state);
        for (_, client_state) in &state.clients {
            id_state = client::render(id_state, ui_cell, &client_state, state);
        }
    }

    renderer.draw();
    (renderer, id_state.unwrap())
}

fn handle_ui_event(event: Event, state: &State) {
    match event {
        glutin::Event::WindowEvent { event, .. } => handle_window_event(event, state),
        _ => (),
    }
}

fn handle_window_event(event: glutin::WindowEvent, state: &State) {
    match event {
        glutin::WindowEvent::Closed |
        glutin::WindowEvent::KeyboardInput {
            input: glutin::KeyboardInput { virtual_keycode: Some(glutin::VirtualKeyCode::Escape), .. },
            ..
        } => {
            let action = Action {
                domain: String::from("application"),
                invocation: String::from("quit"),
                message: Message::Empty,
                sender: String::from("ui"),
            };

            state.dispatcher.send(action).expect("Failed to send quit action to the application state");
        },
        _ => (),
    }
}
