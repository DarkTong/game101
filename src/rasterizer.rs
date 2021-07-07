use nalgebra::*;
use std::collections::HashMap;

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

pub struct pos_buf_id(u32);
pub struct ind_buf_id(u32);

#[derive(Default)]
pub struct Rasterizer {
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    projection: Matrix4<f32>,

    pos_buf: HashMap<pos_buf_id, Vec<Vector3<f32>>>,
    ind_buf: HashMap<ind_buf_id, Vec<Vector3<f32>>>,

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
    pub fn set_model(&mut self, mat: &Matrix4<f32>) {
        self.model = mat.clone();
    }

    pub fn set_view(&mut self, mat: &Matrix4<f32>) {
        self.view = mat.clone();
    }

    pub fn set_projection(&mut self, mat: &Matrix4<f32>) {
        self.projection = mat.clone();
    }

    pub fn set_pixel(&mut self, point: &Vector2<f32>, color: &Vector3<f32>) {
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

    fn get_next_id(&self) -> u32 {
        self.next_id
    }

    fn get_index(&self, row: i32, col: i32) -> usize {
        assert!(row >= 0 && row < self.width as i32);
        assert!(col >= 0 && col < self.height as i32);

        ((self.height - col as u32) * self.width + row as u32) as usize
    }
}
