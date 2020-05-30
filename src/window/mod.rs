mod glutin_interface;

use crate::color::Color;
use crate::window::glutin_interface::GlutinInterface;
use glutin::event::ElementState;
use glutin::event::MouseButton;
use std::time::Duration;

pub struct Window<T: 'static> {
    pub user_data: T,
    pub(crate) must_draw: bool,
    pub(crate) update: Option<fn(&mut Window<T>)>,
    pub(crate) update_time: Option<Duration>,
    pub(crate) mouse_position: (usize, usize),
    pub(crate) glutin_interface: Option<GlutinInterface>,
    pub(crate) mouse_input_handlers:
        Option<Vec<fn(&mut Window<T>, MouseButton, ElementState, (usize, usize))>>,
    pub(crate) pixels: Vec<u8>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl<T: 'static> Window<T> {
    pub fn new(user_data: T, title: &str, width: usize, height: usize) -> Window<T> {
        let mut pixels: Vec<u8> = Vec::with_capacity(width * height * 4);
        for _y in 0..height {
            for _x in 0..width {
                pixels.push(0xff);
                pixels.push(0xff);
                pixels.push(0xff);
                pixels.push(0xff);
            }
        }
        let glutin_interface = GlutinInterface::new(title, width, height);
        Window {
            user_data: user_data,
            must_draw: false,
            update_time: None,
            update: None,
            mouse_position: (0, 0),
            mouse_input_handlers: Some(Vec::new()),
            width: width,
            height: height,
            pixels: pixels,
            glutin_interface: Some(glutin_interface),
        }
    }
    pub fn add_mouse_input_handler(
        &mut self,
        handler: fn(&mut Window<T>, MouseButton, ElementState, (usize, usize)),
    ) {
        match &mut self.mouse_input_handlers {
            Some(mouse_input_handlers) => mouse_input_handlers.push(handler),
            None => (),
        }
    }
    pub fn set_update(&mut self, function: fn(&mut Window<T>), milliseconds: u64) {
        self.update = Some(function);
        self.update_time = Some(Duration::from_millis(milliseconds));
    }
    pub fn show(mut self) {
        let glutin_interface = self.glutin_interface.take().unwrap();
        glutin_interface.show(self);
    }
    pub fn request_redraw(&mut self) {
        self.must_draw = true;
    }
    pub fn draw_rectangle(
        &mut self,
        color: &Color,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        for yi in y..y+height {
            if yi < self.height {
                let y_index = yi * self.width * 4;
                for xi in x..x+width {
                    if xi < self.width {
                        let index = y_index + xi * 4;
                        self.pixels[index] = color.red;
                        self.pixels[index + 1] = color.green;
                        self.pixels[index + 2] = color.blue;
                        self.pixels[index + 3] = 0xff;
                    }
                }
            }
        }
    }
}
