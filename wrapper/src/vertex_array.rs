use std::ffi::c_void;

use gl::types::{GLboolean, GLenum};

pub struct VertexArray {
    pub id: u32,
}

impl VertexArray {
    pub fn new() -> Self {
        let mut arr = Self { id: 0 };

        unsafe {
            gl::GenVertexArrays(1, &mut arr.id);
        }

        arr
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        };
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn vertex_atrrib_pointer<T: GlType>(
        index: u32,
        size: i32,
        normalized: GLboolean,
        step: usize,
        pointer: usize,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                T::resolve(),
                normalized,
                (step * std::mem::size_of::<T>()) as i32,
                match pointer {
                    0 => std::ptr::null(),
                    x => (x * std::mem::size_of::<T>()) as i32 as *const c_void,
                },
            );
        }
    }

    pub fn enable_vertex_attrib_array(i: u32) {
        unsafe {
            gl::EnableVertexAttribArray(i);
        }
    }
}

impl Default for VertexArray {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub trait GlType {
    fn resolve() -> GLenum;
}

impl GlType for f32 {
    fn resolve() -> GLenum {
        gl::FLOAT
    }
}
