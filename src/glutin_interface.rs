//use crate::window;
use crate::window_internal;
use core::ffi::c_void;
use gl::types::GLuint;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use window_internal::WindowInternal;

pub struct GlutinInterface {
    _framebuffer: GLuint,
    _texture: GLuint,
    event_loop: Option<EventLoop<()>>,
    current_context_wrapper:
        Option<ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>>,
}

impl GlutinInterface {
    pub fn new(title: &str, width: usize, height: usize) -> GlutinInterface {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width as u32, height as u32))
            .with_visible(false)
            .with_resizable(false);
        let context_wrapper = ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        // It is essential to make the context current before calling `gl::load_with`.
        let current_context_wrapper = unsafe { context_wrapper.make_current().unwrap() };

        // Load the OpenGL function pointers
        gl::load_with(|symbol| current_context_wrapper.get_proc_address(symbol));

        let mut texture: GLuint = 0;
        let mut framebuffer: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::GenFramebuffers(1, &mut framebuffer);
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, framebuffer);
            gl::NamedFramebufferTexture(framebuffer, gl::COLOR_ATTACHMENT0, texture, 0);
        }
        GlutinInterface {
            _framebuffer: framebuffer,
            _texture: texture,
            event_loop: Some(event_loop),
            current_context_wrapper: Some(current_context_wrapper),
        }
    }
    pub fn show<T: 'static>(mut self, mut internal: WindowInternal<T>) {
        let event_loop = self.event_loop.take().unwrap();
        let current_context = self.current_context_wrapper.take().unwrap();
        current_context.window().set_visible(true);
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("leaving");
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        //println!("MouseInput");
                        for handler in &internal.mouse_input_handlers {
                            handler(button, state, internal.mouse_position);
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let mouse_position: (f64, f64) = position.into();
                        internal.mouse_position.0 = mouse_position.0 as usize;
                        internal.mouse_position.1 = mouse_position.1 as usize;
                        //println!("position {}, {}", window.mouse_position.0, window.mouse_position.1)
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    unsafe {
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RGBA8 as i32,
                            internal.width as i32,
                            internal.height as i32,
                            0,
                            gl::RGBA,
                            gl::UNSIGNED_BYTE,
                            internal.pixels.as_ptr() as *const c_void,
                        );
                        gl::BlitFramebuffer(
                            0,
                            0,
                            internal.width as i32,
                            internal.height as i32,
                            0,
                            0,
                            internal.width as i32,
                            internal.height as i32,
                            gl::COLOR_BUFFER_BIT,
                            gl::NEAREST,
                        );
                    }
                    current_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
