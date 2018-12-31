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
    let mut index = 0;
    let (_tnear, _object) = trace(ray_origin, ray_direction, objects);
    if let Some(object) = _object {
        let tnear = _tnear;
        let hit_point = *ray_origin + *ray_direction * tnear;
        let normal = object.get_surface_property(&hit_point, ray_direction);
        let hit_color = match object.get_material_type() {
            MaterialType::REFLECTION_AND_REFRACTION => {
                let reflection_direction = reflect(ray_direction, &normal).normalize();
                let reflection_origin = if ray_direction.dot(&normal) < 0.0 {
                    hit_point + normal * options.bias
                } else {
                    //inside
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
                let mut refraction_direction =
                    refract(ray_direction, &normal, object.get_ior()).normalize();

                let refraction_color = if refraction_direction.length2() > 0.0 {
                    let refraction_origin = if ray_direction.dot(&normal) < 0.0 {
                        hit_point - normal * options.bias
                    } else {
                        hit_point + normal * options.bias
                    };

                    cast_ray(
                        &refraction_origin,
                        &refraction_direction.normalize(),
                        objects,
                        lights,
                        options,
                        depth + 1,
                    )
                } else {
                    Vec3f::new(0.0, 0.0, 0.0)
                };
                let kr = fresnel(ray_direction, &normal, object.get_ior());
                reflection_color * object.get_surface_color() * kr + refraction_color * (1.0 - kr)
            }
            MaterialType::REFLECTION => {
                // println!("hit second sphere");
                let reflection_direction = reflect(ray_direction, &normal).normalize();
                let reflection_origin = if ray_direction.dot(&normal) < 0.0 {
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

                reflection_color * object.get_surface_color()
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
        return hit_color;
    }
    // Vec3f::zero()
    // hit_color
    // else if ray_direction.dot(&Vec3f::new(0.0, 1.0, 0.0)) > 0.0 {
    //     let r = Vec3f::new(3.0, 3.0, 3.0) * ray_direction.dot(&Vec3f::new(0.0, 1.0, 0.0));
    //     // println!(" r is {:?}  {:?} ", r.x * 255.0, (r * 255.0).x as u8);
    //     return r + options.background_color;
    // } else {
    if depth >= 2 {
        println!("org {:?} dir {:?}", ray_origin, ray_direction);
    }
    return options.background_color;
    // }
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
            let color = cast_ray(&orig, &dir, objects, lights, options, 0) * 255.0;

            file.write_all(&[
                color.x.min(255.0) as u8,
                color.y.min(255.0) as u8,
                color.z.min(255.0) as u8,
            ])?;
        }
    }
    Ok(())
}

fn main() {
    let mut c1 = Vec3f::new(0.0, 0.0, -5.0);
    // {
    let sc1 = Vec3f::new(0.5, 0.5, 0.5);
    let mut s1 = Sphere::new(
        c1,
        2.0,
        ObjectAttributes {
            surface_color: Some(sc1),
            emission_color: None,
            transparency: None,
            reflection: None,
            material_type: Some(MaterialType::REFLECTION_AND_REFRACTION),
        },
    );

    let mut c2 = Vec3f::new(0.0, 0.0, -5.0);
    // {
    let sc2 = Vec3f::new(0.0, 1.0, 1.0);
    let mut s2 = Sphere::new(
        c2,
        2.0,
        ObjectAttributes {
            surface_color: Some(sc2),
            emission_color: None,
            transparency: None,
            reflection: None,
            material_type: Some(MaterialType::REFLECTION),
        },
    );
    let m3 = MeshTriangle::new(
        &vec![
            Vec3f::new(10.0, 10.0, 0.0),
            Vec3f::new(10.0, 10.0, -10.0),
            Vec3f::new(10.0, -10.0, -10.0),
            Vec3f::new(10.0, -10.0, 0.0),
        ],
        &vec![0, 1, 3, 1, 2, 3],
        2,
        &Vec2f { x: 1.0, y: 0.0 },
        ObjectAttributes {
            surface_color: Some(Vec3f::new(1.0, 0.0, 1.0)),
            emission_color: None,
            transparency: None,
            reflection: None,
            material_type: Some(MaterialType::REFLECTION),
        },
    );

    let options = Options {
        width: 640,
        height: 480,
        fov: 120.0,
        background_color: Vec3f::new(1.0, 1.0, 1.0),
        max_depth: 4,
        bias: 0.00001,
    };
    render(
        &options,
        &vec![/*Box::new(s1) ,*/ Box::new(s2), Box::new(m3)],
        &vec![],
    );
    // println!(refract(&Vec3f::new(), _normal: &Vec3f, ior: f64))
    // s1.intersect(&Vec3f::new(0.0, 0.0, -3.1), &Vec3f::new(0.0, 0.0, -1.0));
}
