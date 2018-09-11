use std::ops::Add;
use std::ops::Mul;
#[derive(Debug,Copy,Clone)]
struct Vec3<T>{
    x:T,
    y:T,
    z:T
}

impl<T:Mul<Output=T> + Add<Output=T>> Vec3<T>{
    fn dot(self,other : Vec3<T>) -> T{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn length2(self) -> T{
        self.x * self.x + self.y * self.y + self.z* self.z
    }
    fn length(self) -> T{
        self.length2().sqrt()
    }
}

impl<T :Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other : Vec3<T>) -> Vec3<T>{
        Vec3{
            x:self.x + other.x,
            y:self.y + other.y,
            z:self.z + other.z
        }
    }
}

impl <T : Mul<Output = T>>


fn main() {
    let point = Vec3{
        x:1,
        y:2,
        z:3
    };

    let point2 = point + point;
    println!("{:?}",point2);
    println!("{:?}",point.dot(point));
}
