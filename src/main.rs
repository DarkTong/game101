#![allow(dead_code)]

extern crate nalgebra_glm as glm;
extern crate opencv;

mod triangle;
mod rasterizer;
mod utility;

use triangle::*;
use rasterizer::*;
use opencv::{core::{self, CV_32FC2, CV_32FC3, CV_8UC3}, highgui, imgcodecs::{self, IMREAD_COLOR}, imgproc, prelude::*, videoio};

fn get_model_matrix(rotation_angle: f32, axis: &glm::Vec3) -> glm::Mat4x4 {
    let _mat = glm::Mat4x4::identity();
    let radians = rotation_angle * 3.14 / 180.0;
    let _mat = glm::rotate(&_mat, radians, axis);

    return _mat;
}

fn main(){
    let t = Triangle::new();
    let b1 = Buffer::COLOR;
    let b2 = Buffer::DEPTH;
    let b3 = b1 | b2;
    println!("Hello, world!");

    let angle = 0f32;
    // let command_line = true;
    let command_line = false;

    let width = 300;
    let height = 300;

    let mut rst = Rasterizer::new(width, height);

    // 组装数据 --begin
    let mut pos = Vec::with_capacity(6);
    let mut col = Vec::with_capacity(6);
    pos.push(glm::vec3(2.0f32, 0.0, 0.0));
    pos.push(glm::vec3(0.0f32, 2.0, 0.0));
    pos.push(glm::vec3(-2.0f32, 0.0, 0.0));
    col.push(glm::vec3(1.0f32, 0.0, 0.0));
    col.push(glm::vec3(0.0f32, 1.0, 0.0));
    col.push(glm::vec3(0.0f32, 0.0, 1.0));

    pos.push(glm::vec3(2.0f32, 2.0, -1.0));
    pos.push(glm::vec3(0.0f32, 2.0, 0.0));
    pos.push(glm::vec3(-1.5f32, -1.0, 1.0));
    col.push(glm::vec3(0.0f32, 1.0, 0.0));
    col.push(glm::vec3(0.0f32, 1.0, 0.0));
    col.push(glm::vec3(0.0f32, 1.0, 0.0));

    
    let mut ind = Vec::with_capacity(2);
    ind.push(glm::vec3(0, 1, 2));
    // ind.push(glm::vec3(3, 4, 5));
    // 世界
    let model_mat = glm::Mat4::identity();
    // 相机
    let eye = glm::vec3(0.0, 0.0, -10.0);
    let at = glm::vec3(0.0, 0.0, 1.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view_mat = glm::look_at_lh(&eye, &at, &up);
    // 投影
    // let proj_mat = glm::ortho_lh(-2.5, 2.5, -2.5, 2.5, 0.0, 100.0);
    let proj_mat = 
        glm::perspective_fov_lh(3.14f32/6.0, width as f32, height as f32, 0.0, 100.0);
    // 组装数据 --end

    let pos_id = rst.load_position(pos);
    let ind_id = rst.load_indices(ind);
    let col_id = rst.load_color(col);

    if command_line {
        rst.clear(Buffer::DEPTH | Buffer::COLOR);

        rst.set_model(&model_mat);
        rst.set_view(&view_mat);
        rst.set_projection(&proj_mat);

        rst.draw(pos_id, ind_id, col_id, Primitive::TRIANGLE);

        return;
    }

    let win_name = "window";

    highgui::named_window(win_name, highgui::WINDOW_AUTOSIZE).unwrap();

    let mut key = 0i32;
    let mut angle = 0.;
    while key != 27 {
        rst.clear(Buffer::DEPTH | Buffer::COLOR);

        rst.set_model(&get_model_matrix(angle, &glm::vec3(0., 1., 0.)));
        rst.set_view(&view_mat);
        rst.set_projection(&proj_mat);

        rst.draw(pos_id, ind_id, col_id, Primitive::TRIANGLE);

        let mat = unsafe {
            Mat::new_nd_with_data(
                &[width as i32, height as i32], CV_32FC3, 
                rst.frame_buf_ptr(),
                None).unwrap()
        };

        highgui::imshow(win_name, &mat).unwrap();

        key = highgui::wait_key(10).unwrap();

        if key == ('a' as i32) {
            angle += 10.0;
        }
        else if key == ('d' as i32) {
            angle -= 10.0;
        }

    }

}
