#![allow(unused)]
use crate::ray::Ray;
use crate::vec;
use crate::shape::Record;

type Vec3 = vec::Vec3;

pub trait Material{
    fn scatter(&self, rec: &Record, ray_in: &Ray, ray_scattered: &mut Ray, attenuation: &mut Vec3) -> bool;
}

pub struct Diffuse{
    albedo: Vec3
}

pub struct Specular{
    albedo: Vec3
}

impl Diffuse{
    fn new(diffuse_ref: Vec3) -> Diffuse{
        return Diffuse{ albedo: diffuse_ref.clone() };
    }
}

impl Specular{
    fn new (diffuse_ref: Vec3) -> Specular{
         return Specular{ albedo: diffuse_ref.clone() };
    }
}

impl Material for Diffuse{
    fn scatter(&self, rec: &Record, ray_in: &Ray, ray_scattered: &mut Ray, attenuation: &mut Vec3) -> bool{
        use vec::random_unit_sphere;
        
        let target: Vec3 = rec.p + rec.normal + random_unit_sphere();
        *ray_scattered = Ray::new(&rec.p, &(target - rec.p));
        *attenuation = self.albedo.clone(); 
        return true;
    }
}

impl Material for Specular{
    fn scatter(&self, rec: &Record, ray_in: &Ray, ray_scattered: &mut Ray, attenuation: &mut Vec3) -> bool{
        let reflected: Vec3 = reflect(&vec::make_unit_vector(&ray_in.direction()), &rec.normal);
        *ray_scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        return (Vec3::dot(&ray_scattered.direction(), &rec.normal)) > 0.0;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - (*n * Vec3::dot(&v, &n)) * 2.0;
}
