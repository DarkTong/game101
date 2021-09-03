
pub fn reflect(vec: &glm::Vec3, axis: &glm::Vec3) -> glm::Vec3{
    let cos_theta = vec.dot(axis);
    let r_vec: glm::Vec3 = 2 * cos_theta * axis - vec;
    return r_vec.normalize();
}

pub fn texture_sample(texture: &opencv::core::Mat, uv: glm::Vec2) -> glm::Vec4{
    return glm::one()
}
