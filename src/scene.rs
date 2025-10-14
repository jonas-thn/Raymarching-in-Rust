use glam::{Quat, Vec2, Vec3};

fn sdf_sphere(p: Vec3, center: Vec3, radius: f32) -> f32 {
    p.distance(center) - radius
}

fn sdf_plane(p:Vec3, n: Vec3, h: f32) -> f32 {
    p.dot(n) - h
}

fn sdf_box(p: Vec3, center: Vec3, size: Vec3) -> f32 {
    let q = (p - center).abs() - size;
    q.max(Vec3::ZERO).length() + q.max_element().min(0.0)
}

fn sdf_torus(p: Vec3, center: Vec3, rotation: Quat, r: Vec2) -> f32 {
    let p_centered = p - center;

    let p_rotated = rotation.conjugate() * p_centered;

    let q = Vec2::new(
        Vec2::new(p_rotated.x, p_rotated.z).length() - r.x,
        p_rotated.y
    );
    q.length() - r.y
}

fn op_union(obj1: (f32, Vec3), obj2: (f32, Vec3)) -> (f32, Vec3) {
    if obj1.0 < obj2.0 {
        obj1
    } else {
        obj2
    }
}

fn op_smooth_union(obj1: (f32, Vec3), obj2: (f32, Vec3), k: f32) -> (f32, Vec3) {
    let d1 = obj1.0;
    let d2 = obj2.0;
    
    let h = (k - (d1 - d2).abs()).max(0.0) / k;
    let blended_dist = d1.min(d2) - h * h * k * 0.25;

    let blended_color = if d1 < d2 { obj1.1 } else { obj2.1 };

    (blended_dist, blended_color)
}

pub fn scene_sdf(p: Vec3, time: f32) -> (f32, Vec3) {
   let sphere1 = (
        sdf_sphere(p, Vec3::new(-1.5, 0.0, 0.0), 1.0),
        Vec3::new(1.0, 0.0, 0.0), 
    );

    let sphere2 = (
        sdf_sphere(p, Vec3::new(1.5, 0.0, 0.0), 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    );

    let cube = (sdf_box(p, Vec3::new(0.0, time.sin(), 0.0), Vec3::new(0.5, 0.5, 0.5)),
    Vec3::new(0.0, 1.0, 0.0)
    );

    let torus_rotation = Quat::from_rotation_y(time) * Quat::from_rotation_x(time * 0.5);

    let torus1 = (sdf_torus(p, Vec3::new(0.0, 0.0, -8.0), torus_rotation, Vec2::new(1.5, 0.3)), Vec3::new(1.0, 1.0, 0.0));
    let torus2 = (sdf_torus(p, Vec3::new(0.0, 2.0, -8.0), torus_rotation, Vec2::new(1.0, 0.3)), Vec3::new(1.0, 0.0, 1.0));

    let plane = (
        sdf_plane(p, Vec3::new(0.0, 1.0, 0.0), -1.5),
        Vec3::new(0.8, 0.8, 0.8), 
    );

    let objects1 = op_smooth_union(sphere1, sphere2, 0.75); 
    let objects1 = op_smooth_union(objects1, cube, 0.75); 

    let objects2 = op_smooth_union(torus1, torus2, 2.0);

    let scene = op_union(objects1, objects2);
    let scene = op_union(scene, plane);

    scene
}

pub fn get_normal(p: Vec3, time: f32) -> Vec3 {
    let epsilon = 0.001;
    let dx = Vec3::new(epsilon, 0.0, 0.0);
    let dy = Vec3::new(0.0, epsilon, 0.0);
    let dz = Vec3::new(0.0, 0.0, epsilon);

    let normal = Vec3::new(
        scene_sdf(p + dx, time).0 - scene_sdf(p - dx, time).0,
        scene_sdf(p + dy, time).0 - scene_sdf(p - dy, time).0,
        scene_sdf(p + dz, time).0 - scene_sdf(p - dz, time).0,
    );

    normal.normalize()
}