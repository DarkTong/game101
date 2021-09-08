use opencv::prelude::MatTraitManual;
use opencv::core::{MatTrait, UMatUsageFlags, Vec3b};
use opencv::core::AccessFlag::ACCESS_READ;
use std::f32::consts::PI;

pub fn reflect(vec: &glm::Vec3, axis: &glm::Vec3) -> glm::Vec3{
    let cos_theta = vec.dot(axis);
    let r_vec = 2. * cos_theta * axis - vec;
    return r_vec.normalize();
}

pub fn texture_sample(texture: &opencv::prelude::Mat, uv: &glm::Vec2) -> glm::Vec4{
    let width = texture.rows();
    let height = texture.cols();
    let _mat = glm::Mat4x4::identity();
    let _mat = glm::rotate(&_mat, PI / 2., &glm::vec3(0., 0., 1.));
    let _uv = _mat * glm::vec4(uv.x, uv.y, 0., 0.);
    let uv = _uv.xy();

    let u = (((uv.x % 1. + 1.) * width  as f32) as i32) % width;
    let v = (((uv.y % 1. + 1.) * height as f32) as i32) % height;
    // let u = width - u;
    let pixel = *texture.at_2d::<Vec3b>(u, v).unwrap();
    return glm::vec4(
        pixel[2] as f32 / 255.0,
        pixel[1] as f32 / 255.0,
        pixel[0] as f32 / 255.0,
        1.0f32,
    );
}

pub fn texture_sample2(texture: &opencv::prelude::Mat, uv: &glm::Vec2) -> glm::Vec3{
    let width = texture.rows();
    let height = texture.cols();

    let mut u = (uv.x * width as f32) as i32;
    let mut v = (uv.y * height as f32) as i32;
    if u < 0 { u = 0; }
    if u >= width { u = width - 1; }
    if v < 0 { v = 0; }
    if v >= height { v = height - 1; }
    let p = *texture.at_2d::<Vec3b>(u, v).unwrap();

    return glm::vec3(
        p[2] as f32,
        p[1] as f32,
        p[0] as f32,
    );
}
