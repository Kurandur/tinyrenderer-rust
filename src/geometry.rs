use std::fmt::{self, Display};
use std::ops::{Add, BitXor, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Copy,
{
    pub fn new(u: T, v: T) -> Self {
        Vec2 { x: u, y: v }
    }
    pub fn get(&self, idx: usize) -> T {
        match idx {
            0 => self.x,
            1 => self.y,
            _ => panic!("Vec2 index out of bounds"),
        }
    }
    pub fn set(&mut self, idx: usize, value: T) {
        match idx {
            0 => self.x = value,
            1 => self.y = value,
            _ => panic!("Vec2f index out of bounds"),
        }
    }
}

impl<T> Vec2<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

impl<T> Add for Vec2<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub for Vec2<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> fmt::Display for Vec2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    pub fn get(&self, index: usize) -> T {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index out of bounds for Vec3f"),
        }
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

impl Mul<f32> for Vec2<f32> {
    type Output = Vec2<f32>;
    fn mul(self, scalar: f32) -> Self::Output {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Mul<f32> for Vec2<i32> {
    type Output = Vec2<i32>;
    fn mul(self, scalar: f32) -> Self::Output {
        Vec2::new(
            (self.x as f32 * scalar) as i32,
            (self.y as f32 * scalar) as i32,
        )
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

impl<T> Mul<T> for Vec3<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Vec3<T>;
    fn mul(self, s: T) -> Vec3<T> {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl<T> BitXor for Vec3<T>
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
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

    pub fn normalize(&mut self)
    where
        T: From<f32>,
    {
        let n = self.norm();
        let inv = 1.0 / n;
        *self = Self::new(
            T::from(f32::from(self.x) * inv),
            T::from(f32::from(self.y) * inv),
            T::from(f32::from(self.z) * inv),
        );
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Mul<f32> for Vec3i {
    type Output = Vec3i;
    fn mul(self, scalar: f32) -> Self::Output {
        Vec3::new(
            (self.x as f32 * scalar) as i32,
            (self.y as f32 * scalar) as i32,
            (self.z as f32 * scalar) as i32,
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

impl From<Vec3<f32>> for Vec3<i32> {
    fn from(v: Vec3<f32>) -> Self {
        Vec3 {
            x: v.x as i32,
            y: v.y as i32,
            z: v.z as i32,
        }
    }
}

pub struct Matrix {
    m: Vec<Vec<f32>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            cols: cols,
            rows: rows,
            m: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn new_from_viewport(x: usize, y: usize, w: usize, h: usize) -> Self {
        let mut m = Matrix::identity(4);

        m[0][3] = x as f32 + w as f32 / 2.0;
        m[1][3] = y as f32 + h as f32 / 2.0;
        m[2][3] = 255.0 / 2.0;

        m[0][0] = w as f32 / 2.0;
        m[1][1] = h as f32 / 2.0;
        m[2][2] = 255.0 / 2.0;

        return m;
    }

    pub fn new_from_vector(v: Vec3f) -> Self {
        let mut m = Matrix::new(4, 1);
        m[0][0] = v.x;
        m[1][0] = v.y;
        m[2][0] = v.z;
        m[3][0] = 1.0;
        return m;
    }

    pub fn nrows(&self) -> usize {
        self.rows
    }

    pub fn ncols(&self) -> usize {
        self.cols
    }

    fn get(&self, index: usize) -> &Vec<f32> {
        &self.m[index]
    }

    fn get_mut(&mut self, index: usize) -> &mut Vec<f32> {
        &mut self.m[index]
    }

    pub fn identity(dimensions: usize) -> Matrix {
        let mut result = Matrix::new(dimensions, dimensions);

        for i in 0..dimensions {
            for j in 0..dimensions {
                result[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }

        result
    }

    pub fn to_vector(&self) -> Vec3f {
        return Vec3f {
            x: self[0][0] / self[3][0],
            y: self[1][0] / self[3][0],
            z: self[2][0] / self[3][0],
        };
    }

    pub fn zoom(factor: f32) -> Matrix {
        let mut z = Matrix::identity(4);
        z[0][0] = factor;
        z[1][1] = factor;
        z[2][2] = factor;
        return z;
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f32>;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, a: &Matrix) -> Self::Output {
        let mut result = Matrix::new(self.rows, a.cols);
        for i in 0..self.rows {
            for j in 0..a.cols {
                result[i][j] = 0.0;
                for k in 0..self.cols {
                    result[i][j] += self[i][k] * a[k][j];
                }
            }
        }
        result
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        &self * &rhs
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = vec![vec![String::new(); self.cols]; self.rows];
        let mut max_width = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let s = format!("{:.3}", self[i][j]);
                max_width = max_width.max(s.len());
                formatted[i][j] = s;
            }
        }

        for i in 0..self.rows {
            write!(f, "|")?;
            for j in 0..self.cols {
                write!(f, " {:>width$}", formatted[i][j], width = max_width)?;
            }
            writeln!(f, " |")?;
        }

        Ok(())
    }
}
