extern crate gl;

use std::ffi::c_void;

use gl::types::GLenum;

pub enum BufferType {
   Array,
   ElementArray
}

pub enum DrawType {
    StaticDraw,
}

pub struct Buffer {
    id: u32,
    t: BufferType,
}

impl Buffer {
    pub fn new(t: BufferType) -> Self {
        let mut buffer = Buffer { id: 0, t };

        unsafe { gl::GenBuffers(1, &mut buffer.id) };

        buffer 
    }

    fn resolve_type(t: &BufferType) -> GLenum { 
        match t {
            BufferType::Array => gl::ARRAY_BUFFER,
            BufferType::ElementArray => gl::ELEMENT_ARRAY_BUFFER
        }
    }

    pub fn bind(&self) {
        unsafe { 
            gl::BindBuffer(
                Self::resolve_type(&self.t),
                self.id
            ); 
        };
    }

    pub fn data<T, const SIZE: usize>(&self, data: [T; SIZE], t: DrawType) {
        self.bind();

        unsafe {
            gl::BufferData(
                Self::resolve_type(&self.t),
                (SIZE * std::mem::size_of::<T>()) as isize,
                &data[0] as *const T as *const c_void,
                match t {
                    DrawType::StaticDraw => gl::STATIC_DRAW
                }   
            );
        };
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
