use opencv::prelude::MatTraitManual;
use opencv::core::{MatTrait, UMatUsageFlags, Vec3b};
use opencv::core::AccessFlag::ACCESS_READ;

pub fn reflect(vec: &glm::Vec3, axis: &glm::Vec3) -> glm::Vec3{
    let cos_theta = vec.dot(axis);
    let r_vec = 2. * cos_theta * axis - vec;
    return r_vec.normalize();
}

pub fn texture_sample(texture: &opencv::prelude::Mat, uv: &glm::Vec2) -> glm::Vec4{
    let width = texture.rows();
    let height = texture.cols();
    let u = (((uv.x % 1. + 1.) * width  as f32) as i32) % width;
    let v = (((uv.y % 1. + 1.) * height as f32) as i32) % height;
    let u = width - u;
    let pixel = *texture.at_2d::<Vec3b>(u, v).unwrap();
    return glm::vec4(
        pixel[2] as f32 / 255.0,
        pixel[1] as f32 / 255.0,
        pixel[0] as f32 / 255.0,
        1.0f32,
    );
}
