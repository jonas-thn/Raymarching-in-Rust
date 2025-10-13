use glam::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub focal_length: f32,
    pub move_speed: f32
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            focal_length: 1.0,
            move_speed: 2.0,
        }
    }

    pub fn move_forward(&mut self, dt: f32) {
        self.position.z += self.move_speed * dt;
    }
    pub fn move_backward(&mut self, dt: f32) {
        self.position.z -= self.move_speed * dt;
    }
    pub fn move_left(&mut self, dt: f32) {
        self.position.x -= self.move_speed * dt;
    }
    pub fn move_right(&mut self, dt: f32) {
        self.position.x += self.move_speed * dt;
    }
}