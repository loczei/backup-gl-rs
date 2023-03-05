extern crate gl;

use egui::Slider;
use glutin::{
    config::ConfigTemplateBuilder,
    context::ContextAttributesBuilder,
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::GlSurface,
};
use glutin_winit::{DisplayBuilder, GlWindow};
use nalgebra_glm as glm;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    dpi::Pixel,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoopBuilder,
    window::{CursorGrabMode, WindowBuilder},
};

use std::{
    ffi::{c_void, CString},
    time::{Duration, Instant},
};

#[macro_use]
mod shader_program;
mod buffer;
mod texture;
use buffer::{Buffer, BufferType, DrawType};
use texture::Texture;
mod vertex_array;
use shader_program::{Shader, ShaderProgram, ShaderType};
use vertex_array::VertexArray;
mod camera;
use camera::Camera;
mod verticies;

fn main() {
    let event_loop = EventLoopBuilder::<()>::with_user_event().build();

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

    gl::load_with(|s| {
        let s = CString::new(s).unwrap();
        gl_display.get_proc_address(s.as_c_str()).cast()
    });

    let glow = unsafe {
        glow::Context::from_loader_function(|s| {
            let s = CString::new(s).unwrap();

            gl_display.get_proc_address(&s)
        })
    };

    let glow = std::sync::Arc::new(glow);

    let mut egui = egui_glow::EguiGlow::new(&event_loop, glow, None);

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::Enable(gl::DEPTH_TEST);
    }

    let cube_positions: [glm::Vec3; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    let shader_program = ShaderProgram::builder()
        .attach(Shader::from_file("shaders/vertex.vert", ShaderType::Vertex))
        .attach(Shader::from_file(
            "shaders/fragment.frag",
            ShaderType::Fragment,
        ))
        .link();

    Texture::active_number(0);
    let mut texture = Texture::from_file("resources/container.jpg");
    texture.set_activate_number(0);

    Texture::active_number(0);
    let mut texture1 = Texture::from_file("resources/awesomeface.png");
    texture1.set_activate_number(1);

    let vbo = Buffer::new(BufferType::Array);
    let vao = VertexArray::new();

    vao.bind();

    vbo.data::<f32, 180>(verticies::VERTICIES, DrawType::StaticDraw);

    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as i32 as *const c_void,
        );
        gl::EnableVertexAttribArray(1);
    }

    shader_program.use_program();
    shader_program::uniform!(shader_program, Uniform1i, "texture1", 0);
    shader_program::uniform!(shader_program, Uniform1i, "texture2", 1);

    VertexArray::unbind();

    let mut cam = Camera::default();
    let projection = glm::perspective(800. / 600., (45f32).to_radians(), 0.1, 100.);

    let mut time = Instant::now();
    let mut counter = 0;

    let start_time = Instant::now();

    let mut last_frame = Instant::now();

    window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
    window.set_cursor_visible(false);
    let mut cursor_toggle = true;

    let mut mix = 0.20f32;

    let mut keys_pushed: [bool; 165] = [false; 165];

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => {
            let _ = egui.on_event(&event);
            match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                WindowEvent::Resized(size) => unsafe {
                    gl::Viewport(0, 0, size.width.cast(), size.height.cast())
                },
                _ => (),
            }
        }
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::Key(input) => {
                let key = input.virtual_keycode.unwrap();

                if input.state == ElementState::Released {
                    keys_pushed[key as usize] = false;
                    return;
                }

                keys_pushed[key as usize] = true;

                match key {
                    VirtualKeyCode::F1 => {
                        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) };
                    }
                    VirtualKeyCode::F2 => {
                        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL) };
                    }
                    VirtualKeyCode::T => {
                        window.set_cursor_visible(cursor_toggle);
                        window
                            .set_cursor_grab(match cursor_toggle {
                                false => CursorGrabMode::Confined,
                                true => CursorGrabMode::None,
                            })
                            .unwrap();

                        println!("Mix: {mix}");
                        cursor_toggle = !cursor_toggle;
                    }
                    VirtualKeyCode::Escape => {
                        control_flow.set_exit();
                    }
                    _ => (),
                }
            }
            DeviceEvent::MouseMotion { delta } => {
                if cursor_toggle {
                    cam.mouse_input(delta);
                }
            }
            _ => (),
        },
        Event::MainEventsCleared => {
            let delta = last_frame.elapsed().as_secs_f32();
            last_frame = Instant::now();

            egui.run(&window, |egui_ctx| {
                egui::Window::new("Options").show(egui_ctx, |ui| {
                    ui.heading("Options!");

                    ui.heading(format!("delta: {delta}"));

                    ui.add(Slider::new(&mut mix, 0.0..=1.0).text(" Texture mix"));
                });
            });

            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                gl::ClearColor(0., 0., 0., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            texture.bind();
            texture1.bind();

            shader_program.use_program();

            cam.process_input(keys_pushed, delta);
            shader_program::uniform!(
                shader_program,
                UniformMatrix4fv,
                "view",
                1,
                gl::FALSE,
                glm::value_ptr(&cam.view_matrix()).as_ptr()
            );

            shader_program::uniform!(
                shader_program,
                UniformMatrix4fv,
                "projection",
                1,
                gl::FALSE,
                glm::value_ptr(&projection).as_ptr()
            );

            shader_program::uniform!(shader_program, Uniform1f, "mixValue", mix);

            vao.bind();

            for (i, position) in cube_positions.iter().enumerate() {
                let mut model = glm::Mat4::identity();
                model = glm::translate(&model, position);
                let angle = start_time.elapsed().as_secs_f32() + i as f32;
                model = glm::rotate(&model, angle, &glm::vec3(1., 0.3, 0.5));

                shader_program::uniform!(
                    shader_program,
                    UniformMatrix4fv,
                    "model",
                    1,
                    gl::FALSE,
                    glm::value_ptr(&model).as_ptr()
                );

                unsafe {
                    gl::DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }

            counter += 1;
            if time.elapsed() >= Duration::from_secs(1) {
                println!("FPS: {}", counter);

                time = Instant::now();
                counter = 0;
            }

            egui.paint(&window);

            gl_surface.swap_buffers(&gl_context).unwrap();
        }
        _ => (),
    });
}
