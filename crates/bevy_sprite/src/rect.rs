use bevy_math::Vec2;
use bevy_reflect::Reflect;

/// A rectangle defined by two points. There is no defined origin, so 0,0 could be anywhere
/// (top-left, bottom-left, etc)
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, Reflect)]
pub struct Rect {
    /// The beginning point of the rect
    min: Vec2,
    /// The ending point of the rect
    max: Vec2,
}

impl Rect {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self {
            min: cmp::min(p1, p2),
            max: cmp::max(p1, p2),
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    /// Returns the minimum (first) point of this [`Rect`].
    pub fn min(&self) -> Vec2 {
        self.min
    }

    /// Returns the maximum (second) point of this [`Rect`].
    pub fn max(&self) -> Vec2 {
        self.max
    }
}
