use conrod::backend::glium;
use conrod::backend::glium::glium::{glutin, texture, Surface, Display};
use conrod::{image, Ui, UiCell};

pub struct Renderer {
    pub display: Display,
    pub ui: Ui,
    pub events_loop: glutin::EventsLoop,
    pub events: Vec<glutin::Event>,
    pub renderer: glium::Renderer,
    pub image_map: image::Map<texture::Texture2d>,
}

impl Renderer {
    pub fn draw(&mut self) {
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.renderer.draw(&self.display, &mut target, &self.image_map).unwrap();
            target.finish().unwrap();
        }
    }

    pub fn set_widgets<'a>(&'a mut self) -> UiCell<'a> {
        self.ui.set_widgets()
    }
}
