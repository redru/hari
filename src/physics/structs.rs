use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CollisionRectangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
    pub v4: Vec2,
}

impl CollisionRectangle {
    pub fn from_translation(xy: Vec2, width: f32, height: f32) -> Self {
        let half_width = width / 2.;
        let half_height = height / 2.;

        Self {
            v1: Vec2::new(xy.x - half_width, xy.y + half_height), // Top Left
            v2: Vec2::new(xy.x + half_width, xy.y + half_height), // Top Right
            v3: Vec2::new(xy.x + half_width, xy.y - half_height), // Bottom Right
            v4: Vec2::new(xy.x - half_width, xy.y - half_height), // Bottom Left
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.v1 += offset;
        self.v2 += offset;
        self.v3 += offset;
        self.v4 += offset;

        self
    }
}
