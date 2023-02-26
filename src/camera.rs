use glfw::{Action, Key, Window};
use nalgebra_glm as glm;

pub struct Camera {
    postition: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    speed: f32,
    last_mouse_pos: glm::Vec2,
    yaw: f32,
    pitch: f32,
    fov: f32,
}

impl Camera {
    pub fn process_input(&mut self, window: &Window, delta: f64) {
        let speed = self.speed * delta as f32;

        if window.get_key(Key::W) == Action::Press {
            self.postition += self.front * speed;
        }

        if window.get_key(Key::S) == Action::Press {
            self.postition -= self.front * speed;
        }

        if window.get_key(Key::A) == Action::Press {
            self.postition -= glm::normalize(&glm::cross(&self.front, &self.up)) * speed;
        }

        if window.get_key(Key::D) == Action::Press {
            self.postition += glm::normalize(&glm::cross(&self.front, &self.up)) * speed;
        }

        let mouse_pos = window.get_cursor_pos();

        let mut xoffset = mouse_pos.0 as f32 - self.last_mouse_pos.x;
        let mut yoffset = self.last_mouse_pos.y - mouse_pos.1 as f32;
        self.last_mouse_pos.x = mouse_pos.0 as f32;
        self.last_mouse_pos.y = mouse_pos.1 as f32;

        let sensitivity = 0.1;
        xoffset *= sensitivity;
        yoffset *= sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.pitch > 89. {
            self.pitch = 89.;
        }

        if self.pitch < -89. {
            self.pitch = -89.;
        }
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
            last_mouse_pos: glm::vec2(400., 300.),
            yaw: -90.,
            pitch: 0.,
            fov: 45.,
        }
    }
}
