#![allow(unused)]
use crate::ray::Ray;
use crate::vec::Vec3;
use std::vec::Vec;
use std::default::Default; 
use std::clone::Clone;

pub trait Hitable{
    fn hit(&self, rec: &mut Record, r: &Ray, t_min: f32, t_max: f32) -> bool;
}

#[derive(Clone)]
pub struct Record{
    t: f32,
    p: Vec3,
    pub normal: Vec3
}

pub struct Sphere{
    centre: Vec3,
    radius: f32
}

pub struct Shapes{
    list: Vec<Sphere>,
    size: usize
}

impl Sphere{
    pub fn new(centre: Vec3, r: f32) -> Sphere{
        return Sphere{ centre: centre, radius: r };
    }
}

impl Shapes{
    pub fn new(list: Vec<Sphere>, size: usize) -> Shapes{
        return Shapes{ list: list, size: size };
    }
}

impl Record{
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> Record{
        return Record{ t: t, p: p, normal: normal };
    }
}

impl Default for Record{
    fn default() -> Record{
        return Record{ t: 0.0, p: Vec3::new(0.0, 0.0, 0.0), normal:Vec3::new(0.0, 0.0, 0.0)};
    }
}

impl Hitable for Sphere{
    fn hit(&self, rec: &mut Record, r: &Ray, t_min: f32, t_max: f32) -> bool{
        let oc: Vec3 = r.origin() - self.centre;
        let a: f32 = Vec3::dot(&r.direction(), &r.direction());
        let b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = Vec3::dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant: f32 = (b * b) - (a * c);
        
        if discriminant > 0.0 {
            let mut temp: f32 = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp; 
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min{
                rec.t = temp; 
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
                return true;
            }
        }
        return false;
    }
}

impl Hitable for Shapes{
    fn hit(&self, rec: &mut Record, r: &Ray, t_min: f32, t_max: f32) -> bool{
        let mut temp_rec: Record = Record::default(); 
        let mut is_hit: bool = false; 
        let mut closest = t_max; 

        for i in 0..self.size{
            if self.list[i].hit(&mut temp_rec, r, t_min, t_max){
                is_hit = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        } 
        return is_hit;
    }
}