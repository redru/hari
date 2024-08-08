use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CollisionRectangle {
    pub v1: Vec2,
    pub v2: Vec2,
    pub v3: Vec2,
    pub v4: Vec2,
    e1: Edge,
    e2: Edge,
    e3: Edge,
    e4: Edge,
}

impl CollisionRectangle {
    pub fn from_translation(xy: Vec2, width: f32, height: f32) -> Self {
        let half_width = width / 2.;
        let half_height = height / 2.;

        let v1 = Vec2::new(xy.x - half_width, xy.y + half_height); // Top Left
        let v2 = Vec2::new(xy.x + half_width, xy.y + half_height); // Top Right
        let v3 = Vec2::new(xy.x + half_width, xy.y - half_height); // Bottom Right
        let v4 = Vec2::new(xy.x - half_width, xy.y - half_height); // Bottom Left

        let e1 = Edge::new(v1.clone(), v2.clone()); // Top
        let e2 = Edge::new(v2.clone(), v3.clone()); // Right
        let e3 = Edge::new(v3.clone(), v4.clone()); // Bottom
        let e4 = Edge::new(v4.clone(), v1.clone()); // Left

        Self {
            v1,
            v2,
            v3,
            v4,
            e1,
            e2,
            e3,
            e4,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.v1 += offset;
        self.v2 += offset;
        self.v3 += offset;
        self.v4 += offset;

        self.e1.v1 += offset;
        self.e1.v2 += offset;
        self.e2.v1 += offset;
        self.e2.v2 += offset;
        self.e3.v1 += offset;
        self.e3.v2 += offset;
        self.e4.v1 += offset;
        self.e4.v2 += offset;

        self
    }
}

#[derive(Debug, Default)]
pub struct Edge {
    pub v1: Vec2,
    pub v2: Vec2,
}

impl Edge {
    pub fn new(v1: Vec2, v2: Vec2) -> Self {
        Self { v1, v2 }
    }
}

pub fn rectangles_collision_axis_aligned(
    rect1: &CollisionRectangle,
    rect2: &CollisionRectangle,
) -> bool {
    rect1.v1.x < rect2.v2.x
        && rect1.v2.x > rect2.v1.x
        && rect1.v1.y > rect2.v4.y
        && rect1.v4.y < rect2.v1.y
}
