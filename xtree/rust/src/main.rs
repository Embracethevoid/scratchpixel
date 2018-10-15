
mod geometry;
use geometry::vector::*;
use geometry::matrix::*;

type Vec2i = Vec2<i32>;
type Vec2f = Vec2<f64>;
fn compute_pixel_coordinates(
    p_world:&Vec3f,
    world_to_camera:Matrix44f,
    canvas_width:f64,
    canvas_height:f64,
    image_width:i32,
    image_height:i32
) -> Vec2i{
    let p_camera = world_to_camera.mul_vec_matrix(*p_world);
    let p_screen: Vec2f = Vec2f{
        x:p_camera.x / -p_camera.z,
        y:p_camera.y / -p_camera.z
    };
    let p_NDC = Vec2f{
        x: (p_screen.x + canvas_width * 0.5 ) / canvas_width,
        y: (p_screen.y + canvas_height * 0.5) / canvas_height
    };
    println!("{:?}",p_NDC );
    Vec2i{
        x:(p_NDC.x  * (image_width as f64) )as i32 ,
        y:((1.0 - p_NDC.y)  * (image_height as f64)) as i32
    }
}
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let num_tris = 128;
    let tris = vec![8, 7, 9, 6, 5, 7, 4, 3, 5, 2, 1, 3, 0, 9, 1,
5, 3, 7, 7, 3, 9, 9, 3, 1, 10, 12, 11, 13, 15, 14,
15, 13, 16, 13, 17, 16, 18, 20, 19, 17, 20, 21, 20, 23, 22,
20, 24, 23, 23, 26, 25, 24, 26, 23, 26, 27, 25, 26, 28, 27,
27, 30, 29, 28, 30, 27, 30, 32, 31, 30, 33, 32, 27, 30, 34,
32, 36, 35, 33, 36, 32, 36, 38, 37, 36, 39, 38, 38, 41, 40,
39, 41, 38, 41, 43, 42, 41, 44, 43, 44, 45, 43, 44, 47, 46,
44, 48, 47, 48, 49, 47, 48, 51, 50, 10, 52, 12, 13, 53, 54,
55, 17, 54, 13, 54, 17, 56, 57, 20, 17, 58, 20, 20, 59, 60,
20, 60, 24, 60, 61, 26, 24, 60, 26, 26, 61, 62, 26, 62, 28,
62, 63, 30, 28, 62, 30, 30, 64, 65, 30, 65, 33, 62, 66, 30,
65, 67, 36, 33, 65, 36, 36, 68, 69, 36, 69, 39, 69, 70, 41,
39, 69, 41, 41, 71, 72, 41, 72, 44, 44, 72, 73, 44, 74, 75,
44, 75, 48, 48, 75, 76, 48, 77, 51, 78, 80, 79, 81, 83, 82,
83, 81, 84, 81, 85, 84, 86, 88, 87, 85, 88, 89, 88, 91, 90,
88, 92, 91, 91, 94, 93, 92, 94, 91, 94, 95, 93, 94, 96, 95,
95, 98, 97, 96, 98, 95, 98, 100, 99, 98, 101, 100, 95, 98, 102,
100, 104, 103, 101, 104, 100, 104, 106, 105, 104, 107, 106, 106, 109, 108,
107, 109, 106, 109, 111, 110, 109, 112, 111, 112, 113, 111, 112, 115, 114,
112, 116, 115, 116, 117, 115, 116, 119, 118, 78, 120, 80, 81, 121, 122,
123, 85, 122, 81, 122, 85, 124, 125, 88, 85, 126, 88, 88, 127, 128,
88, 128, 92, 128, 129, 94, 92, 128, 94, 94, 129, 130, 94, 130, 96,
130, 131, 98, 96, 130, 98, 98, 132, 133, 98, 133, 101, 130, 134, 98,
133, 135, 104, 101, 133, 104, 104, 136, 137, 104, 137, 107, 137, 138, 109,
107, 137, 109, 109, 139, 140, 109, 140, 112, 112, 140, 141, 112, 142, 143,
112, 143, 116, 116, 143, 144, 116, 145, 119 ];

let verts:Vec<Vec3f> = vec![
    Vec3f{ x:   0.0 ,y: 39.034, z: 0.0 },
Vec3f{ x:  0.76212 ,y: 36.843, z: 0.0 },
Vec3f{ x: 3.0 ,y: 36.604, z: 0.0 },
Vec3f{ x:  1.0 ,y: 35.604, z: 0.0 },
Vec3f{ x: 2.0162 ,y: 33.382, z: 0.0 },
Vec3f{ x:  0.0 ,y: 34.541, z: 0.0 },
Vec3f{ x: -2.0162 ,y: 33.382, z: 0.0 },
Vec3f{ x:  -1.0,y: 35.604, z: 0.0 },
Vec3f{ x: -3.0 ,y: 36.604, z: 0.0 },
Vec3f{ x:  -0.76212 ,y: 36.843, z: 0.0 },
Vec3f{ x:-0.040181 ,y: 34.31, z: 0.0 },
Vec3f{ x:  3.2778 ,y: 30.464, z: 0.0 },
Vec3f{ x:-0.040181 ,y: 30.464, z: 0.0 },
Vec3f{ x: -0.028749 ,y: 30.464, z: 0.0 },
Vec3f{ x: 3.2778 ,y: 30.464, z: 0.0 },
Vec3f{ x:  1.2722 ,y: 29.197, z: 0.0 },
Vec3f{ x: 1.2722 ,y: 29.197, z: 0.0 },
Vec3f{ x: -0.028703 ,y: 29.197, z: 0.0 },
Vec3f{ x: 1.2722 ,y: 29.197, z: 0.0 },
Vec3f{ x:  5.2778 ,y: 25.398, z: 0.0 },
Vec3f{ x: -0.02865 ,y: 25.398, z: 0.0 },
Vec3f{ x:  1.2722 ,y: 29.197, z: 0.0 },
Vec3f{ x: 5.2778 ,y: 25.398, z: 0.0 },
Vec3f{ x:  3.3322 ,y: 24.099, z: 0.0 },
Vec3f{ x:-0.028683 ,y: 24.099, z: 0.0 },
Vec3f{ x:  7.1957 ,y: 20.299, z: 0.0 },
Vec3f{ x: -0.02861 ,y: 20.299, z: 0.0 },
Vec3f{ x:  5.2778 ,y: 19.065, z: 0.0 },
Vec3f{ x:-0.028663 ,y: 18.984, z: 0.0 },
Vec3f{ x:  9.2778 ,y: 15.265, z: 0.0 },
Vec3f{ x:-0.028571 ,y: 15.185, z: 0.0 },
Vec3f{ x:  9.2778 ,y: 15.265, z: 0.0 },
Vec3f{ x: 7.3772 ,y: 13.999, z: 0.0 },
Vec3f{ x: -0.028625 ,y: 13.901, z: 0.0 },
Vec3f{ x: 9.2778 ,y: 15.265, z: 0.0 },
Vec3f{ x:  12.278 ,y: 8.9323, z: 0.0 },
Vec3f{ x:-0.028771 ,y: 8.9742, z: 0.0 },
Vec3f{ x:  12.278 ,y: 8.9323, z: 0.0 },
Vec3f{ x: 10.278 ,y: 7.6657, z: 0.0 },
Vec3f{ x: -0.028592 ,y: 7.6552, z: 0.0 },
Vec3f{ x: 15.278 ,y: 2.5994, z: 0.0 },
Vec3f{ x: -0.028775 ,y: 2.6077, z: 0.0 },
Vec3f{ x: 15.278 ,y: 2.5994, z: 0.0 },
Vec3f{ x:  13.278 ,y: 1.3329, z: 0.0 },
Vec3f{ x:-0.028727 ,y: 1.2617, z: 0.0 },
Vec3f{ x:  18.278 ,y: -3.7334, z: 0.0 },
Vec3f{ x: 18.278 ,y: -3.7334, z: 0.0 },
Vec3f{ x:  2.2722 ,y: -1.2003, z: 0.0 },
Vec3f{ x:-0.028727 ,y: -1.3098, z: 0.0 },
Vec3f{ x:  4.2722 ,y: -5.0, z: 0.0 },
Vec3f{ x: 4.2722 ,y: -5.0, z: 0.0 },
Vec3f{ x: -0.028727 ,y: -5.0, z: 0.0 },
Vec3f{ x: -3.3582 ,y: 30.464, z: 0.0 },
Vec3f{ x:  -3.3582 ,y: 30.464, z: 0.0 },
Vec3f{ x: -1.3526 ,y: 29.197, z: 0.0 },
Vec3f{ x:  -1.3526 ,y: 29.197, z: 0.0 },
Vec3f{ x: -1.3526 ,y: 29.197, z: 0.0 },
Vec3f{ x:  -5.3582 ,y: 25.398, z: 0.0 },
Vec3f{ x: -1.3526 ,y: 29.197, z: 0.0 },
Vec3f{ x:  -5.3582 ,y: 25.398, z: 0.0 },
Vec3f{ x: -3.4126 ,y: 24.099, z: 0.0 },
Vec3f{ x:  -7.276 ,y: 20.299, z: 0.0 },
Vec3f{ x: -5.3582 ,y: 19.065, z: 0.0 },
Vec3f{ x:  -9.3582 ,y: 15.265, z: 0.0 },
Vec3f{ x: -9.3582 ,y: 15.265, z: 0.0 },
Vec3f{ x:  -7.4575 ,y: 13.999, z: 0.0 },
Vec3f{ x: -9.3582 ,y: 15.265, z: 0.0 },
Vec3f{ x:  -12.358 ,y: 8.9323, z: 0.0 },
Vec3f{ x: -12.358 ,y: 8.9323, z: 0.0 },
Vec3f{ x:  -10.358 ,y: 7.6657, z: 0.0 },
Vec3f{ x: -15.358 ,y: 2.5994, z: 0.0 },
Vec3f{ x:  -15.358 ,y: 2.5994, z: 0.0 },
Vec3f{ x: -13.358 ,y: 1.3329, z: 0.0 },
Vec3f{ x:  -18.358 ,y: -3.7334, z: 0.0 },
Vec3f{ x: -18.358 ,y: -3.7334, z: 0.0 },
Vec3f{ x:  -2.3526 ,y: -1.2003, z: 0.0 },
Vec3f{ x: -4.3526 ,y: -5.0, z: 0.0 },
Vec3f{ x:  -4.3526 ,y: -5.0, z: 0.0 },
Vec3f{ x: 0.0 ,y: 34.31, z: 0.040181 },
Vec3f{ x:  0.0 ,y: 30.464, z: -3.2778 },
Vec3f{ x: 0.0 ,y: 30.464, z: 0.040181 },
Vec3f{ x:  0.0 ,y: 30.464, z: 0.028749 },
Vec3f{ x: 0.0 ,y: 30.464, z: -3.2778 },
Vec3f{ x:  0.0 ,y: 29.197, z: -1.2722 },
Vec3f{ x: 0.0 ,y: 29.197, z: -1.2722 },
Vec3f{ x:  0.0 ,y: 29.197, z: 0.028703 },
Vec3f{ x: 0.0 ,y: 29.197, z: -1.2722 },
Vec3f{ x:  0.0 ,y: 25.398, z: -5.2778 },
Vec3f{ x: 0.0 ,y: 25.398, z: 0.02865 },
Vec3f{ x:  0.0 ,y: 29.197, z: -1.2722 },
Vec3f{ x: 0.0 ,y: 25.398, z: -5.2778 },
Vec3f{ x:  0.0 ,y: 24.099, z: -3.3322 },
Vec3f{ x: 0.0 ,y: 24.099, z: 0.028683 },
Vec3f{ x:  0.0 ,y: 20.299, z: -7.1957 },
Vec3f{ x: 0.0 ,y: 20.299, z: 0.02861 },
Vec3f{ x:  0.0 ,y: 19.065, z: -5.2778 },
Vec3f{ x: 0.0 ,y: 18.984, z: 0.028663 },
Vec3f{ x:  0.0 ,y: 15.265, z: -9.2778 },
Vec3f{ x: 0.0 ,y: 15.185, z: 0.028571 },
Vec3f{ x:  0.0 ,y: 15.265, z: -9.2778 },
Vec3f{ x: 0.0 ,y: 13.999, z: -7.3772 },
Vec3f{ x:  0.0 ,y: 13.901, z: 0.028625 },
Vec3f{ x: 0.0 ,y: 15.265, z: -9.2778 },
Vec3f{ x:  0.0 ,y: 8.9323, z: -12.278 },
Vec3f{ x: 0.0 ,y: 8.9742, z: 0.028771 },
Vec3f{ x:  0.0 ,y: 8.9323, z: -12.278 },
Vec3f{ x: 0.0 ,y: 7.6657, z: -10.278 },
Vec3f{ x:  0.0 ,y: 7.6552, z: 0.028592 },
Vec3f{ x: 0.0 ,y: 2.5994, z: -15.278 },
Vec3f{ x:  0.0 ,y: 2.6077, z: 0.028775 },
Vec3f{ x: 0.0 ,y: 2.5994, z: -15.278 },
Vec3f{ x:  0.0 ,y: 1.3329, z: -13.278 },
Vec3f{ x: 0.0 ,y: 1.2617, z: 0.028727 },
Vec3f{ x:  0.0 ,y: -3.7334, z: -18.278 },
Vec3f{ x: 0.0 ,y: -3.7334, z: -18.278 },
Vec3f{ x:  0.0 ,y: -1.2003, z: -2.2722 },
Vec3f{ x: 0.0 ,y: -1.3098, z: 0.028727 },
Vec3f{ x:  0.0 ,y: -5.0, z: -4.2722 },
Vec3f{ x: 0.0 ,y: -5.0, z: -4.2722 },
Vec3f{ x:  0.0 ,y: -5.0, z: 0.028727 },
Vec3f{ x: 0.0 ,y: 30.464, z: 3.3582 },
Vec3f{ x:  0.0 ,y: 30.464, z: 3.3582 },
Vec3f{ x: 0.0 ,y: 29.197, z: 1.3526 },
Vec3f{ x:  0.0 ,y: 29.197, z: 1.3526 },
Vec3f{ x: 0.0 ,y: 29.197, z: 1.3526 },
Vec3f{ x:  0.0 ,y: 25.398, z: 5.3582 },
Vec3f{ x: 0.0 ,y: 29.197, z: 1.3526 },
Vec3f{ x:  0.0 ,y: 25.398, z: 5.3582 },
Vec3f{ x: 0.0 ,y: 24.099, z: 3.4126 },
Vec3f{ x:  0.0 ,y: 20.299, z: 7.276 },
Vec3f{ x: 0.0 ,y: 19.065, z: 5.3582 },
Vec3f{ x:  0.0 ,y: 15.265, z: 9.3582 },
Vec3f{ x: 0.0 ,y: 15.265, z: 9.3582 },
Vec3f{ x:  0.0 ,y: 13.999, z: 7.4575 },
Vec3f{ x: 0.0 ,y: 15.265, z: 9.3582 },
Vec3f{ x:  0.0 ,y: 8.9323, z: 12.358 },
Vec3f{ x: 0.0 ,y: 8.9323, z: 12.358 },
Vec3f{ x:  0.0 ,y: 7.6657, z: 10.358 },
Vec3f{ x: 0.0 ,y: 2.5994, z: 15.358 },
Vec3f{ x:  0.0 ,y: 2.5994, z: 15.358 },
Vec3f{ x: 0.0 ,y: 1.3329, z: 13.358 },
Vec3f{ x:  0.0 ,y: -3.7334, z: 18.358 },
Vec3f{ x: 0.0 ,y: -3.7334, z: 18.358 },
Vec3f{ x:  0.0 ,y: -1.2003, z: 2.3526 },
Vec3f{ x: 0.0 ,y: -5.0, z: 4.3526 },
Vec3f{ x:  0.0 ,y: -5.0, z: 4.3526  }
];
    let camera_to_world = Matrix44f::new(
        0.871214, 0.0, -0.490904, 0.0, 
        -0.192902, 0.919559, -0.342346, 0.0, 
        0.451415, 0.392953, 0.801132, 0.0, 
        14.777467, 29.361945, 27.993464, 1.0
    );



    let path = Path::new("./proj.svg");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
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

    let world_to_camera = camera_to_world.inverse();
    let canvas_width = 2.0;
    let canvas_height = 2.0;
    let image_width = 512;
    let image_height = 512;
    for i in 0..num_tris{
        let v0_world = &verts[tris[i*3]];
        let v1_world = &verts[tris[i*3+1]];
        let v2_world = &verts[tris[i*3+2]];
        let v0_raspter = compute_pixel_coordinates(v0_world, world_to_camera, canvas_width, canvas_height, image_width, image_height);
        let v1_raspter = compute_pixel_coordinates(v1_world, world_to_camera, canvas_width, canvas_height, image_width, image_height);
        let v2_raspter = compute_pixel_coordinates(v2_world, world_to_camera, canvas_width, canvas_height, image_width, image_height);
        println!(" v0 is {:?} v1 is {:?} v2 : {:?}",v0_raspter ,v1_raspter ,v2_raspter  );
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
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
            },
            Ok(_) => (),
        }
}
