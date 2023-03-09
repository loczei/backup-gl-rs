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
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
