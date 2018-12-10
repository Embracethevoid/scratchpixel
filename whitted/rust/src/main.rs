mod geometry;
use geometry::matrix::*;
use geometry::objects::*;
use geometry::vector::*;

use std::f64::INFINITY;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Options {
    width: u32,
    height: u32,
    fov: f64,
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
) -> (f64, Option<Box<Object>>) {
    let mut res: Option<Box<Object>> = None;
    let mut tnear = INFINITY;
    for object in objects.iter() {
        let (_intersect, _tnear, _object) = object.intersect(ray_origin, ray_direction);
        if _intersect && _tnear < tnear {
            tnear = _tnear;
            res = _object;
        }
    }
    (tnear, res)
}

fn cast_ray<'a>(
    ray_origin: &Vec3f,
    ray_direction: &Vec3f,
    objects: &Vec<Box<Object>>,
    lights: &Vec<Light>,
    options: &Options,
    depth: u32,
) -> Vec3f {
    if (depth > options.max_depth) {
        return options.background_color;
    }
    let mut tnear = INFINITY;
    let mut uv = Vec2f { x: 0.0, y: 1.0 };
    let mut index = 0;
    let (_tnear, _object) = trace(ray_origin, ray_direction, objects);
    if let Some(object) = _object {
        let tnear = _tnear;
        let hit_point = *ray_origin + *ray_direction * tnear;
        let normal = object.get_surface_property(&hit_point, ray_direction);
        let hit_color = match object.get_material_type() {
            MaterialType::REFLECTION_AND_REFRACTION => {
                let reflection_direction = reflect(ray_direction, &normal).normalize();
                let reflection_origin = if reflection_direction.dot(&normal) < 0.0 {
                    hit_point + normal * options.bias
                } else {
                    hit_point - normal * options.bias
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
                for light in lights {
                    let mut light_dir = light.position - hit_point;
                    let light_distance2 = light_dir.dot(&light_dir);
                    light_dir.normalize();
                }
                Vec3f::zero()
            }
        };
    }
    Vec3f::zero()
}

fn render(
    options: &Options,
    objects: &Vec<Box<Object>>,
    lights: &Vec<Light>,
) -> Result<(), std::io::Error> {
    let path = Path::new("./output.ppm");

    let display = path.display();

    let mut file = File::create(&path)?;

    file.write_all(format!("P6\n{} {}\n255\n", options.width, options.height).as_bytes())?;
    let image_aspect_ratio = options.width as f64 / options.height as f64;
    let scale = (options.fov * 0.5 / 180.0 * std::f64::consts::PI).tan();
    let orig = Vec3f::new(0.0, 0.0, 0.0);
    for j in 0..options.height {
        for i in 0..options.width {
            let x =
                (2.0 * (i as f64 + 0.5) / options.width as f64 - 1.0) * image_aspect_ratio * scale;
            let y = (1.0 - 2.0 * (j as f64 + 0.5) / options.height as f64) * scale;
            let dir = Vec3f::new(x, y, -1.0).normalize();
            let color = cast_ray(&orig, &dir, objects, lights, options, 0);
            file.write_all(&[color.x as u8, color.y as u8, color.z as u8])?;
        }
    }
    // for v in frame_buffer {
    //     let array = [v.x as u8, v.y as u8, v.z as u8];
    //     match file.write_all(&array) {
    //         Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
    //         Ok(_) => (),
    //     }
    // }
    Ok(())
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

    let mut c = Vec3f::new(0.0, 0.0, -10.0);
    // {
    let sc = Vec3f::new(0.5, 0.5, 0.5);
    let mut s = Sphere::new(
        c,
        2.0,
        ObjectAttributes {
            surface_color: Some(sc),
            emission_color: None,
            transparency: None,
            reflection: None,
            material_type: Some(MaterialType::RFLECTION),
        },
    );
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

    // let s = Sphere: {
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
    let options = Options {
        width: 1280,
        height: 960,
        fov: 90.0,
        background_color: Vec3f::new(0.235294, 0.67451, 0.843137),
        max_depth: 5,
        bias: 0.00001,
    };
    render(&options, &vec![Box::new(s)], &vec![]);
}
