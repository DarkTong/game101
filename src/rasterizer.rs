#![allow(dead_code)]

use std::collections::HashMap;

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

#[derive(Default)]
pub struct Rasterizer {
    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,

    pos_buf: HashMap<PosBufId, Vec<glm::Vec3>>,
    ind_buf: HashMap<IndBufId, Vec<glm::U32Vec3>>,

    frame_buf: Vec<glm::Vec3>,
    depth_buf: Vec<f32>,

    width: u32,
    height: u32,

    next_id: u32,
}

fn inside_triangle(x: i32, y:i32, _v: &[glm::Vec3; 3]) -> bool {
    let p = glm::vec3(x as f32, y as f32, 1.);
    let mut result = true;
    for i in 0..3 {
        let v0 = _v[i];
        let v1 = _v[(i + 1) % 3];
        let va = p - v0;
        let vb = v1 - v0;
        let vc = glm::cross(&va, &vb);
        result &= vc.z > 0.;
    }

    return result;
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Rasterizer {
        let mut frame_buf = Vec::new();
        let mut depth_buf = Vec::new();
        frame_buf.resize((width * height) as usize, glm::Vec3::zeros());
        depth_buf.resize((width * height) as usize, 0f32);
        Rasterizer {
            width,
            height,
            frame_buf,
            depth_buf,
            ..Default::default()
        }
    }

    pub fn load_position(&mut self, positions: Vec<glm::Vec3>) -> PosBufId {
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

    pub fn set_pixel(&mut self, point: &glm::Vec3, color: &glm::Vec3) {
        if point.x < 0.0 || point.x > self.width as f32 ||
            point.y < 0.0 || point.y > self.height as f32 
        {
            return;
        }
        let ind = self.get_index(point.x as i32, point.y as i32);
        self.frame_buf[ind] = color.clone();
    }

    pub fn clear(&mut self, buff: Buffer) {
        if (buff & Buffer::COLOR).0 != 0 {
            self.frame_buf
                .iter_mut()
                .map(|color| *color = glm::Vec3::zeros())
                .count();
        }
        if (buff & Buffer::DEPTH).0 != 0 {
            self.depth_buf
                .iter_mut()
                .map(|depth| *depth = f32::INFINITY)
                .count();
        }
    }

    pub fn frame_buf(&self) -> &Vec<glm::Vec3> {
        &self.frame_buf
    }

    pub fn frame_buf_sclice(&self) -> &[u8] {
        let ptr = self.frame_buf.as_ptr() as *const u8;
        println!("frame buf size: {}", self.frame_buf.len() * std::mem::size_of::<glm::U8Vec4>());
        unsafe {
            std::slice::from_raw_parts(ptr, self.frame_buf.len() * std::mem::size_of::<glm::U8Vec4>())
        }
    }

    pub unsafe fn frame_buf_ptr(&mut self) -> *mut std::ffi::c_void {
        self.frame_buf.as_mut_ptr() as *mut std::ffi::c_void
    }

    pub fn draw(&mut self, pos_id: PosBufId, ind_id: IndBufId, primitive_type: Primitive) {
        if primitive_type != Primitive::TRIANGLE {
            panic!("Drawing primitives other than triangle is not implemented yet!");
        }

        let pos_buf = self.pos_buf.get(&pos_id).unwrap();
        let ind_buf = self.ind_buf.get(&ind_id).unwrap();

        let f1 = (100.0 - 0.1) / 2.0;
        let f2 = (100.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        let mut v = Vec::new();
        for ind in ind_buf {
            v.push(mvp * utility::to_vec4(&pos_buf[ind.x as usize]));
            v.push(mvp * utility::to_vec4(&pos_buf[ind.y as usize]));
            v.push(mvp * utility::to_vec4(&pos_buf[ind.z as usize]));
        }

        for vec in v.iter_mut() {
            *vec /= vec.w;
        }

        for vert in v.iter_mut() {
            vert.x = 0.5 * (self.width as f32) * (vert.x + 1.0);
            vert.y = 0.5 * (self.height as f32) * (vert.y + 1.0);
            vert.z = vert.z * f1 + f2;
        }

        let mut t = Triangle::new();
        for i in 0..3 {
            t.set_vertex(i, &v[i as usize].xyz());
            t.set_vertex(i, &v[i as usize].xyz());
            t.set_vertex(i, &v[i as usize].xyz());
        }

        t.set_color(0, 255.0, 0.0, 0.0);
        t.set_color(0, 0.0, 255.0, 0.0);
        t.set_color(0, 0.0, 0.0, 255.0);

        self.rasterize_wireframe(&t);
    }

    fn rasterize_wireframe(&mut self, t: &Triangle) {
        self.draw_line(&t.c(), &t.a());
        self.draw_line(&t.a(), &t.b());
        self.draw_line(&t.b(), &t.c());
    }

    fn rasterize_triangle(&mut self, t: &Triangle) {
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
        
        for x in lb.x .. rt.x {
            for y in lb.y .. rt.y {
                let _ok = inside_triangle(x, y, &t.v);
                if _ok {
                    let idx = self.get_index(x, y) as usize;
                    self.frame_buf[idx] = t.color[0];
                }
            }
        }
    }   

    fn draw_line(&mut self, begin: &glm::Vec3, end: &glm::Vec3) {
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
