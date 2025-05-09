use macroquad::math::IVec2;

#[derive(Debug, Clone, PartialEq)]
pub struct Monster {
    pub active: bool,
    pub chunk: IVec2,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub speed: f32,
}
