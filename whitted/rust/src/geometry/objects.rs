use geometry::vector::*;

static parallel_threshold: f64 = 0.0001;
enum MaterialType {
    DIFFUSE_AND_GLOSSY,
    RFLECTION,
    REFLECTION_AND_REFRACTION,
}

pub trait Object {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64 , normal &mut Vec3f) -> bool;
}
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3f,
    radius: f64,
    surface_color: Vec3f,
    emission_color: Vec3f,
    transparency: f64,
    reflection: f64,
}

impl Object for Sphere {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64 ,normal: &mut Vec3f) -> bool {
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
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vec3f,
    pub y: Vec3f,
    pub z: Vec3f,
}

impl Object for Triangle {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64 , normal: &mut Vec3f) -> bool {
        let e1 = self.y - self.x;
        let e2 = self.z - self.x;
        let s = *ray_origin - self.x;
        let p = ray_direction.cross_product(&e2);
        let q = s.cross_product(&e1);
        let det = p.dot(&e1);
        if det < parallel_threshold {
            return false;
        }
        let t = q.dot(&e2) / det;
        if t <= 0.0 {
            return false;
        }
        let u = p.dot(&s) / det;
        let v = ray_direction.dot(&q) / det;
        if 0.0 < u && u < 1.0 && 0.0 < v && v < 1.0 && (u + v) < 1.0 {
            *tnear = t;
            if det < 0 {
                *normal =  - e1.cross_product(&e2);
            } else {
                *normal = e1.cross_product(&e2);
            }
            return true;
        }
        return false;
    }
}
