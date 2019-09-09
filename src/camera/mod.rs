#![allow(unused)]
use crate::ray::Ray;
use crate::vec::Vec3;
use std::default::Default; 

pub struct Camera{
    upper_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3
}

impl Camera{
    pub fn new(left_corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Camera{
        return Camera{
            upper_left_corner: left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray{
        return Ray::new(
           &self.origin, 
           &(self.upper_left_corner + self.horizontal * u + (self.vertical * v) - self.origin))
    }
}

impl Default for Camera{
    fn default() -> Camera{
        return Camera{
            upper_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}