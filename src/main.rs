extern crate gl;

extern crate glfw;
use glfw::{Action, Context, Key};

use std::ffi::{ CString, c_void};


#[macro_use] mod shader_program;
mod buffer;
mod texture;
use texture::Texture;
use buffer::{Buffer, BufferType, DrawType};
mod vertex_array;
use vertex_array::VertexArray;
use shader_program::{ Shader, ShaderProgram, ShaderType };

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

    gl::load_with(|s| window.get_proc_address(s).cast());

    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let shader_program = ShaderProgram::builder()
        .attach(Shader::from_file("shaders/vertex.vert", ShaderType::Vertex))
        .attach(Shader::from_file("shaders/fragment.frag", ShaderType::Fragment))
        .link();
   
    Texture::active_number(0);
    let mut texture = Texture::from_file("resources/container.jpg");
    texture.set_activate_number(0);

    Texture::active_number(0);
    let mut texture1 = Texture::from_file("resources/awesomeface.png");
    texture1.set_activate_number(1);

    let verticies: [f32; 32] = [
         0.5,  0.5, 0., 1., 0., 0., 1., 1.,
         0.5, -0.5, 0., 0., 1., 0., 1., 0.,
        -0.5, -0.5, 0., 0., 0., 1., 0., 0.,
        -0.5,  0.5, 0., 1., 1., 1., 0., 1.
    ];

    let indices: [i32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];
    
    let vbo = Buffer::new(BufferType::Array);
    let ebo = Buffer::new(BufferType::ElementArray);
    let vao = VertexArray::new();

    vao.bind();

    vbo.data::<f32, 32>(verticies, DrawType::StaticDraw);
    ebo.data::<i32, 6>(indices, DrawType::StaticDraw);

    unsafe {
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (8 * std::mem::size_of::<f32>()) as i32, 
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as i32,
            (3 * std::mem::size_of::<f32>()) as i32 as *const c_void
        );
        gl::EnableVertexAttribArray(1);
        
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<f32>()) as i32,
            (6 * std::mem::size_of::<f32>()) as i32 as *const c_void
        ); 
        gl::EnableVertexAttribArray(2);
    }
    
    shader_program.use_program();
    shader_program::uniform!(shader_program, Uniform1i, "texture1", 0);
    shader_program::uniform!(shader_program, Uniform1i, "texture2", 1);

    VertexArray::unbind();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        texture.bind();
        texture1.bind();
        shader_program.use_program();
        vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);
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

        glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => {
            unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
        }

        glfw::WindowEvent::Key(Key::F2, _, Action::Press, _) => {
            unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
        }

        _ => {}
    }
}

