#![allow(dead_code)]

use nalgebra_glm as glm;
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

impl Rasterizer {
    pub fn new(w: u32, h: u32) -> Rasterizer {
        Rasterizer {
            width: w,
            height: h,
            frame_buf: Vec::with_capacity((w * h) as usize),
            depth_buf: Vec::with_capacity((w * h) as usize),
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

    pub fn draw(&mut self, pos_id: PosBufId, ind_id: IndBufId, primitive_type: Primitive) {
        if primitive_type == Primitive::TRIANGLE {
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
            vert.x = 0.5 * self.width as f32 * (vert.x + 1.0);
            vert.y = 0.5 * self.height as f32 * (vert.y + 1.0);
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
