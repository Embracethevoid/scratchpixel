mod geometry;
use geometry::matrix::*;
use geometry::vector::*;

mod cow;
use cow::*;

use std::cmp::max;
use std::cmp::min;

const inch_to_mm: f64 = 25.4;

enum fit_resolution_gate {
    KFill,
    KOverscan,
}
fn compute_screen_coordinations(
    film_aperture_width: f64,
    film_aperture_height: f64,
    image_width: u32,
    image_height: u32,
    fit_film: fit_resolution_gate,
    near_clipping_plane: f64,
    focal_length: f64,
    top: &mut f64,
    bottom: &mut f64,
    left: &mut f64,
    right: &mut f64,
) {
    let film_aspect_ratio = film_aperture_width / film_aperture_height;
    let device_aspect_ratio = image_width as f64 / image_height as f64;

    *top = film_aperture_height * inch_to_mm / 2.0 / focal_length / near_clipping_plane;
    *right = film_aperture_width * inch_to_mm / 2.0 / focal_length / near_clipping_plane;

    let mut x_scale: f64 = 1.0;
    let mut y_scale: f64 = 1.0;
    match fit_film {
        KFill => {
            if film_aspect_ratio > device_aspect_ratio {
                x_scale = device_aspect_ratio / film_aspect_ratio;
            } else {
                y_scale = film_aspect_ratio / device_aspect_ratio;
            }
        }
        KOverscan => {
            if film_aspect_ratio < device_aspect_ratio {
                x_scale = device_aspect_ratio / film_aspect_ratio;
            } else {
                y_scale = film_aspect_ratio / device_aspect_ratio;
            }
        }
    };
    *right *= x_scale;
    *top *= y_scale;

    *left = -*right;
    *bottom = -*top;
}

fn convert_to_raster(
    vertex_world: &Vec3f,
    world_to_camera: &Matrix44f,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
    near: f64,
    image_width: u32,
    image_height: u32,
) -> Vec3f {
    let vertex_camera = world_to_camera.mul_vec_matrix(vertex_world);
    let vertex_screen = Vec2f {
        x: near * vertex_camera.x / -vertex_camera.z,
        y: near * vertex_camera.y / -vertex_camera.z,
    };
    let vertex_NDC = Vec2f {
        x: 2.0 * vertex_screen.x / (right - left) - (right + left) / (right - left),
        y: 2.0 * vertex_screen.y / (top - bottom) - (top + bottom) / (top - bottom),
    };
    Vec3f {
        x: (vertex_NDC.x + 1.0) / 2.0 * image_width as f64,
        y: (1.0 - vertex_NDC.y) / 2.0 * image_height as f64,
        z: -vertex_camera.z,
    }
}

fn min3(x: f64, y: f64, z: f64) -> f64 {
    x.min(y).min(z)
}

fn max3(x: f64, y: f64, z: f64) -> f64 {
    x.max(y).max(z)
}

fn edge_function(a: &Vec3f, b: &Vec3f, c: &Vec3f) -> f64 {
    (c[0] - a[0]) * (b[1] - a[1]) - (c[1] - a[1]) * (b[0] - a[0])
}

fn main() {
    let image_width: u32 = 640;
    let image_height: u32 = 480;
    let world_to_camera: Matrix44f = Matrix44f::new(
        0.707107, -0.331295, 0.624695, 0.0, 0.0, 0.883452, 0.468521, 0.0, -0.707107, -0.331295,
        0.624695, 0.0, -1.63871, -5.747777, -40.400412, 1.0,
    );
    let ntris = 3156;
    let near_clipping_plane = 1.0;
    let far_clipping_plane = 1000.0;
    let focal_length = 20.0;
    let film_aperture_width = 0.980;
    let film_aperture_height = 0.735;

    let camera_to_world = world_to_camera.inverse();

    let (mut top, mut bottom, mut left, mut right) = (0.0, 0.0, 0.0, 0.0);
    compute_screen_coordinations(
        film_aperture_width,
        film_aperture_height,
        image_width,
        image_height,
        fit_resolution_gate::KOverscan,
        near_clipping_plane,
        focal_length,
        &mut top,
        &mut bottom,
        &mut left,
        &mut right,
    );

    let mut frame_buffer = vec![
        Vec3f {
            x: 255.0,
            y: 255.0,
            z: 255.0,
        };
        (image_width * image_height) as usize
    ];
    let mut depth_buffer = vec![far_clipping_plane; (image_width * image_height) as usize];
    for i in 0..ntris {
        let v0 = vertices[nvertices[i * 3]];
        let v1 = vertices[nvertices[i * 3 + 1]];
        let v2 = vertices[nvertices[i * 3 + 2]];

        let mut v0_raster = convert_to_raster(&v0,&world_to_camera,left,right,top,bottom,near_clipping_plane,image_width,image_height);
        let mut v1_raster = convert_to_raster(&v1,&world_to_camera,left,right,top,bottom,near_clipping_plane,image_width,image_height);
        let mut v2_raster = convert_to_raster(&v2,&world_to_camera,left,right,top,bottom,near_clipping_plane,image_width,image_height);
        v0_raster.z = 1.0 / v0_raster.z;
        v1_raster.z = 1.0 / v1_raster.z;
        v2_raster.z = 1.0 / v2_raster.z;

        let mut st0 = st[stindices[i * 3]];
        let mut st1 = st[stindices[i * 3 +1 ]];
        let mut st2 = st[stindices[i * 3 +2 ]];

        st0 *= v0_raster.z;
        st1 *= v1_raster.z;
        st2 *= v2_raster.z;

        let xmin = min3(v0_raster.x , v1_raster.x ,v2_raster.x);
        let ymin = min3(v0_raster.y , v1_raster.y ,v2_raster.y);
        let xmax = max3(v0_raster.x , v1_raster.x ,v2_raster.x);
        let ymax = max3(v0_raster.y , v1_raster.y ,v2_raster.y);

        if xmin > (image_width -1) as f64 || xmax <0.0 || ymin > (image_height -1) as f64 ||ymax < 0.0 {
            continue;
        }

        let x0 = xmin.floor().max(0.0) as u32;
        let x1 = xmax.floor().min(image_width as f64 - 1.0) as u32;
        let y0 = xmin.floor().max(0.0) as u32;
        let y1 = xmin.floor().min(image_height as f64 - 1.0) as u32;

        let area = edge_function(&v0_raster,&v1_raster,&v2_raster);

        for y in y0..y1{
            for x in x0..x1{
                let pixel_sample = Vec3f{
                    x: x as f64 +0.5,
                    y: y as f64 + 0.5,
                    z : 0.0
                };

                let mut w0 = edge_function(&v1_raster,&v2_raster,&pixel_sample);
                let mut w1 = edge_function(&v2_raster,&v0_raster,&pixel_sample);
                let mut w2 = edge_function(&v0_raster,&v1_raster,&pixel_sample);

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    w0 /= area;
                    w1 /= area;
                    w2 /= area;

                    let one_over_z = v0_raster.z * w0 + v1_raster.z * w1 + v2_raster.z * w2;
                    let z = 1.0 / one_over_z;

                    if z < depth_buffer[(y * image_width + x) as usize ]{
                        depth_buffer[(y * image_width + x) as usize] = z;

                        let st = st0 * w0
                    }
                }
                
            }
        }
        






    }
}
