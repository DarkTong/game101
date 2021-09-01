
pub fn to_vec4(v: &glm::Vec3, w: Option<f32>) -> glm::Vec4 {
    let _w = match w {
        Some(v) => v,
        None => 1.0f32,
    };
    glm::Vec4::new(v.x, v.y, v.z, _w)
}

pub fn draw_line(
    begin: &glm::Vec3,
    end: &glm::Vec3,
    mut f_action: Box<dyn FnMut(&glm::Vec3, &glm::Vec3) + '_>,
) {
    let x1: f32 = begin.x;
    let y1: f32 = begin.y;
    let x2: f32 = end.x;
    let y2: f32 = end.y;

    let line_color = glm::vec3(1.0f32, 1.0f32, 1.0f32);

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
        let point = glm::vec3(x, y, 1.0);
        f_action(&point, &line_color);
        while x < xe {
            x = x + 1.0;
            if px <= 0.0 {
                px = px + 2.0 * dy1;
            } else {
                if (dx < 0.0 && dy < 0.0) || (dx > 0.0 && dy > 0.0) {
                    y = y + 1.0;
                } else {
                    y = y - 1.0;
                }
                px = px + 2.0 * (dy1 - dx1);
            }

            let point = glm::vec3(x, y, 1.0);
            f_action(&point, &line_color);
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
        let point = glm::vec3(x, y, 1.0);
        f_action(&point, &line_color);
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
                py = py + 2.0 * (dx1 - dy1);
            }

            let point = glm::vec3(x, y, 1.0);
            f_action(&point, &line_color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_line() {
        let begin = glm::Vec3::new(0.0, 1.0, 0.0);
        let end = glm::Vec3::new(6.0, 4.0, 0.0);

        let mut result = Vec::new();

        draw_line(
            &begin,
            &end,
            Box::new(|point: &glm::Vec3, _| {
                result.push(point.clone());
                // println!("draw point: {:?}", point);
            }),
        );

        let check_result = [
            glm::Vec3::new(0.0, 1.0, 1.0),
            glm::Vec3::new(1.0, 1.0, 1.0),
            glm::Vec3::new(2.0, 2.0, 1.0),
            glm::Vec3::new(3.0, 2.0, 1.0),
            glm::Vec3::new(4.0, 3.0, 1.0),
            glm::Vec3::new(5.0, 3.0, 1.0),
            glm::Vec3::new(6.0, 4.0, 1.0),
        ];
        for (idx, p) in check_result.iter().enumerate() {
            assert_eq!(p, &result[idx]);
        }
    }
}
