mod geometry;
use geometry::matrix::*;
use geometry::vector::*;

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

fn convert_to_raster(vertex_world: Vec3f, world_to_camera: &Matrix44f) {}
fn main() {
    println!("Hello, world!");
}
