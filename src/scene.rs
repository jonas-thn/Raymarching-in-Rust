use glam::Vec3;

pub fn scene_sdf(p: Vec3) -> f32 {
    let sphere_pos = Vec3::new(0.0, 0.0, 0.0);
    let sphere_radius = 1.0;
    return p.distance(sphere_pos) - sphere_radius;
}