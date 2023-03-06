use std::ffi::CString;

use egui_glow::EguiGlow;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::{Display, GetGlDisplay},
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{Surface, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn create_window_with_opengl_context<T>(
    event_loop: &EventLoop<T>,
) -> (
    Display,
    Surface<WindowSurface>,
    PossiblyCurrentContext,
    Window,
) {
    let window_builder = WindowBuilder::new().with_title("Learn OpenGL");

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(&event_loop, ConfigTemplateBuilder::new(), |configs| {
            configs
                .reduce(|a, b| {
                    if a.num_samples() > b.num_samples() {
                        a
                    } else {
                        b
                    }
                })
                .unwrap()
        })
        .unwrap();

    let window = window.unwrap();

    let gl_display = gl_config.display();

    let attrs = window.build_surface_attributes(<_>::default());
    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let context_attrs = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    let gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attrs)
            .unwrap()
            .make_current(&gl_surface)
            .unwrap()
    };

    (gl_display, gl_surface, gl_context, window)
}

pub fn init_egui<T>(event_loop: &EventLoop<T>, gl_display: &Display) -> EguiGlow {
    let glow = unsafe {
        glow::Context::from_loader_function(|s| {
            let s = CString::new(s).unwrap();

            gl_display.get_proc_address(&s)
        })
    };

    let glow = std::sync::Arc::new(glow);

    egui_glow::EguiGlow::new(&event_loop, glow, None)
}
