mod geometry;
use geometry::matrix::*;
use geometry::objects::*;
use geometry::vector::*;

use std::f64::INFINITY;
static MAX_DEPTH: u8 = 5;

fn create_scene() -> Vec<Box<Object>> {
    let mut v: Vec<Box<Object>> = Vec::new();
    let s1 = Sphere::new(
        Vec3f::new(-1.0, 0.0, -12.0),
        2.0,
        None,
        None,
        None,
        None,
        Some(MaterialType::DIFFUSE_AND_GLOSSY),
    );
    v.push(Box::new(s1));
    v
}

fn trace<'a>(
    ray_origin: &Vec3f,
    ray_direction: &Vec3f,
    objects: Vec<&'a Object>,
    tnear: &mut f64,
    index: &mut usize,
    uv: &mut Vec2f,
) -> Option<&'a Object> {
    let mut closest_object: Option<&Object> = None;
    for object in objects.iter() {
        let mut tmp_near = INFINITY;
        if object.intersect(ray_origin, ray_direction, &mut tmp_near, index, uv) {
            if tmp_near < *tnear {
                *tnear = tmp_near;
                closest_object = Some(*object);
            }
        }
    }
    closest_object
}

fn cast_ray<'a>(
    ray_origin: &Vec3f,
    ray_direction: &Vec3f,
    objects: &Vec<&'a Object>,
    tnear: &mut f64,
    depth: u32,
) {
    // if (depth > MAX_DEPTH) {
    //     return background_color;
    // }
    // let tnear = INFINITY;
    // let mut uv = Vec2f { x: 0.0, y: 1.0 };
    // let index = 0;
    // if let Some(object) = trace(ray_origin, ray_direction, &mut tnear, &mut index, &mut uv) {
    //     let hit_point = *ray_origin + *ray_direction * tnear;
    //     let mut normal = Vec3f::zero();
    //     let mut st = Vec2f { x: 0.0, y: 1.0 };
    //     match(object.get_material_type()){
    //         _ => ();
    //     }
    // }
}

fn main() {
    // let t = Object::Triangle;
    // println!("{:?}",t.x);
    // let t = Triangle {
    //     x: Vec3f {
    //         x: 1.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     y: Vec3f {
    //         x: -1.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     z: Vec3f {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 2.0,
    //     },
    // };
    let mut v: Vec<&Sphere> = Vec::new();

    let c = Vec3f::new(0.0, 0.0, 0.0);
    // {
    let sc = Vec3f::new(0.5, 0.5, 0.5);
    let s = Sphere::new(c, 2.0, Some(sc), None, None, None, None);
    //     // println!("{:?}", c);
    //     // le t s = String::from("hello"); // s comes into scope
    // }
    {
        println!("{:?}", c);
    }
    // v.push(&s);
    // takes_ownership(c);

    // println!("{:?}", c);
    // println!("{:?}", sc);
    // println!("{:?}", v[0].center);

    // let s = Sphere {
    //     center: Vec3f {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     },
    //     radius: 2.0,
    //     surface_color: Vec3f {
    //         x: 1.0,
    //         y: 1.0,
    //         z: 1.0,
    //     },
    //     emission_color: Vec3f {
    //         x: 1.0,
    //         y: 1.0,
    //         z: 1.0,
    //     },
    //     transparency: 0.5,
    //     reflection: 0.5,
    // };
    // let mut v: Vec<&Object> = Vec::new();
    // // v.push(&t);
    // v.push(&s);
    // let ray_origin = Vec3f {
    //     x: 0.0,
    //     y: 2.0,
    //     z: 1.0,
    // };
    // let mut ray_direction = Vec3f {
    //     x: 0.0,
    //     y: -2.0,
    //     z: 0.0,
    // };
    // ray_direction.normalize();
    // let mut tnear = 1000.0;
    // let mut uv = Vec2f { x: 0.0, y: 1.0 };
    // let mut index = 0;
    // println!(
    //     "{:?} {:?} {:?}",
    //     // t.intersect(&ray_origin, &ray_direction, &mut tnear, &mut index, &mut uv),
    //     tnear,
    //     uv,
    //     2
    // );
}
fn takes_ownership(some_string: Vec3f) {
    // some_string comes into scope
    println!("{:?}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
