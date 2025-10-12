use glam::Vec3;

fn sdf_sphere(p: Vec3, center: Vec3, radius: f32) -> f32 {
    p.distance(center) - radius
}

fn sdf_plane(p:Vec3, n: Vec3, h: f32) -> f32 {
    p.dot(n) - h
}

fn op_union(obj1: (f32, Vec3), obj2: (f32, Vec3)) -> (f32, Vec3) {
    if obj1.0 < obj2.0 {
        obj1
    } else {
        obj2
    }
}

pub fn scene_sdf(p: Vec3) -> (f32, Vec3) {
   let sphere1 = (
        sdf_sphere(p, Vec3::new(-1.5, 0.0, 0.0), 1.0),
        Vec3::new(1.0, 0.0, 0.0), 
    );
    let sphere2 = (
        sdf_sphere(p, Vec3::new(1.5, 0.0, 0.0), 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let plane = (
        sdf_plane(p, Vec3::new(0.0, 1.0, 0.0), -1.5),
        Vec3::new(0.8, 0.8, 0.8), 
    );

    let scene = op_union(sphere1, sphere2);
    let scene = op_union(scene, plane);

    scene
}

pub fn get_normal(p: Vec3) -> Vec3 {
    let epsilon = 0.001;
    let dx = Vec3::new(epsilon, 0.0, 0.0);
    let dy = Vec3::new(0.0, epsilon, 0.0);
    let dz = Vec3::new(0.0, 0.0, epsilon);

    let normal = Vec3::new(
        scene_sdf(p + dx).0 - scene_sdf(p - dx).0,
        scene_sdf(p + dy).0 - scene_sdf(p - dy).0,
        scene_sdf(p + dz).0 - scene_sdf(p - dz).0,
    );

    normal.normalize()
}