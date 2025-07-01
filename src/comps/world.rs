use std::collections::HashMap;

use macroquad::math::{Mat3, Vec2, Vec3Swizzles, vec2};

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub anchor_points: HashMap<String, Mat3>,
}

impl Transform {
    pub fn from_mat3(mat: Mat3) -> Self {
        let col_x = mat.x_axis.xy();
        let col_y = mat.y_axis.xy();

        let scale_x = col_x.length();
        let scale_y = col_y.length();

        // Evita divisão por zero
        let norm_x = if scale_x != 0.0 {
            col_x / scale_x
        } else {
            col_x
        };
        let rotation = norm_x.y.atan2(norm_x.x);

        let translation = mat.z_axis.xy();

        Self {
            position: translation,
            rotation: rotation,
            scale: Vec2::new(scale_x, scale_y),
            anchor_points: HashMap::new(),
        }
    }
    pub fn global_mat(&self) -> Mat3 {
        Mat3::from_translation(self.position)
            * Mat3::from_angle(self.rotation)
            * Mat3::from_scale(self.scale)
    }

    pub fn global_mat_of_anchor_point(&self, name: &str) -> Option<Mat3> {
        self.anchor_points
            .get(name)
            .map(|local_mat| self.global_mat() * *local_mat)
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: vec2(0.0, 0.0),
            scale: vec2(1.0, 1.0),
            rotation: 0.0,
            anchor_points: HashMap::new(),
        }
    }
}
