extern crate gl;
use gl::types::GLchar;

use std::ptr;
use std::ffi::CString;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum ShaderType {
    Fragment,
    Vertex
}

pub struct Shader {
    id: u32,
    //t == type, bruh
    t: ShaderType,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

impl Shader {
    pub fn new(source: &str, t: ShaderType) -> Shader {
        let mut shader = Shader { id: 0, t };

        unsafe { 
            shader.id = match shader.t {
                ShaderType::Vertex => gl::CreateShader(gl::VERTEX_SHADER),
                ShaderType::Fragment => gl::CreateShader(gl::FRAGMENT_SHADER),
            };
            
            let c_str_source = CString::new(source.as_bytes()).unwrap(); 

            gl::ShaderSource(
                shader.id,
                1,
                &c_str_source.as_ptr(),
                ptr::null()
            );

            gl::CompileShader(shader.id);
            check_errors(shader.id, true);

        };

        shader
    }

    pub fn from_file(file_path: &str, t: ShaderType) -> Shader {
        let path = Path::new(file_path);
        let mut file = File::open(path).unwrap();

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        Self::new(s.as_str(), t)
    }
}

pub struct Builder {
    id: u32,
    attached_shaders: Vec<Shader>,
}

pub struct ShaderProgram {
    id: u32,
}

impl Builder {
    fn new() -> Self {
        unsafe { 
            Self {
                id: gl::CreateProgram(),
                attached_shaders: Vec::new(),
            }
        }
    }

    pub fn attach(mut self, shader: Shader) -> Self {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }

        self.attached_shaders.push(shader);

        self
    }

    pub fn link(self) -> ShaderProgram {
        unsafe {
            gl::LinkProgram(self.id);
            
            check_errors(self.id, false);

            for shader in self.attached_shaders {
                drop(shader);
            }
        } 

        ShaderProgram {
            id: self.id
        }
    }
}

impl ShaderProgram {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

// true == shader, false == program
fn check_errors(id: u32, t: bool) {
    let mut success: i32 = i32::from(gl::FALSE);
    let mut info_log = Vec::with_capacity(512);
    
    unsafe {
        if t {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        } else {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success != i32::from(gl::TRUE) {
            if t { 
                gl::GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::COMPILATION_ERROR\n {}", std::string::String::from_utf8_lossy(&info_log));
            } else { 
                gl::GetProgramInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM::LINKING_ERROR\n {}", std::string::String::from_utf8_lossy(&info_log));
            }
        }
    }
}
