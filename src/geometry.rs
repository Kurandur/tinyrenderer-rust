use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub u: T,
    pub v: T,
}

impl<T> Vec2<T>
where
    T: Copy,
{
    pub fn new(u: T, v: T) -> Self {
        Vec2 { u, v }
    }
}

impl<T> Add for Vec2<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2::new(self.u + other.u, self.v + other.v)
    }
}

impl<T> Sub for Vec2<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2::new(self.u - other.u, self.v - other.v)
    }
}

impl<T> Mul<f32> for Vec2<T>
where
    T: Copy + Mul<f32, Output = T>,
{
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Vec2::new(self.u * scalar, self.v * scalar)
    }
}

impl<T> fmt::Display for Vec2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.u, self.v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> Add for Vec3<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Sub for Vec3<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Mul<f32> for Vec3<T>
where
    T: Copy + Mul<f32, Output = T>,
{
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl<T> Mul for Vec3<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    type Output = T;
    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Vec3<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    f32: From<T>,
{
    pub fn norm(&self) -> f32 {
        ((f32::from(self.x) * f32::from(self.x))
            + (f32::from(self.y) * f32::from(self.y))
            + (f32::from(self.z) * f32::from(self.z)))
        .sqrt()
    }

    pub fn normalize(&self) -> Self
    where
        T: From<f32>,
    {
        let n = self.norm();
        let inv = 1.0 / n;
        Self::new(
            T::from(f32::from(self.x) * inv),
            T::from(f32::from(self.y) * inv),
            T::from(f32::from(self.z) * inv),
        )
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T> fmt::Display for Vec3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec3f = Vec3<f32>;
pub type Vec3i = Vec3<i32>;
