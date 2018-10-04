use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,MulAssign,Neg}
struct Vec3<T> {
    x:T,y:t,z:T
}

impl <T> Add<Vec3<T>> for Vec3<T>{
    type Output = Vec3<T>;
    fn add(self, other:Vec3<T>) -> Vec3<T>{
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z:self.z + other.z
        }
    }
}

