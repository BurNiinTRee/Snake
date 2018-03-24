use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, CanvasRenderingContext2d};

use stdweb::web::html_element::CanvasElement;

pub struct Renderer {
    pub canvas: CanvasElement,
    pub context: CanvasRenderingContext2d,
    px_width: u32,
    px_height: u32,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(id: &str, width: u32, height: u32) -> Renderer {
        let canvas: CanvasElement = document()
            .query_selector(id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let px_width = canvas.width() / width;
        let px_height = canvas.height() / height;

        Renderer {
            canvas,
            context,
            px_width,
            px_height,
            width,
            height,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.context.set_fill_style_color(color);
        let x = x * self.px_width;
        let y = y * self.px_height;
        self.context.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.px_width),
            f64::from(self.px_height),
        );
    }

    pub fn clear_all(&self) {
        self.context.set_fill_style_color("white");
        self.context.fill_rect(
            0.,
            0.,
            f64::from(self.width * self.px_width),
            f64::from(self.height * self.px_height),
        );
    }
}
