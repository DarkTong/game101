#![allow(dead_code)]

#[derive(Default)]
pub struct Triangle {
    pub v: [glm::Vec3; 3],
    pub color: [glm::Vec3; 3],
    pub tex_coords: [glm::Vec3; 3],
    pub normal: [glm::Vec3; 3],
    pub position: [glm::Vec3; 3],
    pub perp_pos: [glm::Vec4; 3],
}

impl Triangle {
    pub fn new() -> Triangle{
        Triangle{
            ..Default::default()
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

    pub fn set_vertex(&mut self, ind: usize, v: &glm::Vec3) {
        assert!(ind < 3);
        self.v[ind] = v.clone();
    }

    pub fn set_normal(&mut self, ind: usize, n: &glm::Vec3) {
        assert!(ind < 3);
        self.normal[ind] = n.clone();
    }

    pub fn set_tex_coord(&mut self, ind: usize, s: f32, t: f32) {
        assert!(ind < 3);
        self.tex_coords[ind] = glm::vec3(s, t, 1.0);
    }

    pub fn set_color(&mut self, ind: usize, r: f32, g: f32, b: f32) {
        assert!(ind < 3);
        self.color[ind] = glm::vec3(r, g, b);
    }

    pub fn set_position(&mut self, ind: usize, p: &glm::Vec3) {
        assert!(ind < 3);
        self.position[ind] = p.clone();
    }

    pub fn set_perp_pos(&mut self, ind:usize, p: &glm::Vec4) {
        assert!(ind < 4);
        self.perp_pos[ind] = p.clone();
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
