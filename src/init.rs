use std::ffi::CString;

use egui_glow::EguiGlow;
use winit::event_loop::EventLoop;

use crate::window::Window;

pub fn init_egui<T>(event_loop: &EventLoop<T>, window: &Window) -> EguiGlow {
    let glow = unsafe {
        glow::Context::from_loader_function(|s| {
            window.get_proc_address(CString::new(s).unwrap().as_c_str())
        })
    };

    let glow = std::sync::Arc::new(glow);

    egui_glow::EguiGlow::new(&event_loop, glow, None)
}
