#![allow(dead_code)]

use std::{borrow::BorrowMut, collections::HashMap};
use std::cell::RefCell;

use crate::{triangle::*, utility};

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

#[derive(Default, Clone, Copy)]
pub struct SVertex {
    pub pos: glm::Vec3,
    pub normal: glm::Vec3,
    pub uv: glm::Vec3,
    pub color: glm::Vec3,
}

#[dervie(Default)]
pub struct SVertex {
    pub pos: glm::Vec3,
    pub normal: glm::Vec3,
    pub uv: glm::Vec3,
    pub color: glm::Vec3,
}

#[derive(Default, Clone, Copy)]
pub struct Rasterizer {
    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,

    pos_buf: HashMap<PosBufId, Vec<SVertex>>,
    ind_buf: HashMap<IndBufId, Vec<glm::U32Vec3>>,

    frame_buf: RefCell<Vec<glm::Vec3>>,
    depth_buf: RefCell<Vec<f32>>,

    width: u32,
    height: u32,

    next_id: u32,
}

fn inside_triangle(x: f32, y:f32, _v: &[glm::Vec3; 3]) -> bool {
    let p = glm::vec3(x, y, 1.);
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

    return result != 0;
}

fn interpolated_value(value: &[glm::Vec3;3], z_interpolated: f32, barycentric: &glm::Vec3, pos: &[glm::Vec3;3]) -> glm::Vec3{
    let mut out_v = glm::vec3(0f32, 0f32, 0f32);
    for i in 0..3usize {
        out_v[i] =
            barycentric[0]*value[0][i]/pos[0].z +
            barycentric[1]*value[1][i]/pos[1].z +
            barycentric[2]*value[2][i]/pos[2].z;
        out_v[i] *= z_interpolated;
    }
    return out_v;
}

fn compute_barycentric_2d(x:f32, y:f32, v:&[glm::Vec3; 3]) -> (f32, f32, f32) {
    let c1 = (x*(v[1].y - v[2].y) + (v[2].x - v[1].x)*y + v[1].x*v[2].y - v[2].x*v[1].y) / (v[0].x*(v[1].y - v[2].y) + (v[2].x - v[1].x)*v[0].y + v[1].x*v[2].y - v[2].x*v[1].y);
    let c2 = (x*(v[2].y - v[0].y) + (v[0].x - v[2].x)*y + v[2].x*v[0].y - v[0].x*v[2].y) / (v[1].x*(v[2].y - v[0].y) + (v[0].x - v[2].x)*v[1].y + v[2].x*v[0].y - v[0].x*v[2].y);
    let c3 = (x*(v[0].y - v[1].y) + (v[1].x - v[0].x)*y + v[0].x*v[1].y - v[1].x*v[0].y) / (v[2].x*(v[0].y - v[1].y) + (v[1].x - v[0].x)*v[2].y + v[0].x*v[1].y - v[1].x*v[0].y);
    return (c1, c2, c3);
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Rasterizer {
        let mut frame_buf = RefCell::new(Vec::new());
        let mut depth_buf = RefCell::new(Vec::new());
        frame_buf.borrow_mut().resize((width * height) as usize, glm::Vec3::zeros());
        depth_buf.borrow_mut().resize((width * height) as usize, 0f32);
        Rasterizer {
            width,
            height,
            frame_buf,
            depth_buf,
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
        self.frame_buf.borrow_mut()[ind] = color.clone();
    }

    pub fn clear(&self, buff: Buffer) {
        if (buff & Buffer::COLOR).0 != 0 {
            self.frame_buf.borrow_mut()
                .iter_mut()
                .map(|color| *color = glm::Vec3::zeros())
                .count();
        }
        if (buff & Buffer::DEPTH).0 != 0 {
            self.depth_buf.borrow_mut()
                .iter_mut()
                .map(|depth| *depth = f32::INFINITY)
                .count();
        }
    }

    pub fn frame_buf_sclice(&self) -> &[u8] {
        let ptr = self.frame_buf.as_ptr() as *const u8;
        println!("frame buf size: {}", self.frame_buf.borrow().len() * std::mem::size_of::<glm::U8Vec4>());
        unsafe {
            std::slice::from_raw_parts(ptr, self.frame_buf.borrow().len() * std::mem::size_of::<glm::U8Vec4>())
        }
    }

    pub unsafe fn frame_buf_ptr(&mut self) -> *mut std::ffi::c_void {
        self.frame_buf.borrow_mut().as_mut_ptr() as *mut std::ffi::c_void
    }

    pub fn draw(&self, pos_id: PosBufId, ind_id: IndBufId, primitive_type: Primitive) {
        if primitive_type != Primitive::TRIANGLE {
            panic!("Drawing primitives other than triangle is not implemented yet!");
        }

        let pos_buf = self.pos_buf.get(&pos_id).unwrap();
        let ind_buf = self.ind_buf.get(&ind_id).unwrap();

        let f1 = (100.0 - 0.1) / 2.0;
        let f2 = (100.0 + 0.1) / 2.0;

        let pvm = self.projection * self.view * self.model;

        for ind in ind_buf {
            let mut v = Vec::new();

            v.push(pvm * utility::to_vec4(&pos_buf[ind.x as usize].pos));
            v.push(pvm * utility::to_vec4(&pos_buf[ind.y as usize].pos));
            v.push(pvm * utility::to_vec4(&pos_buf[ind.z as usize].pos));
            // println!("pos:{:?}", v);

            for vec in v.iter_mut() {
                *vec /= vec.w;
            }

            for vert in v.iter_mut() {
                vert.x = 0.5 * (self.width as f32) * (vert.x + 1.0);
                vert.y = 0.5 * (self.height as f32) * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }

            let mut t = Triangle::new();
            for i in 0..3usize {
                t.set_vertex(i, &v[i].xyz());
                t.set_color(i,
                            pos_buf[ind[i] as usize].color.x,
                            pos_buf[ind[i] as usize].color.y,
                            pos_buf[ind[i] as usize].color.z);
                t.set_normal(i, &pos_buf[ind[i] as usize].normal);
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
        
        let v = t.to_vector4();
        // println!("tri v3:{:?}", &t.v);
        // println!("tri c3:{:?}", &t.color);
        let sample_list = [
            (0.5f32, 0.5f32),
            // (0.25f32, 0.25),
            // (0.25f32, 0.75),
            // (0.75f32, 0.25),
            // (0.75f32, 0.75),
        ];
        for x in lb.x .. rt.x {
            for y in lb.y .. rt.y {
                let idx = self.get_index(x, y) as usize;
                let mut result_color = glm::vec3(0f32, 0., 0.);
                let mut result_normal = glm::vec3(0f32, 0., 0.);
                let mut result_tex_coord = glm::vec3(0f32, 0., 0.);
                let mut result_depth = 0f32;
                let mut cnt = 0u32;
                for (off_x, off_y) in sample_list {
                    let (_x, _y) = (x as f32 + off_x, y as f32 + off_y);
                    let mut _ok = inside_triangle(_x, _y, &t.v);
                    if _ok {
                        let (alpha, beta, gamma) = compute_barycentric_2d(_x, _y, &t.v);
                        let barycentric = glm::vec3(alpha, beta, gamma);
                        let z_reciprocal = alpha/v[0].z + beta/v[1].z + gamma/v[2].z;
                        let z_interpolated = 1f32 / z_reciprocal;

                        // z test
                        if z_interpolated < self.depth_buf.borrow_mut()[idx] {
                            // z write
                            result_depth += z_interpolated;
                            // interpolated color
                            let color_interpolated = interpolated_value(
                                &t.color, z_interpolated, &barycentric, &t.v
                            );
                            let normal_interpolated = interpolated_value(
                                &t.normal, z_interpolated, &barycentric, &t.v
                            );
                            let tex_coord_interpolated = interpolated_value(
                                &t.tex_coords, z_interpolated, &barycentric, &t.v
                            );

                            result_color += color_interpolated / sample_list.len() as f32;
                            result_normal += normal_interpolated / sample_list.len() as f32;
                            result_tex_coord += tex_coord_interpolated / sample_list.len() as f32;
                            cnt += 1u32;
                        }
                        else {
                            _ok = false;
                        }
                    }

                    if !_ok {
                        result_color += self.frame_buf.borrow()[idx];
                        result_depth += self.depth_buf.borrow()[idx];
                    }
                }
                if cnt >= (sample_list.len() as u32) / 2 {
                    self.frame_buf.borrow_mut()[idx] = result_color;
                    // self.frame_buf.borrow_mut()[idx] = result_normal;
                    // self.frame_buf.borrow_mut()[idx] = result_tex_coord;
                    self.depth_buf.borrow_mut()[idx] = result_depth / cnt as f32;
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
