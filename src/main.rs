#![allow(dead_code)]

extern crate nalgebra_glm as glm;
extern crate image;

mod triangle;
mod rasterizer;
mod utility;

use triangle::*;
use rasterizer::*;

fn main(){
    let t = Triangle::new();
    let b1 = Buffer::COLOR;
    let b2 = Buffer::DEPTH;
    let b3 = b1 | b2;
    println!("Hello, world!");

    let angle = 0f32;
    let command_line = true;

    let width = 300;
    let height = 300;

    let mut rst = Rasterizer::new(width, height);

    // 组装数据 --begin
    let mut pos = Vec::with_capacity(3);
    pos.push(glm::vec3(2.0f32, 0.0, -2.0));
    pos.push(glm::vec3(0.0f32, 2.0, -2.0));
    pos.push(glm::vec3(-2.0f32, 0.0, -2.0));
    
    let mut ind = Vec::with_capacity(1);
    ind.push(glm::vec3(0u32, 1, 2));
    // 世界
    let model_mat = glm::Mat4::identity();
    // 相机
    let eye = glm::vec3(0.0, 0.0, 0.0);
    let at = glm::vec3(0.0, 0.0, 1.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view_mat = glm::look_at_lh(&eye, &at, &up);
    // 投影
    let proj_mat = glm::ortho_lh(-2.5, 2.5, -2.5, 2.5, 0.0, 100.0);
    // 组装数据 --end

    let pos_id = rst.load_position(pos);
    let ind_id = rst.load_indices(ind);

    if command_line {
        rst.clear(Buffer::DEPTH | Buffer::COLOR);

        rst.set_model(&model_mat);
        rst.set_view(&view_mat);
        rst.set_projection(&proj_mat);

        rst.draw(pos_id, ind_id, Primitive::TRIANGLE);

        image::save_buffer("output.png", rst.frame_buf_sclice(), width, height, image::ColorType::Rgba8);

        return;
    }



}
