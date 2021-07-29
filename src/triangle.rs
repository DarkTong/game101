#![allow(dead_code)]

pub struct Triangle {
    pub v: [glm::Vec3; 3],
    pub color: [glm::Vec3; 3],
    pub tex_coords: [glm::Vec2; 2],
    pub normal: [glm::Vec3; 3],
}

impl Triangle {
    pub fn new() -> Triangle {
        let v = [glm::vec3(0f32, 0f32, 0f32); 3];
        let color = [glm::vec3(1f32, 1f32, 1f32); 3];
        let tex_coords = [glm::vec2(0f32, 0f32); 2];
        let normal = [glm::vec3(1f32, 0f32, 0f32); 3];

        Triangle {
            v,
            color,
            tex_coords,
            normal,
        }
    }

    pub fn a(&self) -> glm::Vec3 {
        self.v[0]
    }

    pub fn b(&self) -> glm::Vec3 {
        self.v[1]
    }

    pub fn c(&self) -> glm::Vec3 {
        self.v[2]
    }

    pub fn set_vertex(&mut self, ind: u32, v: &glm::Vec3) {
        assert!(ind < 3);
        self.v[ind as usize] = v.clone();
    }

    pub fn set_normal(&mut self, ind: u32, n: &glm::Vec3) {
        assert!(ind < 3);
        self.normal[ind as usize] = n.clone();
    }

    pub fn set_tex_coord(&mut self, ind: u32, s: f32, t: f32) {
        assert!(ind < 3);
        self.tex_coords[ind as usize] = glm::vec2(s, t);
    }

    pub fn set_color(&mut self, ind: u32, r: f32, g: f32, b: f32) {
        assert!(ind < 3);
        self.color[ind as usize] = glm::vec3(r, g, b);
    }

    pub fn to_vector4(&self) -> [glm::Vec4; 3] {
        let mut v3 = [glm::vec4(0f32, 0f32, 0f32, 0f32); 3];
        for ind in 0..3 {
            v3[ind].x = self.v[ind].x;
            v3[ind].y = self.v[ind].y;
            v3[ind].z = self.v[ind].z;
            v3[ind].w = 1f32;
        }

        v3
    }
}
