use crate::window::glutin_interface;

use glutin::event::ElementState;
use glutin::event::MouseButton;
use glutin_interface::GlutinInterface;

pub struct WindowInternal<T: 'static> {
    pub user_data: Option<T>,
    pub mouse_position: (usize, usize),
    pub glutin_interface: Option<GlutinInterface>,
    pub mouse_input_handlers: Vec<fn(MouseButton, ElementState, (usize, usize))>,
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl<T: 'static> WindowInternal<T> {
    pub fn new(user_data: T, title: &str, width: usize, height: usize) -> WindowInternal<T> {
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
        WindowInternal {
            user_data: Some(user_data),
            mouse_position: (0, 0),
            mouse_input_handlers: Vec::new(),
            width: width,
            height: height,
            pixels: pixels,
            glutin_interface: Some(glutin_interface),
        }
    }
    pub fn add_mouse_input_handler(
        &mut self,
        handler: fn(MouseButton, ElementState, (usize, usize)),
    ) {
        self.mouse_input_handlers.push(handler);
    }
    pub fn show(mut self) {
        let glutin_interface = self.glutin_interface.take().unwrap();
        glutin_interface.show(self);
    }
}
