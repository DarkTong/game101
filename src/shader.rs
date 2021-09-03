
// 暂时不用
pub struct SVertexShaderPayload {
    pub position: glm::Vec3,
}

pub struct SVertexShaderOutPayload{
    pub position: glm::Vec3,
}

pub struct SFragmentShaderPayload{
    pub eye_pos: glm::Vec3,
    pub position: glm::Vec3,
    pub color: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,

    pub texture: opencv::core::Mat,
}
