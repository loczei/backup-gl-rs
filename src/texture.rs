extern crate gl;
use std::{ffi::c_void, path::Path};

use gl::types::GLenum;

use image::{ io::Reader, DynamicImage };

pub enum TextureFormat {
    Rgb,
    Rgba,
}

impl TextureFormat {
    pub fn resolve(&self) -> GLenum {
        match self {
            Self::Rgb => gl::RGB,
            Self::Rgba => gl::RGBA,
        }
    }
}

pub struct Texture {
    id: u32,
    number: i32
}

impl Texture {
    pub fn new(data: Vec<u8>, height: u32, width: u32, format: TextureFormat) -> Self {
        let mut texture = Texture { 
            id: 0,
            number: -1
        };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
            texture.bind();

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                height as i32,
                width as i32,
                0,
                format.resolve(),
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        texture
    }

    pub fn from_file(path: &str) -> Self {
        let img = Reader::open(&Path::new(path)).unwrap()
            .decode().unwrap().fliph().flipv();

        Self::new(
            img.as_bytes().to_vec(),
            img.height(),
            img.width(),
            match img {
                DynamicImage::ImageRgb8(_) => TextureFormat::Rgb,
                DynamicImage::ImageRgba8(_) => TextureFormat::Rgba,
                _ => panic!("Ty kurwo jebana"),
            }
        )
    }

    pub fn bind(&self) {
        unsafe {
            if self.number > -1 {
                gl::ActiveTexture(gl::TEXTURE0 + self.number as u32);
            }

            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn set_activate_number(&mut self, number: i32) {
        self.number = number;
    }

    pub fn active_number(number: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + number);
        }
    }
}
