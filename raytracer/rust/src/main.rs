use std::ops::{Add,AddAssign,Sub,Mul,MulAssign,Neg};
use std::cmp;
use std::io::prelude::*;
use std::fs::File;
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
#[derive(Debug,Copy,Clone,PartialEq)]
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
            let mut thc = (self.radius2 - d2 ).sqrt();
             *t0 = tca -  thc;
             *t1 =  tca + thc;
            return true
        }
    }
}

const MAX_DEPTH:u8 = 5;
fn trace(
    ray_origin:Vec3,
    ray_direction:Vec3,
    spheres:&[Sphere],
    depth:u8
) -> Vec3{
    if depth > MAX_DEPTH{
        return Vec3{
            x:2.0,y:2.0,z:2.0
        }
    }
    let mut s:Option<Sphere>= None;
    let mut tnear:f64 = 1e8;
    for sphere in spheres{
        let mut t0 = 0.0;
        let mut t1= 0.0;
        
        if sphere.intersect(ray_origin,ray_direction,&mut t0, &mut t1){
            s = Some(*sphere);
            tnear = if tnear < t0 { t0} else {tnear};
        }
    }
    if let Some(value) = s{
        let bias:f64 = 1e-4;
        let point_at_intersection = ray_direction * tnear + ray_origin;
        let mut normal_at_intersection = point_at_intersection - value.center;
        normal_at_intersection.normalize();
        let reflection_direciton = ray_direction - normal_at_intersection*ray_direction.dot(normal_at_intersection)*2.0;
        let reflection_origin = normal_at_intersection * bias + point_at_intersection;
        return value.surface_color * trace(reflection_origin,reflection_direciton,spheres,depth+1)
    } else {
        return Vec3{
            x:1.0,y:1.0,z:1.0
        }
    }
}

fn render(spheres:&[Sphere])-> std::io::Result<()>{
    let width:u16 = 640;
    let height:u16 = 480;
    let invWidth = 1.0 / width as f64;
    let invHeight = 1.0 / height as f64;
    let fov:f64 = 30.0;
    let aspect_ratio = width as f64 / height as f64;
    let angle = (std::f64::consts::PI * 0.5 * fov / 180.0).tan() ;
    let mut file = File::create("./self.ppm")?;
    let max_value:u8 = 255;
    file.write_fmt(format_args!("P6\n {} {}\n255\n",width,height))?;
    for y in 0..height{
        for x in 0..width{
            println!("x is {}",x);
            println!("y is {}",y);
            let xx = (2.0 * (x as f64 + 0.5)* invWidth);
            let yy = (1.0 - 2.0*((y as f64+ 0.5)* invHeight));
            let mut ray_dir = Vec3{
                x:xx,
                y:yy,
                z:-1.0
            };
            ray_dir.normalize();
            println!("{:?}",ray_dir);
            let pixel = trace(Vec3{x:0.0,y:0.0,z:0.0},ray_dir,spheres,0);
            file.write_fmt(format_args!("{}{}{}",cmp::min((pixel.x * 255.0) as u8,max_value) as char,cmp::min((pixel.y * 255.0) as u8,max_value) as char,cmp::min((pixel.z * 255.0) as u8,max_value) as char ))?;
        }
    }
    Ok(())
}

fn main() {
    let mut s = Sphere{
        center: Vec3 {x:0.0,y:0.0,z:-10.0},
        radius:2.0,
        radius2:4.0,
        surface_color: Vec3{x:1.0,y:0.5,z:1.0},
        emission_color: Vec3{x:1.0,y:1.0,z:1.0},
        transparency:0.0,
        reflection:0.0
    };
    let mut v = Vec::new();
    v.push(s);
    render(&v);
    // let mut t0 = 0.0;
    // let mut t1 = 0.0;
    // let ray_origin = Vec3{x:5.0,y:5.0,z:5.0};
    // let ray_direction = Vec3{x:-1.0,y:-1.0,z:-1.0};

    // let i = s.intersect(ray_origin,ray_direction,&mut t0,&mut t1);
    // let s:Option<Sphere> = None;
    // println!("{:?}",s);
}
