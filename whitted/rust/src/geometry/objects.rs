use geometry::vector::*;

enum MaterialType {
    DIFFUSE_AND_GLOSSY,
    RFLECTION,
    REFLECTION_AND_REFRACTION,
}

trait Object {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64) -> bool;
}
#[derive(Debug, Copy, Clone)]
struct Sphere {
    center: Vec3f,
    radius: f64,
    surface_color: Vec3f,
    emission_color: Vec3f,
    transparency: f64,
    reflection: f64,
}

impl Object for Sphere {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64) -> bool {
        let l = self.center - *ray_origin;
        let distance_to_center = l.length2();
        let radius2 = self.radius.powi(2);
        // ray is inside the sphere
        if distance_to_center < radius2 {
            *tnear = (radius2 - distance_to_center).sqrt();
            return true;
        }
        let tca = l.dot(&ray_direction);
        if tca < 0.0 {
            return false;
        }
        let d2 = l.length2() - tca * tca;
        if d2 > radius2 {
            return false;
        } else {
            let thc = (radius2 - d2).sqrt();
            *tnear = tca - thc;
            return true;
        }
    }
}

struct point3(f64, f64, f64);

struct Triangle {
    x: point3,
    y: point3,
    z: point3,
}

impl Object for Triangle {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64) -> bool {
        true
    }
}
