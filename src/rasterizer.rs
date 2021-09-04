#![allow(dead_code)]

use std::{borrow::BorrowMut, collections::HashMap};
use std::cell::RefCell;
use std::default::Default;

use crate::{
    triangle::*, utility,
    shader::*,
    shader_program::*
};
use crate::utility::to_vec4;

#[derive(Default, Clone, Copy)]
pub struct Buffer(u32);

impl Buffer {
    pub const COLOR: Self = Buffer(1);
    pub const DEPTH: Self = Buffer(2);
}

impl std::ops::BitOr for Buffer {
    type Output = Buffer;
    fn bitor(self, rhs: Self) -> Self::Output {
        Buffer(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Buffer {
    type Output = Buffer;
    fn bitand(self, rhs: Self) -> Self::Output {
        Buffer(self.0 & rhs.0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Primitive {
    LINE,
    TRIANGLE,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PosBufId(u32);
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct IndBufId(u32);

#[derive(Debug, Default, Clone, Copy)]
pub struct SVertex {
    pub pos: glm::Vec3,
    pub normal: glm::Vec3,
    pub uv: glm::Vec3,
    pub color: glm::Vec3,
}

const MSAA_COUNT: u32 = 1u32;
const SAMPLE_LIST: [(f32, f32); MSAA_COUNT as usize] = [
    // 1x msaa
    (0.5f32, 0.5f32),
    // 4x msaa
    // (0.25, 0.25),
    // (0.75, 0.25),
    // (0.25, 0.75),
    // (0.75, 0.75),
    // 4x msaa
    // (0.4, 0.1),
    // (0.9, 0.4),
    // (0.1, 0.6),
    // (0.6, 0.9),
];

pub struct Rasterizer {
    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,

    pos_buf: HashMap<PosBufId, Vec<SVertex>>,
    ind_buf: HashMap<IndBufId, Vec<glm::U32Vec3>>,

    frame_bufs: Vec<RefCell<Vec<glm::Vec3>>>,
    depth_bufs: Vec<RefCell<Vec<f32>>>,

    width: u32,
    height: u32,

    next_id: u32,

    frame_shader: FrameShaderProgram,
    msaa: u32,
    // vertex_shader: dyn Fn(SVertexShaderPayload) -> glm::Vec3,

    // constant fragment shader value
    cfv_eye_pos: glm::Vec3,
    cfv_texture0: opencv::prelude::Mat,
}

impl Default for Rasterizer{
    fn default() -> Self {
        let mut frame_bufs = Vec::<RefCell<Vec<glm::Vec3>>>::new();
        let mut depth_bufs = Vec::<RefCell<Vec<f32>>>::new();

        Rasterizer{
            model: glm::one(),
            view: glm::one(),
            projection: glm::one(),

            pos_buf: HashMap::new(),
            ind_buf: HashMap::new(),

            frame_bufs,
            depth_bufs,

            width: 0u32,
            height: 0u32,
            next_id: 0u32,

            frame_shader: Box::new(empty_fs),
            msaa: 0u32,

            cfv_eye_pos: glm::vec3(0., 0., 0.),
            cfv_texture0: opencv::prelude::Mat::default(),
        }
    }
}

fn inside_triangle(x: f32, y:f32, _v: &[glm::Vec3; 3]) -> bool {
    let off = [
        (0f32, 0f32),
        (0f32, 1f32),
        (1f32, 0f32),
        (1f32, 1f32),
    ];
    for (off_x, off_y) in off.iter() {
        let p = glm::vec3(x + off_x, y + off_y, 1.);
        let mut result = 3;
        for i in 0..3 {
            let v0 = _v[i];
            let v1 = _v[(i + 1) % 3];
            let va = p - v0;
            let vb = v1 - v0;
            let vc = glm::cross(&va, &vb);
            if vc.z > 0. {
                result = result & 0x2
            }
            else {
                result = result & 0x1
            }
        }
        if result != 0 {
            return true;
        }
    }

    return false;
}

fn interpolated_value(value: &[glm::Vec3;3], z_interpolated: f32, barycentric: &glm::Vec3, pos: &[glm::Vec4;3]) -> glm::Vec3{
    let mut out_v = glm::vec3(0f32, 0f32, 0f32);
    for i in 0..3usize {
        out_v[i] =
            barycentric[0]*value[0][i]/pos[0].w +
            barycentric[1]*value[1][i]/pos[1].w +
            barycentric[2]*value[2][i]/pos[2].w;
        out_v[i] *= z_interpolated;
    }
    return out_v;
}

fn compute_barycentric_2d(x:f32, y:f32, v:&[glm::Vec3; 3]) -> (f32, f32, f32) {
    // let x = x as f64;
    // let y = y as f64;
    // let v = [
    //     glm::vec3(_v[0].x as f64, _v[0].y as f64, _v[0].z as f64),
    //     glm::vec3(_v[1].x as f64, _v[1].y as f64, _v[1].z as f64),
    //     glm::vec3(_v[2].x as f64, _v[2].y as f64, _v[2].z as f64),
    // ];
    let c1 = (x*(v[1].y - v[2].y) + (v[2].x - v[1].x)*y + v[1].x*v[2].y - v[2].x*v[1].y) / (v[0].x*(v[1].y - v[2].y) + (v[2].x - v[1].x)*v[0].y + v[1].x*v[2].y - v[2].x*v[1].y);
    let c2 = (x*(v[2].y - v[0].y) + (v[0].x - v[2].x)*y + v[2].x*v[0].y - v[0].x*v[2].y) / (v[1].x*(v[2].y - v[0].y) + (v[0].x - v[2].x)*v[1].y + v[2].x*v[0].y - v[0].x*v[2].y);
    let c3 = (x*(v[0].y - v[1].y) + (v[1].x - v[0].x)*y + v[0].x*v[1].y - v[1].x*v[0].y) / (v[2].x*(v[0].y - v[1].y) + (v[1].x - v[0].x)*v[2].y + v[0].x*v[1].y - v[1].x*v[0].y);
    return (c1 as f32, c2 as f32, c3 as f32);
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Rasterizer {
        let msaa = MSAA_COUNT;

        let mut frame_bufs = Vec::new();
        let mut depth_bufs = Vec::new();
        for _ in 0..msaa + 1 {
            let mut frame_buf = RefCell::new(Vec::new());
            let mut depth_buf = RefCell::new(Vec::new());
            frame_buf.borrow_mut().resize((width * height) as usize, glm::Vec3::zeros());
            depth_buf.borrow_mut().resize((width * height) as usize, 0f32);

            frame_bufs.push(frame_buf);
            depth_bufs.push(depth_buf);
        }
        Rasterizer {
            width,
            height,
            frame_bufs,
            depth_bufs,
            msaa,
            ..Default::default()
        }
    }

    pub fn load_position(&mut self, positions: Vec<SVertex>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(PosBufId(id), positions);

        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: Vec<glm::U32Vec3>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(IndBufId(id), indices);

        IndBufId(id)
    }

    pub fn set_model(&mut self, mat: &glm::Mat4) {
        self.model = mat.clone();
    }

    pub fn set_view(&mut self, mat: &glm::Mat4) {
        self.view = mat.clone();
    }

    pub fn set_projection(&mut self, mat: &glm::Mat4) {
        self.projection = mat.clone();
    }

    pub fn set_pixel(&self, point: &glm::Vec3, color: &glm::Vec3) {
        if point.x < 0.0 || point.x > self.width as f32 ||
            point.y < 0.0 || point.y > self.height as f32 
        {
            return;
        }
        let ind = self.get_index(point.x as i32, point.y as i32);
        self.frame_bufs[(self.msaa) as usize].borrow_mut()[ind] = color.clone();
    }

    pub fn set_frame_shader(&mut self, frame_shader: FrameShaderProgram){
        self.frame_shader = frame_shader;
    }

    // constant fragment shader value set
    pub fn set_cfv_eye_pos(&mut self, eye_pos: glm::Vec3) {
        self.cfv_eye_pos = eye_pos;
    }

    pub fn set_cfv_texture0(&mut self, texture: opencv::prelude::Mat) {
        self.cfv_texture0 = texture;
    }

    pub fn clear(&self, buff: Buffer) {
        for i in 0..(self.msaa + 1) as usize {
            if (buff & Buffer::COLOR).0 != 0 {
                self.frame_bufs[i].borrow_mut()
                    .iter_mut()
                    .map(|color| *color = glm::Vec3::zeros())
                    .count();
            }
            if (buff & Buffer::DEPTH).0 != 0 {
                self.depth_bufs[i].borrow_mut()
                    .iter_mut()
                    .map(|depth| *depth = f32::INFINITY)
                    .count();
            }
        }
    }

    pub fn frame_buf_sclice(&self) -> &[u8] {
        let ptr = self.frame_bufs.as_ptr() as *const u8;
        println!("frame buf size: {}", self.frame_bufs[0].borrow().len() * std::mem::size_of::<glm::U8Vec4>());
        unsafe {
            std::slice::from_raw_parts(ptr, self.frame_bufs[0].borrow().len() * std::mem::size_of::<glm::U8Vec4>())
        }
    }

    pub unsafe fn frame_buf_ptr(&mut self) -> *mut std::ffi::c_void {
        self.frame_bufs[0].borrow_mut().as_mut_ptr() as *mut std::ffi::c_void
    }

    pub fn draw(&self, pos_id: PosBufId, ind_id: IndBufId, primitive_type: Primitive) {
        if primitive_type != Primitive::TRIANGLE {
            panic!("Drawing primitives other than triangle is not implemented yet!");
        }

        let pos_buf = self.pos_buf.get(&pos_id).unwrap();
        let ind_buf = self.ind_buf.get(&ind_id).unwrap();


        let pvm = self.projection * self.view * self.model;
        let mut inv_m = self.model.clone();
        inv_m.try_inverse_mut();
        inv_m.transpose_mut();

        for ind in ind_buf {
            let mut v = Vec::new();
            let mut t = Triangle::new();


            for i in 0..3 {
                let v4_pos = utility::to_vec4(&pos_buf[ind[i] as usize].pos, None);
                v.push(pvm * v4_pos);
            }

            for i in 0..3usize {
                t.set_perp_pos(i, &v[i]);
            }

            for vec in v.iter_mut() {
                *vec /= vec.w;
            }

            for vert in v.iter_mut() {
                vert.x = 0.5 * (self.width as f32) * (vert.x + 1.0);
                vert.y = 0.5 * (self.height as f32) * (vert.y + 1.0);
            }

            for i in 0..3usize {
                t.set_vertex(i, &v[i].xyz());
                let uv = &pos_buf[ind[i] as usize].uv;
                t.set_tex_coord(i, uv.x, uv.y);
                t.set_color(i,
                            pos_buf[ind[i] as usize].color.x,
                            pos_buf[ind[i] as usize].color.y,
                            pos_buf[ind[i] as usize].color.z);
                let v4_normal = utility::to_vec4(&pos_buf[ind[i] as usize].normal, None);
                let w_normal = (inv_m * v4_normal).xyz().normalize();
                t.set_normal(i, &w_normal);
                let v4_pos = utility::to_vec4(&pos_buf[ind[i] as usize].pos, None);
                t.set_position(i, &(self.model * v4_pos).xyz());
            }

            // self.rasterize_wireframe(&t);
            self.rasterize_triangle(&t);
        }
    }

    fn rasterize_wireframe(&self, t: &Triangle) {
        self.draw_line(&t.c(), &t.a());
        self.draw_line(&t.a(), &t.b());
        self.draw_line(&t.b(), &t.c());
    }

    fn rasterize_triangle(&self, t: &Triangle) {
        // find aabb
        let mut lb = glm::vec2(self.width as f32, self.height as f32);
        let mut rt = glm::vec2(0f32, 0f32);

        for _v in &t.v {
            lb.x = f32::max(f32::min(lb.x, _v.x), 0f32);
            lb.y = f32::max(f32::min(lb.y, _v.y), 0f32);
            rt.x = f32::min(f32::max(rt.x, _v.x), self.width as f32);
            rt.y = f32::min(f32::max(rt.y, _v.y),  self.height as f32);
        }
        
        let lb = glm::vec2(lb.x as i32, lb.y as i32);
        let rt = glm::vec2(rt.x as i32, rt.y as i32);
        
        let perp_pos = &t.perp_pos;

        let sample_list = &SAMPLE_LIST;

        for x in lb.x .. rt.x {
            for y in lb.y .. rt.y {
                if !inside_triangle(x as f32, y as f32, &t.v) {
                    continue;
                }
                let idx = self.get_index(x, y) as usize;
                for s_idx in 0..sample_list.len() {
                    let _x = x as f32 + sample_list[s_idx].0;
                    let _y = y as f32 + sample_list[s_idx].1;
                    let (alpha, beta, gamma) = compute_barycentric_2d(_x, _y, &t.v);
                    let barycentric = glm::vec3(alpha, beta, gamma);
                    let z_reciprocal = alpha / perp_pos[0].w + beta / perp_pos[1].w + gamma / perp_pos[2].w;
                    let z_interpolated = 1f32 / z_reciprocal;

                    // z test
                    if z_interpolated >= self.depth_bufs[s_idx].borrow_mut()[idx] {
                        continue
                    }

                    // interpolated color
                    let color_interpolated = interpolated_value(
                        &t.color, z_interpolated, &barycentric, &t.perp_pos
                    );
                    let normal_interpolated = interpolated_value(
                        &t.normal, z_interpolated, &barycentric, &t.perp_pos
                    );
                    let tex_coord_interpolated = interpolated_value(
                        &t.tex_coords, z_interpolated, &barycentric, &t.perp_pos
                    );
                    let position_interpolated = interpolated_value(
                        &t.position, z_interpolated, &barycentric, &t.perp_pos
                    );

                    if color_interpolated[0] > 1. {
                        println!("{:?}, {:?}", color_interpolated, z_interpolated);
                    }

                    let fs_payload = SFragmentShaderPayload {
                        eye_pos: self.cfv_eye_pos.clone(),
                        position: position_interpolated,
                        color: color_interpolated,
                        normal: normal_interpolated,
                        tex_coords: tex_coord_interpolated.xy(),

                        texture: self.cfv_texture0.clone(),
                    };

                    // run frame shader
                    let color = (self.frame_shader)(&fs_payload);
                    self.frame_bufs[s_idx].borrow_mut()[idx] = color.zyx(); // rgb -> bgr
                    // z write
                    self.depth_bufs[s_idx].borrow_mut()[idx] = z_interpolated;
                }

                // 合并各buffer信息
                let mut merge_frame_buf = self.frame_bufs[(self.msaa) as usize].borrow_mut();
                let mut merge_depth_buf = self.depth_bufs[(self.msaa) as usize].borrow_mut();
                merge_frame_buf[idx] = glm::zero();
                merge_depth_buf[idx] = 0f32;
                for s_idx in 0..self.msaa as usize {
                    merge_frame_buf[idx] += self.frame_bufs[s_idx].borrow_mut()[idx] / self.msaa as f32;
                    merge_depth_buf[idx] += self.depth_bufs[s_idx].borrow_mut()[idx] / self.msaa as f32;
                }
            }
        }
    }   

    fn draw_line(&self, begin: &glm::Vec3, end: &glm::Vec3) {
        utility::draw_line(
            begin,
            end,
            Box::new(|point: &glm::Vec3, color: &glm::Vec3| self.set_pixel(point, color)),
        );
    }

    fn get_index(&self, row: i32, col: i32) -> usize {
        assert!(row >= 0 && row < self.width as i32);
        assert!(col >= 0 && col < self.height as i32);

        ((self.height - col as u32) * self.width + row as u32) as usize
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }


}
