mod geometry;
use geometry::matrix::*;
use geometry::vector::*;

type Vec2i = Vec2<i32>;
type Vec2f = Vec2<f64>;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn compute_pixel_coordinates(
    p_world: &Vec3f,
    world_to_camera: Matrix44f,
    bottom: f64,
    left: f64,
    top: f64,
    right: f64,
    image_width: u32,
    image_height: u32,
    p_rapster: &mut Vec2i,
) -> bool {
    let p_camera = world_to_camera.mul_vec_matrix(*p_world);
    let p_screen = Vec2f {
        x: p_camera.x / -p_camera.z,
        y: p_camera.y / -p_camera.z,
    };
    let p_NDC = Vec2f {
        x: (p_screen.x + right) / (2.0 * right),
        y: (p_screen.y + top) / (2.0 * top),
    };
    let new_rapster = Vec2i {
        x: (p_NDC.x * (image_width as f64)) as i32,
        y: ((1.0 - p_NDC.y) * (image_height as f64)) as i32,
    };
    *p_rapster = new_rapster;
    let mut visible = true;
    if p_screen.x < left || p_screen.x > right || p_screen.x < bottom || p_screen.x > top {
        visible = false;
    }
    visible
}

enum fit_resolution_gate {
    k_fill,
    k_overscan,
}

fn main() {
    let num_tris = 51;
    let verts = vec![
        Vec3f {
            x: -2.5703,
            y: 0.78053,
            z: -2.4e-05,
        },
        Vec3f {
            x: -0.89264,
            y: 0.022582,
            z: 0.018577,
        },
        Vec3f {
            x: 1.6878,
            y: -0.017131,
            z: 0.022032,
        },
        Vec3f {
            x: 3.4659,
            y: 0.025667,
            z: 0.018577,
        },
        Vec3f {
            x: -2.5703,
            y: 0.78969,
            z: -0.001202,
        },
        Vec3f {
            x: -0.89264,
            y: 0.25121,
            z: 0.93573,
        },
        Vec3f {
            x: 1.6878,
            y: 0.25121,
            z: 1.1097,
        },
        Vec3f {
            x: 3.5031,
            y: 0.25293,
            z: 0.93573,
        },
        Vec3f {
            x: -2.5703,
            y: 1.0558,
            z: -0.001347,
        },
        Vec3f {
            x: -0.89264,
            y: 1.0558,
            z: 1.0487,
        },
        Vec3f {
            x: 1.6878,
            y: 1.0558,
            z: 1.2437,
        },
        Vec3f {
            x: 3.6342,
            y: 1.0527,
            z: 1.0487,
        },
        Vec3f {
            x: -2.5703,
            y: 1.0558,
            z: 0.0,
        },
        Vec3f {
            x: -0.89264,
            y: 1.0558,
            z: 0.0,
        },
        Vec3f {
            x: 1.6878,
            y: 1.0558,
            z: 0.0,
        },
        Vec3f {
            x: 3.6342,
            y: 1.0527,
            z: 0.0,
        },
        Vec3f {
            x: -2.5703,
            y: 1.0558,
            z: 0.001347,
        },
        Vec3f {
            x: -0.89264,
            y: 1.0558,
            z: -1.0487,
        },
        Vec3f {
            x: 1.6878,
            y: 1.0558,
            z: -1.2437,
        },
        Vec3f {
            x: 3.6342,
            y: 1.0527,
            z: -1.0487,
        },
        Vec3f {
            x: -2.5703,
            y: 0.78969,
            z: 0.001202,
        },
        Vec3f {
            x: -0.89264,
            y: 0.25121,
            z: -0.93573,
        },
        Vec3f {
            x: 1.6878,
            y: 0.25121,
            z: -1.1097,
        },
        Vec3f {
            x: 3.5031,
            y: 0.25293,
            z: -0.93573,
        },
        Vec3f {
            x: 3.5031,
            y: 0.25293,
            z: 0.0,
        },
        Vec3f {
            x: -2.5703,
            y: 0.78969,
            z: 0.0,
        },
        Vec3f {
            x: 1.1091,
            y: 1.2179,
            z: 0.0,
        },
        Vec3f {
            x: 1.145,
            y: 6.617,
            z: 0.0,
        },
        Vec3f {
            x: 4.0878,
            y: 1.2383,
            z: 0.0,
        },
        Vec3f {
            x: -2.5693,
            y: 1.1771,
            z: -0.081683,
        },
        Vec3f {
            x: 0.98353,
            y: 6.4948,
            z: -0.081683,
        },
        Vec3f {
            x: -0.72112,
            y: 1.1364,
            z: -0.081683,
        },
        Vec3f {
            x: 0.9297,
            y: 6.454,
            z: 0.0,
        },
        Vec3f {
            x: -0.7929,
            y: 1.279,
            z: 0.0,
        },
        Vec3f {
            x: 0.91176,
            y: 1.2994,
            z: 0.0,
        },
    ];

    let tris = vec![
        4, 0, 5, 0, 1, 5, 1, 2, 5, 5, 2, 6, 3, 7, 2, 2, 7, 6, 5, 9, 4, 4, 9, 8, 5, 6, 9, 9, 6, 10,
        7, 11, 6, 6, 11, 10, 9, 13, 8, 8, 13, 12, 10, 14, 9, 9, 14, 13, 10, 11, 14, 14, 11, 15, 17,
        16, 13, 12, 13, 16, 13, 14, 17, 17, 14, 18, 15, 19, 14, 14, 19, 18, 16, 17, 20, 20, 17, 21,
        18, 22, 17, 17, 22, 21, 18, 19, 22, 22, 19, 23, 20, 21, 0, 21, 1, 0, 22, 2, 21, 21, 2, 1,
        22, 23, 2, 2, 23, 3, 3, 23, 24, 3, 24, 7, 24, 23, 15, 15, 23, 19, 24, 15, 7, 7, 15, 11, 0,
        25, 20, 0, 4, 25, 20, 25, 16, 16, 25, 12, 25, 4, 12, 12, 4, 8, 26, 27, 28, 29, 30, 31, 32,
        34, 33,
    ];
    let focal_length: f64 = 35.0;
    let film_aperture_width = 0.825;
    let film_aperture_height = 0.446;
    let inch_to_mm = 25.4;
    let near_clipping_plane = 0.1;
    let far_clipping_plane = 1000.0;
    let image_width: u32 = 512;
    let image_height: u32 = 512;
    let fit_film = fit_resolution_gate::k_overscan;

    let film_aperture_ratio = film_aperture_width / film_aperture_height;
    let device_aspect_retio = image_width as f64 / image_height as f64;
    let mut top = (film_aperture_height * inch_to_mm) / 2.0 / focal_length * near_clipping_plane;
    let mut right = (film_aperture_width * inch_to_mm) / 2.0 / focal_length * near_clipping_plane;

    let mut x_scale = 1.0;
    let mut y_scale = 1.0;

    match fit_film {
        fit_resolution_gate::k_fill => {
            if film_aperture_ratio > device_aspect_retio {
                x_scale = device_aspect_retio / film_aperture_ratio;
            } else {
                y_scale = film_aperture_ratio / device_aspect_retio;
            }
        }
        fit_resolution_gate::k_overscan => {
            if film_aperture_ratio > device_aspect_retio {
                y_scale = film_aperture_ratio / device_aspect_retio;
            } else {
                x_scale = device_aspect_retio / film_aperture_ratio;
            }
        }
    }

    right *= x_scale;
    top *= y_scale;
    let bottom = -top;
    let left = -right;

    let camera_to_world = Matrix44f::new(
        -0.95424, 0.0, 0.299041, 0.0, 0.0861242, 0.95763, 0.274823, 0.0, -0.28637, 0.288002,
        -0.913809, 0.0, -3.734612, 7.610426, -14.152769, 1.0,
    );
    let world_to_camera = camera_to_world.inverse();
    let path = Path::new("./pinhole.svg");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`

    match file.write_all("<svg version=\"1.1\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xmlns=\"http://www.w3.org/2000/svg\" height=\"512\" width=\"512\">\n".as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => (),
    }
    for i in 0..num_tris {
        let v0_world = &verts[tris[i * 3]];
        let v1_world = &verts[tris[i * 3 + 1]];
        let v2_world = &verts[tris[i * 3 + 2]];
        let mut visible = true;
        let mut v0_raspter = Vec2i { x: 0, y: 0 };
        let mut v1_raspter = Vec2i { x: 0, y: 0 };
        let mut v2_raspter = Vec2i { x: 0, y: 0 };
        visible &= compute_pixel_coordinates(
            v0_world,
            world_to_camera,
            bottom,
            left,
            top,
            right,
            image_width,
            image_height,
            &mut v0_raspter,
        );
        visible &= compute_pixel_coordinates(
            v1_world,
            world_to_camera,
            bottom,
            left,
            top,
            right,
            image_width,
            image_height,
            &mut v1_raspter,
        );
        visible &= compute_pixel_coordinates(
            v2_world,
            world_to_camera,
            bottom,
            left,
            top,
            right,
            image_width,
            image_height,
            &mut v2_raspter,
        );
        let val = match visible {
            true => 0,
            false => 255,
        };
        match file.write_all(format!("<line x1=\"{}\" y1 = \"{}\" x2=\"{}\" y2 = \"{}\"   style=\"stroke:rgb(0,0,0);stroke-width:1\" />\n",v0_raspter.x ,v0_raspter.y , v1_raspter.x ,v1_raspter.y).as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
            },
            Ok(_) => (),
        }
        match file.write_all(format!("<line x1=\"{}\" y1 = \"{}\" x2=\"{}\" y2 = \"{}\"   style=\"stroke:rgb(0,0,0);stroke-width:1\" />\n",v1_raspter.x ,v1_raspter.y , v2_raspter.x ,v2_raspter.y).as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
            },
            Ok(_) => (),
        }
        match file.write_all(format!("<line x1=\"{}\" y1 = \"{}\" x2=\"{}\" y2 = \"{}\"   style=\"stroke:rgb(0,0,0);stroke-width:1\" />\n",v0_raspter.x ,v0_raspter.y , v2_raspter.x ,v2_raspter.y).as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
            },
            Ok(_) => (),
        }
    }

    match file.write_all("</svg>\n".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => (),
    }
    drop(file);
}
