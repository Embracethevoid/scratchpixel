mod geometry;
use geometry::matrix::*;
use geometry::objects::*;
use geometry::vector::*;

use std::f64::INFINITY;
static MAX_DEPTH: u8 = 5;

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
    if (depth > MAX_DEPTH) {
        return background_color;
    }
}

fn main() {
    // let t = Object::Triangle;
    // println!("{:?}",t.x);
    let t = Triangle {
        x: Vec3f {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        y: Vec3f {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        z: Vec3f {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        },
    };
    let s = Sphere {
        center: Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        radius: 2.0,
        surface_color: Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        emission_color: Vec3f {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        transparency: 0.5,
        reflection: 0.5,
    };
    let mut v: Vec<&Object> = Vec::new();
    // v.push(&t);
    v.push(&s);
    let ray_origin = Vec3f {
        x: 0.0,
        y: 2.0,
        z: 1.0,
    };
    let mut ray_direction = Vec3f {
        x: 0.0,
        y: -2.0,
        z: 0.0,
    };
    ray_direction.normalize();
    let mut tnear = 1000.0;
    let mut uv = Vec2f { x: 0.0, y: 1.0 };
    let mut index = 0;
    println!(
        "{:?} {:?} {:?}",
        // t.intersect(&ray_origin, &ray_direction, &mut tnear, &mut index, &mut uv),
        tnear,
        uv,
        2
    );
}
