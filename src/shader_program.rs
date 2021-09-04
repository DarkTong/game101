use crate::shader::{SFragmentShaderPayload, SVertexShaderPayload};
use std::default::Default;
use std::boxed::Box;
use opencv::core::MatTrait;

pub type VertexShaderProgram=Box<dyn Fn(&SVertexShaderPayload) -> glm::Vec3>;
pub type FrameShaderProgram=Box<dyn Fn(&SFragmentShaderPayload) -> glm::Vec3>;

// frame shader
pub fn empty_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3 {
    return glm::vec3(1.0f32, 1.0, 1.0);
}

pub fn normal_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3{
    let result_color = (fs_payload.normal.normalize() + glm::vec3(1., 1., 1.)).scale(0.5);
    return result_color;
}

struct Light {
    pub pos: glm::Vec3,
    pub I: glm::Vec3,
}

pub fn texture_fs(fs_payload: &SFragmentShaderPayload) -> glm::Vec3 {
    let tex_color = glm::vec3(0.7f32, 0., 0.);
    if !fs_payload.texture.empty().unwrap() {

    }

    let ka = glm::vec3(0.005f32, 0.005, 0.005);
    let kd = tex_color.clone();
    let ks = glm::vec3(0.7939, 0.7937, 0.7937);

    let l1 = Light {
        pos: glm::vec3(20., 20., 20.),
        I: glm::vec3(2.0, 2.0, 2.0),
    };
    let l2 = Light {
        pos: glm::vec3(-20., 20., 0.),
        I: glm::vec3(2.0, 2.0, 2.0),
    };
    let lights = [l1, l2];
    let amb_light_I = 0.04;
    let eye_pos = fs_payload.eye_pos;
    let view_pos = fs_payload.position;

    let p = 32.0f32;

    let color = tex_color;
    let normal = fs_payload.normal.normalize();

    let mut out_color = glm::zero();
    for l in lights.iter() {
        let ol = (l.pos - view_pos).normalize();
        let oe = (eye_pos - view_pos).normalize();
        let half_mid = (ol + oe).normalize();
        let nl_ct = f32::max(glm::dot(&normal, &ol), 0.0f32);
        let nh_ct = f32::max(glm::dot(&normal, &half_mid), 0.0f32);

        let ambient_color = glm::matrix_comp_mult(&color, &(amb_light_I * l.I));
        let diffuse_color = glm::matrix_comp_mult(&color, &(nl_ct * l.I));
        let specular_color = l.I * nh_ct.powf(p);

        out_color += ambient_color + diffuse_color + specular_color;
    }

    return out_color;
}
