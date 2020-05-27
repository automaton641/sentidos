use crate::window_internal;

use glutin::event::ElementState;
use glutin::event::MouseButton;
use window_internal::WindowInternal;

pub struct Window<T: 'static> {
    internal: Option<WindowInternal<T>>,
}

impl<T: 'static> Window<T> {
    pub fn new(user_data: T, title: &str, width: usize, height: usize) -> Window<T> {
        let internal = WindowInternal::new(user_data, title, width, height);
        Window {
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
