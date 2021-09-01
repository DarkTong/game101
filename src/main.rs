#![allow(dead_code)]

extern crate nalgebra_glm as glm;
extern crate opencv;

mod shader;
mod shader_program;
mod triangle;
mod rasterizer;
mod utility;

use triangle::*;
use rasterizer::*;
use opencv::{core::{self, CV_32FC2, CV_32FC3, CV_8UC3, Vector}, highgui, imgcodecs::{self, IMREAD_COLOR, imwrite, imwritemulti}, imgproc::{self, COLOR_BGR2RGB, COLOR_RGB2BGR}, prelude::*, videoio};
use std::io::BufReader;
use std::default;

fn get_model_matrix(rotation_angle: f32, axis: &glm::Vec3) -> glm::Mat4x4 {
    let _mat = glm::Mat4x4::identity();
    let radians = rotation_angle * 3.14 / 180.0;
    let _mat = glm::rotate(&_mat, radians, axis);

    return _mat;
}

fn load_mesh(path: String) -> obj::ObjResult<(Vec<SVertex>, Vec<glm::U32Vec3>)> {
    use obj::*;

    let _file = std::fs::File::open(path)?;
    let read_buf = BufReader::new(_file);

    let wf_obj: Obj<TexturedVertex> = load_obj(read_buf)?;

    let mut mesh = Vec::with_capacity(wf_obj.vertices.len());

    for wf_v in &wf_obj.vertices {
        let pos = glm::vec3(wf_v.position[0], wf_v.position[1], wf_v.position[2]);
        let normal = glm::vec3(wf_v.normal[0], wf_v.normal[1], wf_v.normal[2]);
        let uv = glm::vec3(wf_v.texture[0], wf_v.texture[1], wf_v.texture[2]);
        let color = glm::vec3(1.0f32, 1.0, 1.0);
        mesh.push(SVertex{
            pos, normal, uv, color
        });
    }

    let mut indices = Vec::with_capacity(wf_obj.indices.len());
    for i in 0..wf_obj.indices.len()/3 {
        indices.push(glm::vec3(
           wf_obj.indices[i*3 + 0] as u32,
           wf_obj.indices[i*3 + 1] as u32,
           wf_obj.indices[i*3 + 2] as u32,
        ));
    }

    return Ok((mesh, indices))
}

fn load_static_mesh() -> obj::ObjResult<(Vec<SVertex>, Vec<glm::U32Vec3>)>{
    let mut mesh = Vec::with_capacity(6);
    mesh.push(SVertex{
        pos: glm::vec3(2.0f32, 0.0, 0.0),
        color: glm::vec3(1.0f32, 0.0, 0.0),
        ..Default::default()
    });
    mesh.push(SVertex{
        pos: glm::vec3(0.0f32, 2.0, 0.0),
        color: glm::vec3(0.0f32, 1.0, 0.0),
        ..Default::default()
    });
    mesh.push(SVertex{
        pos: glm::vec3(-2.0f32, 0.0, 0.0),
        color: glm::vec3(0.0f32, 0.0, 1.0),
        ..Default::default()
    });

    mesh.push(SVertex{
        pos: glm::vec3(2.0f32, 2.0, -1.0),
        color: glm::vec3(0.0f32, 1.0, 0.0),
        ..Default::default()
    });
    mesh.push(SVertex{
        pos: glm::vec3(0.0f32, 2.0, 0.0),
        color: glm::vec3(0.0f32, 1.0, 0.0),
        ..Default::default()
    });
    mesh.push(SVertex{
        pos: glm::vec3(-1.5f32, -1.0, 1.0),
        color: glm::vec3(0.0f32, 1.0, 0.0),
        ..Default::default()
    });

    let mut ind = Vec::with_capacity(2);
    ind.push(glm::vec3(0, 1, 2));
    ind.push(glm::vec3(3, 4, 5));

    return Ok((mesh, ind));
}

fn main(){
    println!("{:?}", std::fs::canonicalize("."));
    let b1 = Buffer::COLOR;
    let b2 = Buffer::DEPTH;
    println!("Hello, world!");

    let angle = 0f32;
    // let command_line = true;
    let command_line = false;

    let width = 300;
    let height = 300;

    let mut rst = Rasterizer::new(width, height);

    // 组装数据 --begin
    // let (pos, ind) = load_static_mesh().unwrap();
    let (pos, ind) = load_mesh("./models/cube/cube.obj".to_string()).unwrap();
    println!("{:?}", pos);
    // 世界
    let model_mat = glm::Mat4::identity();
    // 相机
    let eye = glm::vec3(0.0, 0.0, -10.0);
    let at = glm::vec3(0.0, 0.0, 1.0);
    let up = glm::vec3(0.0, 1.0, 0.0);
    let view_mat = glm::look_at_lh(&eye, &at, &up);
    // 投影
    let proj_mat = glm::ortho_lh(-2.5, 2.5, -2.5, 2.5, 0.0, 100.0);
    // let proj_mat = 
    //     glm::perspective_fov_lh(3.14f32/6.0, width as f32, height as f32, 0.0, 100.0);
    // 组装数据 --end

    let pos_id = rst.load_position(pos);
    let ind_id = rst.load_indices(ind);

    if command_line {
        rst.clear(Buffer::DEPTH | Buffer::COLOR);

        rst.set_model(&model_mat);
        rst.set_view(&view_mat);
        rst.set_projection(&proj_mat);

        
        rst.draw(pos_id, ind_id, Primitive::TRIANGLE);

        let mat = unsafe {
            Mat::new_nd_with_data(
                &[width as i32, height as i32], CV_32FC3, 
                rst.frame_buf_ptr(),
                None).unwrap()
        };

        let mut out_mat = Mat::default();
        mat.convert_to(&mut out_mat, CV_8UC3, 1.0, 0.0).unwrap();
        println!("{:?}", out_mat);

        let mut params = Vector::new();
        params.push(COLOR_BGR2RGB);
        imwrite("output.png", &mat, &params).unwrap();
        return;
    }

    let win_name = "window";

    highgui::named_window(win_name, highgui::WINDOW_NORMAL).unwrap();

    let mut key = 0i32;
    let mut angle = 0.;
    while key != 27 {
        rst.clear(Buffer::DEPTH | Buffer::COLOR);

        rst.set_model(&get_model_matrix(angle, &glm::vec3(0., 1., 0.)));
        rst.set_view(&view_mat);
        rst.set_projection(&proj_mat);

        rst.draw(pos_id, ind_id, Primitive::TRIANGLE);

        let mat = unsafe {
            Mat::new_nd_with_data(
                &[width as i32, height as i32], CV_32FC3, 
                rst.frame_buf_ptr(),
                None).unwrap()
        };

        let mut out_mat = Mat::default();
        mat.convert_to(&mut out_mat, CV_8UC3, 255.0, 0.0).unwrap();

        highgui::imshow(win_name, &out_mat).unwrap();

        key = highgui::wait_key(10).unwrap();

        if key == ('a' as i32) {
            angle += 10.0;
        }
        else if key == ('d' as i32) {
            angle -= 10.0;
        }

    }

}
