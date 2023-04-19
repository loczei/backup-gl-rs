use std::{ffi::CString, time::Instant};

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
    shader_program::{Shader, ShaderProgram, ShaderType},
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

    let shader_program = ShaderProgram::builder()
        .attach(Shader::from_file("shaders/vertex.vert", ShaderType::Vertex))
        .attach(Shader::from_file(
            "shaders/fragment.frag",
            ShaderType::Fragment,
        ))
        .link();

    let vbo = Buffer::new(BufferType::Array);
    let vao = VertexArray::new();

    vbo.data::<f32, 216>(verticies::VERTICIES, DrawType::StaticDraw);

    vao.bind();

    VertexArray::vertex_atrrib_pointer::<f32>(0, 3, gl::FALSE, 6, 0);
    VertexArray::enable_vertex_attrib_array(0);
    VertexArray::vertex_atrrib_pointer::<f32>(1, 3, gl::FALSE, 6, 3);
    VertexArray::enable_vertex_attrib_array(1);

    VertexArray::unbind();

    let light_vao = VertexArray::new();
    light_vao.bind();

    vbo.bind();

    VertexArray::vertex_atrrib_pointer::<f32>(0, 3, gl::FALSE, 6, 0);
    VertexArray::enable_vertex_attrib_array(0);
    VertexArray::vertex_atrrib_pointer::<f32>(1, 3, gl::FALSE, 6, 3);
    VertexArray::enable_vertex_attrib_array(1);

    VertexArray::unbind();

    let light_shader = ShaderProgram::builder()
        .attach(Shader::from_file("shaders/vertex.vert", ShaderType::Vertex))
        .attach(Shader::from_file(
            "shaders/light.frag",
            ShaderType::Fragment,
        ))
        .link();

    let mut cam = Camera::default();
    let projection = glm::perspective(800. / 600., (45f32).to_radians(), 0.1, 100.);

    let mut last_frame = Instant::now();

    window
        .window
        .set_cursor_grab(CursorGrabMode::Confined)
        .unwrap();
    window.window.set_cursor_visible(false);

    let mut specular_strength = 0.5f32;
    let mut ambient_strength = 0.1f32;
    let mut shininess = 32;

    let start_time = Instant::now();

    let mut cursor_toggle = true;

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

                    ui.add(
                        Slider::new(&mut specular_strength, 0.0..=1.0).text("Specular Strength"),
                    );
                    ui.add(Slider::new(&mut ambient_strength, 0.0..=1.0).text("Ambient Strength"));
                    ui.add(
                        Slider::new(&mut shininess, 0..=1024)
                            .text("Shininess")
                            .logarithmic(true),
                    );
                });
            });

            unsafe {
                gl::Enable(gl::DEPTH_TEST);
                gl::ClearColor(0., 0., 0., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            let angle = start_time.elapsed().as_secs_f32();

            let light_pos = glm::vec3(2. * angle.sin(), 0., angle.cos());

            cam.process_input(keys_pushed, delta);

            shader_program.use_program();
            shader_program.set_mat4f("view", &cam.view_matrix());
            shader_program.set_mat4f("projection", &projection);

            shader_program.set_vec3f("objectColor", &glm::vec3(1., 0.5, 0.31));
            shader_program.set_vec3f("lightColor", &glm::vec3(1., 1., 1.));
            shader_program.set_vec3f("lightPos", &light_pos);
            shader_program.set_vec3f("viewPos", &cam.postition);
            shader_program.set_float("specularStrength", specular_strength);
            shader_program.set_float("ambientStrength", ambient_strength);
            shader_program.set_uint("shininess", shininess);

            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &glm::vec3(0., 0., 0.));

            shader_program.set_mat4f("model", &model);

            vao.bind();

            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            light_shader.use_program();

            light_shader.set_mat4f("view", &cam.view_matrix());
            light_shader.set_mat4f("projection", &projection);

            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, &light_pos);
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));

            light_shader.set_mat4f("model", &model);

            light_vao.bind();

            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }

            egui.paint(&window.window);

            window.swap_buffer();
        }
        _ => (),
    });
}
