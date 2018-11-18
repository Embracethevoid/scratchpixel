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
