use std::{
    ffi::{c_void, CString},
    time::{Duration, Instant},
};

use egui::Slider;
use nalgebra_glm as glm;
use winit::{
    dpi::Pixel,
    event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::EventLoopBuilder,
    window::CursorGrabMode,
};

use window_creator::window::WindowBuilder;
use wrapper::{
    buffer::{Buffer, BufferType, DrawType},
    camera::Camera,
    shader_program::{self, Shader, ShaderProgram, ShaderType},
    texture::Texture,
    vertex_array::VertexArray,
};

mod verticies;

fn main() {
    let event_loop = EventLoopBuilder::<()>::with_user_event().build();

    let window = WindowBuilder::default()
        .window(winit::window::WindowBuilder::new().with_title("LearnOpenGL"))
        .build(&event_loop);

    gl::load_with(|s| {
        let s = CString::new(s).unwrap();
        window.get_proc_address(s.as_c_str()).cast()
    });

    let mut egui = window.init_egui(&event_loop);

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

    window
        .window
        .set_cursor_grab(CursorGrabMode::Confined)
        .unwrap();
    window.window.set_cursor_visible(false);
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
                        window.window.set_cursor_visible(cursor_toggle);
                        window
                            .window
                            .set_cursor_grab(match cursor_toggle {
                                false => CursorGrabMode::Confined,
                                true => CursorGrabMode::None,
                            })
                            .unwrap();

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

            egui.run(&window.window, |egui_ctx| {
                egui::Window::new("Options").show(egui_ctx, |ui| {
                    ui.heading("Options!");

                    ui.heading(format!("delta: {delta}"));
                    ui.heading(format!("FPS: {}", (1. / delta)));

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

            egui.paint(&window.window);

            window.swap_buffer();
        }
        _ => (),
    });
}
