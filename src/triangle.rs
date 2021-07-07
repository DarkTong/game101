use nalgebra::*;
pub struct Triangle {
    pub v: [Vector3<f32>; 3],
    pub color: [Vector3<f32>; 3],
    pub tex_coords: [Vector2<f32>; 2],
    pub normal: [Vector3<f32>; 3],
}

impl Triangle {
    pub fn new() -> Triangle {
        let v = [Vector3::new(0f32, 0f32, 0f32); 3];
        let color = [Vector3::new(1f32, 1f32, 1f32); 3];
        let tex_coords = [Vector2::new(0f32, 0f32); 2];
        let normal = [Vector3::new(1f32, 0f32, 0f32); 3];

        Triangle {
            v,
            color,
            tex_coords,
            normal,
        }
    }

    pub fn a(&self) -> Vector3<f32> {
        self.v[0]
    }

    pub fn b(&self) -> Vector3<f32> {
        self.v[1]
    }

    pub fn c(&self) -> Vector3<f32> {
        self.v[2]
    }

    pub fn set_vector(&mut self, ind: u32, v: &Vector3<f32>) {
        assert!(ind >= 0 && ind < 3);
        self.v[ind as usize] = v.clone();
    }

    pub fn set_normal(&mut self, ind: u32, n: &Vector3<f32>) {
        assert!(ind >= 0 && ind < 3);
        self.normal[ind as usize] = n.clone();
    }

    pub fn set_tex_coord(&mut self, ind: u32, s: f32, t: f32) {
        assert!(ind >= 0 && ind < 3);
        self.tex_coords[ind as usize] = Vector2::new(s, t);
    }

    pub fn set_color(&mut self, ind: u32, r: f32, g:f32, b:f32) {
        assert!(ind >= 0 && ind < 3);
        self.color[ind as usize] = Vector3::new(r, g, b);
    }

    pub fn to_vector4(&self) -> [Vector4<f32>; 3] {
        let mut v3 = [Vector4::new(0f32, 0f32, 0f32, 0f32); 3];
        for ind in 0..3 {
            v3[ind].x = self.v[ind].x;
            v3[ind].y = self.v[ind].y;
            v3[ind].z = self.v[ind].z;
            v3[ind].w = 1f32;
        }

        v3
    }
}
