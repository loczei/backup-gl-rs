use nalgebra_glm as glm;
use winit::event::VirtualKeyCode;

pub struct Camera {
    pub postition: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn process_input(&mut self, keys: [bool; 165], delta: f32) {
        let speed = self.speed * delta;

        if keys[VirtualKeyCode::W as usize] {
            self.postition += self.front * speed;
        }

        if keys[VirtualKeyCode::S as usize] {
            self.postition -= self.front * speed;
        }

        if keys[VirtualKeyCode::A as usize] {
            self.postition -= glm::normalize(&glm::cross(&self.front, &self.up)) * speed;
        }

        if keys[VirtualKeyCode::D as usize] {
            self.postition += glm::normalize(&glm::cross(&self.front, &self.up)) * speed;
        }
    }

    pub fn mouse_input(&mut self, offset: (f64, f64)) {
        let mut offset = offset;

        let sensitivity = 0.1;
        offset.0 *= sensitivity;
        offset.1 *= sensitivity;

        self.yaw += offset.0 as f32;
        self.pitch -= offset.1 as f32;

        self.pitch = self.pitch.clamp(-89., 89.);
    }

    pub fn view_matrix(&mut self) -> glm::Mat4 {
        let mut direction = glm::vec3(0., 0., 0.);
        direction.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        direction.y = self.pitch.to_radians().sin();
        direction.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = glm::normalize(&direction);

        glm::look_at(&self.postition, &(self.postition + self.front), &self.up)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            postition: glm::vec3(0., 0., 0.),
            front: glm::vec3(0., 0., -1.),
            up: glm::vec3(0., 1., 0.),
            speed: 2.5,
            yaw: -90.,
            pitch: 0.,
        }
    }
}
