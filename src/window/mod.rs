mod glutin_interface;

use crate::window::glutin_interface::GlutinInterface;
use glutin::event::ElementState;
use glutin::event::MouseButton;

pub struct Window<T: 'static> {
    pub user_data: T,
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
    pub fn show(mut self) {
        let glutin_interface = self.glutin_interface.take().unwrap();
        glutin_interface.show(self);
    }
}
