use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::io::{Error, ErrorKind};
use ::state::{State, Status};
use ::actions::{Message, Action};
use conrod::backend::winit;
use conrod::backend::glium;
use conrod::backend::glium::glium::{glutin, texture, Surface, Display};
use conrod::backend::glium::glium::glutin::{EventsLoop, Event, WindowBuilder, ContextBuilder};
use conrod::{image, Ui, UiBuilder, color, widget, Colorable, Positionable, Widget};

widget_ids! {
    pub struct Ids {
        canvas,
        title,
        text,
        button,
        canvas_scrollbar,
    }
}

struct Renderer {
    renderer: glium::Renderer,
    display: Display,
    image_map: image::Map::<texture::Texture2d>,
    ui: Ui,
    ids: Ids,
    events_loop: glutin::EventsLoop,
    events: Vec<glutin::Event>,
}

impl Renderer {
    fn draw(&mut self) {
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

pub fn run(rx: Receiver<State>) {
    let mut state = rx.recv().expect("Ui failed to receive application state");
    let mut renderer = init().expect("Ui failed to init renderer");

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

        render(&state, &mut renderer);
    }
}

fn init() -> Result<Renderer, Error> {
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

    Ok(Renderer {
        renderer: renderer,
        display: display,
        image_map: image_map,
        ui: ui,
        ids: ids,
        events_loop: events_loop,
        events: Vec::new(),
    })
}

fn render(state: &State, renderer: &mut Renderer) {
    let mut events = Vec::new();
    renderer.events_loop.poll_events(|event| events.push(event));

    if events.is_empty() {
        renderer.events_loop.run_forever(|event| {
            events.push(event);
            glutin::ControlFlow::Break
        });
    }

    for event in events.drain(..) {
        handle_ui_event(event.clone(), &state);

        let input = match winit::convert_event(event, &renderer.display) {
            None => continue,
            Some(input) => input,
        };

        renderer.ui.handle_event(input);
        let ui_ref = &mut renderer.ui.set_widgets();

        widget::Text::new("Hello World!")
            .middle_of(ui_ref.window)
            .color(color::WHITE)
            .font_size(32)
            .set(renderer.ids.text, ui_ref);
    }

    renderer.draw();
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

            state.dispatcher.send(action);
        },
        _ => (),
    }
}
