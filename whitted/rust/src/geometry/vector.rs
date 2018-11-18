use std::fmt::Debug;
use std::ops::*;
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, other: Vec2<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: SubAssign> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, other: Vec2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: Mul<Output = T>> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, other: T) -> Vec2<T> {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T: MulAssign> MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, other: Vec2<T>) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, other: T) -> Vec2<T> {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;
    fn neg(self) -> Vec2<T> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Sub<Output = T> + Copy> Vec2<T> {
    fn dot(&self, other: &Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    fn length2(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Out of Bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut T {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Out of Bounds"),
        }
    }
}

pub type Vec2f = Vec2<f64>;
pub type Vec2i = Vec2<i64>;

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: AddAssign> AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, other: Vec3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: SubAssign> SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, other: Vec3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: MulAssign> MulAssign<Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, other: Vec3<T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn div(self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Vec3<T> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Sub<Output = T> + Copy + Debug>
    Vec3<T>
{
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn cross_product(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -self.x * other.z + self.z * other.x,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of Bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut T {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of Bounds"),
        }
    }
}

pub type Vec3f = Vec3<f64>;

impl Vec3f {
    fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(&mut self) -> Vec3f {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
        *self
    }
}

/*******************************
Vec4
**********************************/

#[derive(Copy, Clone, Debug)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Add<Output = T>> Add<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn add(self, other: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: AddAssign> AddAssign<Vec4<T>> for Vec4<T> {
    fn add_assign(&mut self, other: Vec4<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl<T: Sub<Output = T>> Sub<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn sub(self, other: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<T: SubAssign> SubAssign<Vec4<T>> for Vec4<T> {
    fn sub_assign(&mut self, other: Vec4<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl<T: Mul<Output = T>> Mul<Vec4<T>> for Vec4<T> {
    type Output = Vec4<T>;
    fn mul(self, other: Vec4<T>) -> Vec4<T> {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec4<T> {
    type Output = Vec4<T>;
    fn mul(self, other: T) -> Vec4<T> {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl<T: MulAssign> MulAssign<Vec4<T>> for Vec4<T> {
    fn mul_assign(&mut self, other: Vec4<T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec4<T> {
    type Output = Vec4<T>;
    fn div(self, other: T) -> Vec4<T> {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec4<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Vec4<T>;
    fn neg(self) -> Vec4<T> {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec4<T> {
    fn dot(&self, other: &Vec4<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn length2(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl<T> Index<usize> for Vec4<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Out of Bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Vec4<T> {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut T {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Out of Bounds"),
        }
    }
}

pub type Vec4f = Vec4<f64>;

impl Vec4f {
    fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    fn normalize(&mut self) -> Vec4f {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        }
        *self
    }
}
