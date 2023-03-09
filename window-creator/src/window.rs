use core::ffi;
use std::ffi::CStr;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    display::{Display, GetGlDisplay},
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::event_loop::EventLoop;

#[cfg(feature = "egui-init")]
use egui_glow::EguiGlow;

pub struct Window {
    pub window: winit::window::Window,
    gl_display: Display,
    gl_context: PossiblyCurrentContext,
    gl_surface: Surface<WindowSurface>,
}

impl Window {
    pub fn swap_buffer(&self) {
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    pub fn get_proc_address(&self, addr: &CStr) -> *const ffi::c_void {
        self.gl_display.get_proc_address(addr)
    }

    #[cfg(feature = "egui-init")]
    pub fn init_egui<T>(&self, event_loop: &EventLoop<T>) -> EguiGlow {
        let glow = unsafe {
            glow::Context::from_loader_function(|s| {
                self.get_proc_address(std::ffi::CString::new(s).unwrap().as_c_str())
            })
        };

        let glow = std::sync::Arc::new(glow);

        egui_glow::EguiGlow::new(&event_loop, glow, None)
    }
}

#[derive(Default)]
pub struct WindowBuilder {
    window: winit::window::WindowBuilder,
    display: DisplayBuilder,
    context: ContextAttributesBuilder,
    surface: SurfaceAttributesBuilder<WindowSurface>,
}

impl WindowBuilder {
    pub fn window(mut self, builder: winit::window::WindowBuilder) -> Self {
        self.window = builder;
        self
    }

    pub fn display(mut self, builder: DisplayBuilder) -> Self {
        self.display = builder;
        self
    }

    pub fn context(mut self, builder: ContextAttributesBuilder) -> Self {
        self.context = builder;
        self
    }

    pub fn surface(mut self, builder: SurfaceAttributesBuilder<WindowSurface>) -> Self {
        self.surface = builder;
        self
    }

    pub fn build<E>(mut self, event_loop: &EventLoop<E>) -> Window {
        self.display = self.display.with_window_builder(Some(self.window));
        let (window, gl_config) = self
            .display
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

        let attrs = window.build_surface_attributes(self.surface.clone());
        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let context_attrs = self.context.build(Some(window.raw_window_handle()));
        let gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attrs)
                .unwrap()
                .make_current(&gl_surface)
                .unwrap()
        };

        Window {
            window,
            gl_display,
            gl_context,
            gl_surface,
        }
    }
}
