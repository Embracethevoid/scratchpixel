mod geometry;
use geometry::matrix::*;
use geometry::objects::*;
use geometry::vector::*;

use std::f64::INFINITY;

struct Options {
    width: u32,
    height: u32,
    fov: f64,
    image_aspect_ratio: f64,
    max_depth: u32,
    background_color: Vec3f,
    bias: f64,
}

// fn create_scene() -> Vec<Box<Object>> {
//     let mut v: Vec<Box<Object>> = Vec::new();
//     let s1 = Sphere::new(
//         Vec3f::new(-1.0, 0.0, -12.0),
//         2.0,
//         None,
//         None,
//         None,
//         None,
//         Some(MaterialType::DIFFUSE_AND_GLOSSY),
//     );
//     v.push(Box::new(s1));
//     v
// }

fn trace<'a>(
    ray_origin: &Vec3f,
    ray_direction: &Vec3f,
    objects: &'a Vec<Box<Object>>,
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
                closest_object = Some(object.as_ref());
            }
        }
    }
    closest_object
}

fn cast_ray<'a>(
    ray_origin: &Vec3f,
    ray_direction: &Vec3f,
    objects: &Vec<Box<Object>>,
    lights: &Vec<Box<Light>>,
    options: &Options,
    depth: u32,
) -> Vec3f {
    if (depth > options.max_depth) {
        return options.background_color;
    }
    let mut tnear = INFINITY;
    let mut uv = Vec2f { x: 0.0, y: 1.0 };
    let mut index = 0;
    if let Some(object) = trace(
        ray_origin,
        ray_direction,
        objects,
        &mut tnear,
        &mut index,
        &mut uv,
    ) {
        let hit_point = *ray_origin + *ray_direction * tnear;
        let mut normal = Vec3f::zero();
        let mut st = Vec2f { x: 0.0, y: 1.0 };
        object.get_surface_property(&hit_point, ray_direction, index, &uv, &mut st, &mut normal);
        let hit_color = match object.get_material_type() {
            MaterialType::REFLECTION_AND_REFRACTION => {
                let reflection_direction = reflect(ray_direction, &normal).normalize();
                let reflection_origin = if reflection_direction.dot(&normal) < 0.0 {
                    hit_point + normal * options.bias
                } else {
                    hit_point _ normal * options.bias
                };
                let refraction_direction = refract(ray_direction, &normal, object.get_ior());
                let refraction_origin = if refraction_direction.dot(&normal) < 0.0 {
                    hit_point + normal * options.bias
                } else {
                    hit_point - normal * options.bias
                };
                let reflection_color = cast_ray(
                    &reflection_origin,
                    &reflection_direction,
                    objects,
                    lights,
                    options,
                    depth + 1,
                );

                let refraction_color = cast_ray(
                    &refraction_origin,
                    &refraction_direction,
                    objects,
                    lights,
                    options,
                    depth + 1,
                );
                let kr = fresnel(ray_direction, &normal, object.get_ior());
                reflection_color * kr + refraction_color * (1.0 - kr)
            }
            MaterialType::RFLECTION => {
                let reflection_direction = reflect(ray_direction, &normal).normalize();
                let reflection_origin = if reflection_direction.dot(&normal) < 0.0 {
                    hit_point + normal * options.bias
                } else {
                    hit_point - normal * options.bias
                };

                let reflection_color = cast_ray(
                    &reflection_origin,
                    &reflection_direction,
                    objects,
                    lights,
                    options,
                    depth + 1,
                );

                let kr = fresnel(ray_direction, &normal, object.get_ior());
                reflection_color * kr
            }
            _ => {
                let light_amount = Vec3f::zero();
                let specular_color = Vec3f::zero();
                let shadow_point_origin = if ray_direction.dot(&normal) < 0.0 {
                    hit_point + normal * options.bias
                } else {
                    hit_point - normal * options.bias
                };
                for light in lights{
                    let mut light_dir = light.position - hit_point;
                    let light_distance2 = light_dir.dot(light_dir);
                    light_dir.normalize();
                }
                Vec3f::zero()
            }
        };
    }
    Vec3f::zero()
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
    // let mut v: Vec<&Sphere> = Vec::new();

    let mut c = Vec3f::new(0.0, 0.0, 0.0);
    // {
    let sc = Vec3f::new(0.5, 0.5, 0.5);
    // let mut s = Sphere::new(c, 2.0, ObjectAttributes{ Some(sc), None, None, None, None});
    // s.center = Vec3f::new(1.0, 1.0, 1.0);
    // //     // println!("{:?}", c);
    // //     // le t s = String::from("hello"); // s comes into scope
    // // }
    // println!("{:?}", c);
    // println!("{:?}", s.center);
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
