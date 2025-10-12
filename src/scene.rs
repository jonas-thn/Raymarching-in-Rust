use glam::Vec3;

pub fn scene_sdf(p: Vec3) -> f32 {
    let sphere_pos = Vec3::new(0.0, 0.0, 0.0);
    let sphere_radius = 1.0;
    return p.distance(sphere_pos) - sphere_radius;
}

pub fn get_normal(p: Vec3) -> Vec3 {
    let epsilon = 0.001;
    let dx = Vec3::new(epsilon, 0.0, 0.0);
    let dy = Vec3::new(0.0, epsilon, 0.0);
    let dz = Vec3::new(0.0, 0.0, epsilon);

    let normal = Vec3::new(
        scene_sdf(p + dx) - scene_sdf(p - dx),
        scene_sdf(p + dy) - scene_sdf(p - dy),
        scene_sdf(p + dz) - scene_sdf(p - dz),
    );

    normal.normalize()
}