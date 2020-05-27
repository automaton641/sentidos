mod window_internal;
mod glutin_interface;

use glutin::event::ElementState;
use glutin::event::MouseButton;
use window_internal::WindowInternal;

pub struct WindowImpl<T: 'static> {
    internal: Option<WindowInternal<T>>,
}

impl<T: 'static> WindowImpl<T> {
    pub fn new(user_data: T, title: &str, width: usize, height: usize) -> Window<T> {
        let internal = WindowInternal::new(user_data, title, width, height);
        WindowImpl {
            internal: Some(internal),
        }
    }
    pub fn add_mouse_input_handler(
        &mut self,
        handler: fn(MouseButton, ElementState, (usize, usize)),
    ) {
        match &mut self.internal {
            Some(internal) => internal.add_mouse_input_handler(handler),
            None => panic!("None internal"),
        }
    }
    pub fn show(self) {
        self.internal.unwrap().show();
    }
}

trait Window {
    pub fn add_mouse_input_handler(
        &mut Self,
        handler: fn(MouseButton, ElementState, (usize, usize)),
    );
    pub fn show(self);
}