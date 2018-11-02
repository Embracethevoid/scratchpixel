mod geometry;
use geometry::matrix::*;
use geometry::objects::*;
use geometry::vector::*;
fn main() {
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

    let ray_origin = Vec3f {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let mut ray_direction = Vec3f {
        x: 0.0,
        y: -2.0,
        z: 1.9,
    };
    ray_direction.normalize();
    let mut tnear = 1000.0;
    println!(
        "{:?} {:?} {:?}",
        t.intersect(&ray_origin, &ray_direction, &mut tnear),
        tnear,
        (7.61 as f64).sqrt()
    );
}
