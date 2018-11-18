use geometry::vector::*;
use std::f64;
static parallel_threshold: f64 = 0.0001;
enum MaterialType {
    DIFFUSE_AND_GLOSSY,
    RFLECTION,
    REFLECTION_AND_REFRACTION,
}

pub trait Object {
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
        tnear: &mut f64,
        index: &mut usize,

        uv: &mut Vec2f,
    ) -> bool;

    // fn get_surface_info(&self,intersect_point:&Vec3f,ray_direction:&Vec3f, uv:&Vec2f)
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
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
        tnear: &mut f64,
        index: &mut usize,

        uv: &mut Vec2f,
    ) -> bool {
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

impl Triangle {
    pub fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
        tnear: &mut f64,
        u: &mut f64,
        v: &mut f64,
    ) -> bool {
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
        let _u = p.dot(&s) / det;
        let _v = ray_direction.dot(&q) / det;
        if 0.0 <= _u && _u <= 1.0 && 0.0 <= _v && _v <= 1.0 && (_u + _v) <= 1.0 {
            *tnear = t;
            *u = _u;
            *v = _v;
            return true;
        }
        return false;
    }
}

pub fn reflect(ray_direction: &Vec3f, normal: &Vec3f) -> Vec3f {
    return (*ray_direction - (*normal) * 2.0 * ray_direction.dot(normal)).normalize();
}

pub fn refract(ray_direction: &Vec3f, normal: &Vec3f, ior: f64) -> Vec3f {
    let cosi = ray_direction.dot(normal);
    if cosi < 0.0 {
        let x = 1;
    }
    return Vec3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

struct MeshTriangle {
    triangles: Vec<Triangle>,
}
impl MeshTriangle {
    pub fn new(
        verts: &Vec<Vec3f>,
        vertsIndex: &Vec<u32>,
        numTris: u32,
        st: &Vec2f,
    ) -> MeshTriangle {
        let mut v: Vec<Triangle> = Vec::new();
        for ind in (0..vertsIndex.len()).step_by(3) {
            v.push(Triangle {
                x: verts[vertsIndex[ind] as usize],
                y: verts[vertsIndex[ind + 1] as usize],
                z: verts[vertsIndex[ind + 2] as usize],
            })
        }
        MeshTriangle { triangles: v }
    }
}
impl Object for MeshTriangle {
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
        tnear: &mut f64,
        index: &mut usize,
        uv: &mut Vec2f,
    ) -> bool {
        let mut res = false;
        for ind in 0..self.triangles.len() {
            let t = self.triangles[ind];
            let mut tmp_near = f64::INFINITY;
            let mut u = 0.0;
            let mut v = 1.0;
            if t.intersect(ray_origin, ray_direction, &mut tmp_near, &mut u, &mut v) {
                if tmp_near < *tnear {
                    *tnear = tmp_near;
                    uv.x = u;
                    uv.y = v;
                    *index = ind;
                    res = true;
                }
            }
        }
        return res;
    }
}
