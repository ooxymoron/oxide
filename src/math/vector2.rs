use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn dot(&self, vec: Vector2) -> f32 {
        self.x * vec.x + self.y * vec.y
    }
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn zeroed() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }
}
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        *self = *self + rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
