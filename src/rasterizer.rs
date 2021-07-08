use nalgebra::*;
use std::collections::HashMap;

use crate::triangle::*;

#[derive(Default, Clone, Copy)]
pub struct Buffer(u32);

impl Buffer {
    pub const Color: Self = Buffer(1);
    pub const Depth: Self = Buffer(2);
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

pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct pos_buf_id(u32);
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ind_buf_id(u32);

#[derive(Default)]
pub struct Rasterizer {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,

    pos_buf: HashMap<pos_buf_id, Vec<Vector3<f32>>>,
    ind_buf: HashMap<ind_buf_id, Vec<Vector3<u32>>>,

    frame_buf: Vec<Vector3<f32>>,
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

    pub fn load_position(&mut self, positions: Vec<Vector3<f32>>) -> pos_buf_id {
        let id = self.get_next_id();
        self.pos_buf.insert(pos_buf_id(id), positions);

        pos_buf_id(id)
    }

    pub fn load_indices(&mut self, indices: Vec<Vector3<u32>>) -> ind_buf_id {
        let id = self.get_next_id();
        self.ind_buf.insert(ind_buf_id(id), indices);

        ind_buf_id(id)
    }

    pub fn set_model(&mut self, mat: &Matrix4<f32>) {
        self.model = mat.clone();
    }

    pub fn set_view(&mut self, mat: &Matrix4<f32>) {
        self.view = mat.clone();
    }

    pub fn set_projection(&mut self, mat: &Matrix4<f32>) {
        self.projection = mat.clone();
    }

    pub fn set_pixel(&mut self, point: &Vector3<f32>, color: &Vector3<f32>) {
        let ind = self.get_index(point.x as i32, point.y as i32);
        self.frame_buf[ind] = color.clone();
    }

    pub fn clear(&mut self, buff: Buffer) {
        if (buff & Buffer::Color).0 != 0 {
            self.frame_buf
                .iter_mut()
                .map(|color| *color = Vector3::<f32>::zeros())
                .count();
        }
        if (buff & Buffer::Depth).0 != 0 {
            self.depth_buf
                .iter_mut()
                .map(|depth| *depth = f32::INFINITY)
                .count();
        }
    }

    pub fn frame_buf(&self) -> &Vec<Vector3<f32>> {
        &self.frame_buf
    }

    pub fn draw(&mut self) {}

    fn draw_line(&mut self, begin: &Vector3<f32>, end: &Vector3<f32>) {
        let x1: f32 = begin.x;
        let y1: f32 = begin.y;
        let x2: f32 = end.x;
        let y2: f32 = end.y;

        let line_color = Vector3::new(255f32, 255f32, 255f32);

        let mut x;
        let mut y;
        let xe;
        let ye;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2.0 * dy1 - dx1;
        let mut py = 2.0 * dx1 - dy1;

        if dy1 <= dx1 {
            if dx >= 0.0 {
                x = x1;
                y = y1;
                xe = x2;
            } else {
                x = x2;
                y = y2;
                xe = x1;
            }
            let point = Vector3::new(x, y, 1.0);
            self.set_pixel(&point, &line_color);
            while x < xe {
                x = x + 1.0;
                if px < 0.0 {
                    px = px + 2.0 * dy1;
                } else {
                    if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                        y = y + 1.0;
                    } else {
                        y = y - 1.0;
                    }
                    px = px + 2.0 * (dy1 - dx1);
                }

                let point = Vector3::new(x, y, 1.0);
                self.set_pixel(&point, &line_color);
            }
        } else {
            if dy >= 0.0 {
                x = x1;
                y = y1;
                ye = y2;
            } else {
                x = x2;
                y = y2;
                ye = y1;
            }
            let point = Vector3::new(x, y, 1.0);
            self.set_pixel(&point, &line_color);
            while y < ye {
                y = y + 1.0;
                if py <= 0.0 {
                    py = py + 2.0 * dx1;
                } else {
                    if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                        x = x + 1.0;
                    } else {
                        x = x - 1.0;
                    }
                }
                let point = Vector3::new(x, y, 1.0);
                self.set_pixel(&point, &line_color);
            }
        }
    }

    fn rasterize_wireframe(tri: &Triangle) {}

    fn get_next_id(&mut self) -> u32 {
        self.next_id += 1;
        self.next_id - 1
    }

    fn get_index(&self, row: i32, col: i32) -> usize {
        assert!(row >= 0 && row < self.width as i32);
        assert!(col >= 0 && col < self.height as i32);

        ((self.height - col as u32) * self.width + row as u32) as usize
    }
}
