extern crate gl;

extern crate glfw;
use glfw::{Action, Context, CursorMode, Key};
use nalgebra_glm as glm;

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

#[rustfmt::skip]
const VERTICIES: [f32; 180] = [
    -0.5, -0.5, -0.5,  0.0, 0.0,
    0.5, -0.5, -0.5,  1.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5,  0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 0.0,

    -0.5, -0.5,  0.5,  0.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 1.0,
    -0.5,  0.5,  0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,

    -0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5, -0.5,  1.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5,  0.5,  1.0, 0.0,

    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5,  0.5,  0.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,

    -0.5, -0.5, -0.5,  0.0, 1.0,
    0.5, -0.5, -0.5,  1.0, 1.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    0.5, -0.5,  0.5,  1.0, 0.0,
    -0.5, -0.5,  0.5,  0.0, 0.0,
    -0.5, -0.5, -0.5,  0.0, 1.0,

    -0.5,  0.5, -0.5,  0.0, 1.0,
    0.5,  0.5, -0.5,  1.0, 1.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    0.5,  0.5,  0.5,  1.0, 0.0,
    -0.5,  0.5,  0.5,  0.0, 0.0,
    -0.5,  0.5, -0.5,  0.0, 1.0
];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create window!");

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);

    gl::load_with(|s| window.get_proc_address(s).cast());

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

    vbo.data::<f32, 180>(VERTICIES, DrawType::StaticDraw);

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

    let mut delta;
    let mut last_frame = glfw.get_time();
    while !window.should_close() {
        let current_frame = glfw.get_time();
        delta = current_frame - last_frame;
        last_frame = current_frame;

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event.clone());
        }

        cam.process_input(&window, delta);

        unsafe {
            gl::ClearColor(0., 0., 0., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        texture.bind();
        texture1.bind();

        shader_program.use_program();

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

        vao.bind();

        for (i, position) in cube_positions.iter().enumerate() {
            let mut model = glm::Mat4::identity();
            model = glm::translate(&model, position);
            let angle = glfw.get_time() + i as f64;
            model = glm::rotate(&model, angle as f32, &glm::vec3(1., 0.3, 0.5));

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

        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            // make sure the viewport matches the new window dimensions; note that width and
            // height will be significantly larger than specified on retina displays.
            unsafe { gl::Viewport(0, 0, width, height) }
        }

        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }

        glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        },

        glfw::WindowEvent::Key(Key::F2, _, Action::Press, _) => unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        },

        _ => {}
    }
}
