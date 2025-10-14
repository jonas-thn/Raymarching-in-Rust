use glam::{Mat3, Quat, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub focal_length: f32,
    pub move_speed: f32,
    yaw: f32, 
    pitch: f32,
    sensitivity: f32
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            focal_length: 0.5,
            move_speed: 2.0,
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.002
        }
    }

    pub fn update_rotation(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.sensitivity;
        self.pitch += delta_y * self.sensitivity;

        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }

    pub fn calculate_ray_dir(&self, u: f32, v: f32, aspect_ratio: f32) -> Vec3 {
        let rotation = Mat3::from_quat(Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch));

        let base_dir = Vec3::new(u * aspect_ratio, -v, self.focal_length);
        (rotation * base_dir).normalize()
    }

    fn get_rotation_matrix(&self) -> Mat3 {
        Mat3::from_quat(Quat::from_rotation_y(self.yaw))
    }

    pub fn move_forward(&mut self, dt: f32) {
        let forward_vec = self.get_rotation_matrix() * Vec3::Z;
        self.position += forward_vec * self.move_speed * dt;
    }

    pub fn move_backward(&mut self, dt: f32) {
        let forward_vec = self.get_rotation_matrix() * Vec3::Z;
        self.position -= forward_vec * self.move_speed * dt;
    }

    pub fn move_left(&mut self, dt: f32) {
        let right_vec = self.get_rotation_matrix() * Vec3::X;
        self.position -= right_vec * self.move_speed * dt;
    }

    pub fn move_right(&mut self, dt: f32) {
        let right_vec = self.get_rotation_matrix() * Vec3::X;
        self.position += right_vec * self.move_speed * dt;
    }
}