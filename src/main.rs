#![allow(dead_code)]

mod triangle;
mod rasterizer;
mod utility;

use nalgebra_glm as glm;
use triangle::*;
use rasterizer::*;
use utility::*;

fn main(){
    let t = Triangle::new();
    let b1 = Buffer::COLOR;
    let b2 = Buffer::DEPTH;
    let b3 = b1 | b2;
    println!("Hello, world!");

    let angle = 0f32;
    let command_line = false;

    let mut rst = Rasterizer::new(700, 700);

    // 组装数据 --begin
    let pos = vec!([
        glm::Vec3(2.0, 0.0, -2.0),
        glm::Vec3(0.0, 2.0, -2.0),
        glm::Vec3(-2.0, 0.0, -2.0),
    ]);
    // 世界
    let model_mat = glm::Mat4::identity();
    // 相机
    let eye = glm::Vec3::new(0.0, 0.0, 0.0);
    let at = glm::Vec3::new(0.0, 0.0, 1.0);
    let up = glm::Vec3::new(0.0, 1.0, 0.0);
    let view_mat = glm::look_at_lh(&eye, &at, &up);
    // 投影
    let proj_mat = glm::ortho_lh(0.0, 700.0, 0.0, 700.0, 0.0, 100.0);
    // 组装数据 --end

    if command_line {
        rst.clear(Buffer::DEPTH | Buffer::COLOR);

        rst.set_model(&model_mat);
        rst.set_view(&view_mat);
        rst.set_projection(&proj_mat);

        return;
    }



}
