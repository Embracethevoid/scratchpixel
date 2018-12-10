use geometry::vector::*;
use std::f64;
static parallel_threshold: f64 = 0.0001;
#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    DIFFUSE_AND_GLOSSY,
    RFLECTION,
    REFLECTION_AND_REFRACTION,
}

pub trait Object {
    // first return value about if it will intersect,
    // the second is tnear, INFINITY if not intersect
    // the third would be the object hit, for sphere it would the object it self,the mesh triangle would be the triangle hit,, if not object hit, it self would be returned
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
    ) -> (bool, f64, Option<Box<Object>>);

    fn get_material_type(&self) -> MaterialType;

    fn get_ior(&self) -> f64;

    fn get_surface_property(&self, hit_point: &Vec3f, ray_direction: &Vec3f) -> Vec3f;
}

pub struct ObjectAttributes {
    pub surface_color: Option<Vec3f>,
    pub emission_color: Option<Vec3f>,
    pub transparency: Option<f64>,
    pub reflection: Option<f64>,
    pub material_type: Option<MaterialType>,
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f64,
    pub surface_color: Vec3f,
    pub emission_color: Vec3f,
    pub transparency: f64,
    pub reflection: f64,
    pub material_type: MaterialType,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f64, object_attributes: ObjectAttributes) -> Sphere {
        let _center = center;
        let mut _surface_color = Vec3f::zero();
        let mut _emission_color = Vec3f::zero();
        let mut _transparency = 0.0;
        let mut _reflection = 0.0;
        let mut _material_type = MaterialType::DIFFUSE_AND_GLOSSY;
        match object_attributes.surface_color {
            Some(value) => _surface_color = value,
            _ => (),
        };

        match object_attributes.emission_color {
            Some(value) => _emission_color = value,
            _ => (),
        };

        match object_attributes.transparency {
            Some(value) => _transparency = value,
            _ => (),
        };

        match object_attributes.reflection {
            Some(value) => _reflection = value,
            _ => (),
        };
        match object_attributes.material_type {
            Some(value) => _material_type = value,
            _ => (),
        };

        Sphere {
            center: _center,
            radius: radius,
            surface_color: _surface_color,
            emission_color: _emission_color,
            transparency: _transparency,
            reflection: _reflection,
            material_type: _material_type,
        }
    }
}

impl Object for Sphere {
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
    ) -> (bool, f64, Option<Box<Object>>) {
        let l = self.center - *ray_origin;
        let distance_to_center = l.length2();
        let radius2 = self.radius.powi(2);
        // ray is inside the sphere
        if distance_to_center < radius2 {
            let tnear = (radius2 - distance_to_center).sqrt();
            return (true, tnear, Some(Box::new(*self)));
        }
        let tca = l.dot(&ray_direction);
        if tca < 0.0 {
            return (false, f64::INFINITY, None);
        }
        let d2 = l.length2() - tca * tca;
        if d2 > radius2 {
            return (false, f64::INFINITY, None);
        } else {
            let thc = (radius2 - d2).sqrt();
            let tnear = tca - thc;
            return (true, tnear, Some(Box::new(*self)));
        }
    }

    fn get_material_type(&self) -> MaterialType {
        self.material_type
    }

    fn get_surface_property(&self, hit_point: &Vec3f, ray_direction: &Vec3f) -> Vec3f {
        (*hit_point - self.center).normalize()
    }

    fn get_ior(&self) -> f64 {
        1.33
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub x: Vec3f,
    pub y: Vec3f,
    pub z: Vec3f,
    pub material_type: MaterialType,
}

impl Object for Triangle {
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
    ) -> (bool, f64, Option<Box<Object>>) {
        let e1 = self.y - self.x;
        let e2 = self.z - self.x;
        let s = *ray_origin - self.x;
        let p = ray_direction.cross_product(&e2);
        let q = s.cross_product(&e1);
        let det = p.dot(&e1);
        if det.abs() < parallel_threshold {
            return (false, f64::INFINITY, None);
        }
        let t = q.dot(&e2) / det;
        if t <= 0.0 {
            return (false, f64::INFINITY, None);
        }
        let _u = p.dot(&s) / det;
        let _v = ray_direction.dot(&q) / det;
        if 0.0 <= _u && _u <= 1.0 && 0.0 <= _v && _v <= 1.0 && (_u + _v) <= 1.0 {
            let tnear = t;
            return (true, tnear, Some(Box::new(*self)));
        }
        return (false, f64::INFINITY, None);
    }

    fn get_material_type(&self) -> MaterialType {
        self.material_type
    }

    fn get_surface_property(&self, hit_point: &Vec3f, ray_direction: &Vec3f) -> Vec3f {
        // *normal = (*hit_point - self.center).normalize();
        Vec3f::new(0.0, 0.0, 0.0)
    }

    fn get_ior(&self) -> f64 {
        1.33
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
    surface_color: Vec3f,
    emission_color: Vec3f,
    transparency: f64,
    reflection: f64,
    material_type: MaterialType,
}
impl MeshTriangle {
    pub fn new(
        verts: &Vec<Vec3f>,
        vertsIndex: &Vec<u32>,
        numTris: u32,
        st: &Vec2f,
        object_attributes: ObjectAttributes,
    ) -> MeshTriangle {
        let mut _surface_color = Vec3f::zero();
        let mut _emission_color = Vec3f::zero();
        let mut _transparency = 0.0;
        let mut _reflection = 0.0;
        let mut _material_type = MaterialType::DIFFUSE_AND_GLOSSY;
        match object_attributes.surface_color {
            Some(value) => _surface_color = value,
            _ => (),
        };

        match object_attributes.emission_color {
            Some(value) => _emission_color = value,
            _ => (),
        };

        match object_attributes.transparency {
            Some(value) => _transparency = value,
            _ => (),
        };

        match object_attributes.reflection {
            Some(value) => _reflection = value,
            _ => (),
        };
        match object_attributes.material_type {
            Some(value) => _material_type = value,
            _ => (),
        };
        let mut v: Vec<Triangle> = Vec::new();
        for ind in (0..vertsIndex.len()).step_by(3) {
            v.push(Triangle {
                x: verts[vertsIndex[ind] as usize],
                y: verts[vertsIndex[ind + 1] as usize],
                z: verts[vertsIndex[ind + 2] as usize],
                material_type: _material_type,
            })
        }

        MeshTriangle {
            triangles: v,
            surface_color: _surface_color,
            emission_color: _emission_color,
            transparency: _transparency,
            reflection: _reflection,
            material_type: _material_type,
        }
    }
}

impl Object for MeshTriangle {
    fn intersect(
        &self,
        ray_origin: &Vec3f,
        ray_direction: &Vec3f,
    ) -> (bool, f64, Option<Box<Object>>) {
        let mut res = false;
        let mut tnear = f64::INFINITY;
        let mut nearest_object: Option<Box<Object>> = None;
        for ind in 0..self.triangles.len() {
            let t = self.triangles[ind];
            let (_interserct, _tnear, _object) = t.intersect(ray_origin, ray_direction);
            if _interserct && _tnear < tnear {
                res = true;
                tnear = _tnear;
                match _object {
                    Some(o) => nearest_object = Some(o),
                    _ => panic!("this should not happen!"),
                }
            }
        }

        return (res, tnear, nearest_object);
    }

    fn get_material_type(&self) -> MaterialType {
        self.material_type
    }
    // It should not come into this part
    fn get_surface_property(&self, hit_point: &Vec3f, ray_direction: &Vec3f) -> Vec3f {
        // let hit_triangle = self.triangles[index];
        // let e0 = (hit_triangle.y - hit_triangle.x).normalize();
        // let e1 = (hit_triangle.z - hit_triangle.y).normalize();

        // *normal = (e1 - e0).normalize();
        panic!("Should not come into this function");
        Vec3f::new(0.0, 0.0, 0.0);
        // *st  =
    }

    fn get_ior(&self) -> f64 {
        1.33
    }
}

pub struct Light {
    pub position: Vec3f,
}

pub fn fresnel(ray_direction: &Vec3f, normal: &Vec3f, ior: f64) -> f64 {
    0.5
}
