use crate::shader::{SFragmentShaderPayload, SVertexShaderPayload};
use std::default::Default;
use std::boxed::Box;

pub fn empty_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3 {
    return glm::vec3(1.0f32, 1.0, 1.0);
}

pub fn normal_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3{
    let result_color = fs_payload.normal.normalize() + glm::vec3(0.5f32, 0.5, 0.5);
    return result_color;
}

pub type VertexShaderProgram=Box<dyn Fn(&SVertexShaderPayload) -> glm::Vec3>;
pub type FrameShaderProgram=Box<dyn Fn(&SFragmentShaderPayload) -> glm::Vec3>;