use std::ops::{Add,AddAssign,Sub,Mul,MulAssign,Neg};
#[derive(Debug,Copy,Clone,PartialEq,Default)]
struct Vec3{
    x:f64,
    y:f64,
    z:f64
}


impl Vec3{
    fn dot(self,other : Vec3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn length2(self) -> f64{
        self.dot(self)
    }
    fn length(self) -> f64
    {
        self.length2().sqrt()
    }
    fn normalize(&mut self){
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }
}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v : Vec3) -> Vec3{
        Vec3{
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, v : Vec3){
        *self = Vec3{
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v : Vec3) -> Vec3{
        Vec3{
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl  Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3{
        Vec3{
            x: self.x * t,
            y: self.y * t,
            z: self.z * t 
        }
    }
}

impl  Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3{
        Vec3{
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z 
        }

    }
}

impl MulAssign for Vec3{
    fn mul_assign(&mut self, v : Vec3) {
        *self = Vec3{
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }
}

impl  Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3{
            x: -self.x ,
            y: -self.y ,
            z: -self.z  
        }

    }
}
#[derive(Default,Debug,Copy,Clone,PartialEq)]
struct Sphere{
    center:Vec3,
    radius:f64,
    radius2:f64,
    surface_color:Vec3,
    emission_color:Vec3,
    transparency:f64,
    reflection:f64
}

impl Sphere{
    fn intersect(self,ray_origin:Vec3,ray_direction:Vec3,t0:&mut f64,t1:&mut f64) -> bool{
        let l = self.center - ray_origin;
        let tca = l.dot(ray_direction);
        if tca < 0.0 {
            return false
        }
        let d2 = l.length2() - tca * tca;
        if d2 > self.radius2{
            return false
        } else {
            let thc = (self.radius2 - d2 ).sqrt();
            //  t0 =  tca - thc;
            //  t1 =  tca + thc;
            return true
        }
    }
}



fn main() {
    let mut s = Sphere{
        center: Vec3 {x:0.0,y:0.0,z:0.0},
        radius:1.0,
        radius2:1.0,
        surface_color: Vec3{x:1.0,y:1.0,z:1.0},
        emission_color: Vec3{x:1.0,y:1.0,z:1.0},
        transparency:0.0,
        reflection:0.0
    };
    let mut t0 = 0.0;
    let mut t1 = 0.0;
    let ray_origin = Vec3{x:5.0,y:5.0,z:5.0};
    let ray_direction = Vec3{x:-1.0,y:-1.0,z:-1.0};

    // let i = s.intersect(ray_origin,ray_direction,t0,t1);
    println!("{:?}",s);
}
