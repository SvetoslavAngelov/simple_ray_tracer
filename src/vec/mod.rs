#![allow(unused)]
use std::ops::Mul;
use std::ops::Div;
use std::ops::Add;
use std::ops::Sub;
use std::ops::{Index, IndexMut};

#[derive(Copy)]
pub struct Vec3{
    pub data: [f32; 3]
}

impl Vec3{
    pub fn new(x: f32, y: f32, z: f32) -> Vec3{
       return Vec3{ data: [x,y,z] };
    }

    pub fn magnitude(&self) -> f32{
        return (self.data[0] * self.data[0]
                + self. data[1] * self.data[1]
                + self.data[2] * self.data[2]).sqrt();
    }
    
    pub fn normalise(self) -> Vec3 {
        return self / self.magnitude();
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32{
        return (a.data[0] * b.data[0] 
                + a.data[1] * b.data[1]
                + a.data[2] * b.data[2]);
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3{
        return Vec3::new(a.data[1] * b.data[2] - a.data[2] * b.data[1],
                            a.data[2] * b.data[0] - a.data[0] * b.data[2],
                            a.data[0] * b.data[1] - a.data[1] * b.data[0]);
    }

    pub fn x(&self) -> f32 {
        return self.data[0];
    }

    pub fn y(&self) -> f32 {
        return self.data[1];
    }

    pub fn z(&self) -> f32{
        return self.data[2];
    }
 }

pub fn make_unit_vector(a: &Vec3) -> Vec3 {
        return a.normalise();
}

impl Clone for Vec3{
    fn clone(&self) -> Self{
        return *self;
    }
}

impl Default for Vec3{
    fn default() -> Vec3{
        return Vec3::new(0.0, 0.0, 0.0);
    }
}

impl Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3{
        return Vec3::new(self.data[0] * scalar, 
                            self.data[1] * scalar,
                            self.data[2] * scalar);
    }
}

impl Div<f32> for Vec3{
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3{
        let temp = 1.0 / scalar; 
        return Vec3::new(self.data[0] * temp, 
                            self.data[1] * temp,
                            self.data[2] * temp);
    }
}

impl Index<usize> for Vec3{
    type Output = f32; 
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output{
        return &self.data[index];
    }
}

impl IndexMut<usize> for Vec3{
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output{
        return &mut self.data[index];
    }
}

impl Add for Vec3{
    type Output = Vec3; 
    fn add(self, other: Vec3) -> Vec3{
        return Vec3::new(self.data[0] + other.data[0],
                            self.data[1] + other.data[1],
                            self.data[2] + other.data[2]);
    }
}

impl Sub for Vec3{
    type Output = Vec3; 
    fn sub(self, other: Vec3) -> Vec3{
        return Vec3::new(self.data[0] - other.data[0],
                            self.data[1] - other.data[1],
                            self.data[2] - other.data[2]);
    }
}