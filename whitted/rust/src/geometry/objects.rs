use geometry::vector::*;

static parallel_threshold: f64 = 0.0001;
enum MaterialType {
    DIFFUSE_AND_GLOSSY,
    RFLECTION,
    REFLECTION_AND_REFRACTION,
}

pub trait Object {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f64,
    pub surface_color: Vec3f,
    pub emission_color: Vec3f,
    pub transparency: f64,
    pub reflection: f64,
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
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vec3f,
    pub y: Vec3f,
    pub z: Vec3f,
}

impl Object for Triangle {
    fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64) -> bool {
        let e1 = self.y - self.x;
        let e2 = self.z - self.x;
        let s = *ray_origin - self.x;
        let p = ray_direction.cross_product(&e2);
        let q = s.cross_product(&e1);
        let det = p.dot(&e1);
        if det.abs() < parallel_threshold {
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
            // if det < 0.0 {
            //     *normal =  - e1.cross_product(&e2);
            // } else {
            //     *normal = e1.cross_product(&e2);
            // }
            return true;
        }
        return false;
    }
}


pub fn refract(ray_direction:&Vec3f, normal:&Vec3f , n1 :f64,n2:f64) -> Vec3f{
    let cosi = ray_direction.dot(normal).abs();
    return *ray_direction * n1 /n2 + *normal * (cosi * n1 / n2 - (1.0 - (n1/n2).powi(2)*(1.0 - cosi.powi(2))).sqrt());
}


// #[derive(Debug)]
// pub struct Object<T>{
//     data:T
// }

// impl<T:Object> Object for Object<T>{
//     pub fn intersect(&self, ray_origin: &Vec3f, ray_direction: &Vec3f, tnear: &mut f64) -> bool{
//         self.data.intersect(ray_origin, ray_direction, tnear)
//     }
// }

// impl<T> Object<T>{
//     pub fn new(other:T) -> Object<T>{
//         Object{
//             data:&other
//         }
//     }
// }
