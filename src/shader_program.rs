use crate::shader::{SFragmentShaderPayload, SVertexShaderPayload};
use std::default::Default;
use std::boxed::Box;

pub type VertexShaderProgram=Box<dyn Fn(&SVertexShaderPayload) -> glm::Vec3>;
pub type FrameShaderProgram=Box<dyn Fn(&SFragmentShaderPayload) -> glm::Vec3>;

// frame shader
pub fn empty_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3 {
    return glm::vec3(1.0f32, 1.0, 1.0);
}

pub fn normal_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3{
    let result_color = (fs_payload.normal.normalize() + glm::vec3(1., 1., 1.)).scale(0.5);
    if result_color.x == 0. && result_color.y == 0. && result_color.z == 0. {
        println!("result_color:{:?}", result_color);
    }
    return result_color;
}

