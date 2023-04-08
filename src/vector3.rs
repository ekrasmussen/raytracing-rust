use std::ops::Index;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;


pub type Point3 = Vector3;
pub type Color = Vector3;

#[derive(Debug)]
pub struct Vector3 {
    //Made as an array to maximize speed
    coords: [f64; 3]
}

impl Vector3 {
    //Methods to get coords from the array
    pub fn x(&self) -> f64 {self.coords[0]}
    pub fn y(&self) -> f64 {self.coords[1]}
    pub fn z(&self) -> f64 {self.coords[2]}

    //Constructors
    pub fn new_empty() -> Self {
        Self{coords: [0., 0., 0.]}
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self{coords: [x, y, z]}
    }
    //End of Constructors

    fn len(&self) -> f64 {
        self.len_sqrt().sqrt()
    }

    fn len_sqrt(&self) -> f64 {
        self.coords[0] * self.coords[0] + self.coords[1] * self.coords[1] + self.coords[2] * self.coords[2]
    }

    fn dot(self, other: Vector3) -> f64 {
        self.coords[0] * other.coords[0] + self.coords[1] * other.coords[1] + self.coords[2] * other.coords[2]
    }

    fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            coords: [self.coords[1] * other.coords[2] - self.coords[2] * other.coords[1],
                    self.coords[2] * other.coords[0] - self.coords[0] * other.coords[2],
                    self.coords[0] * other.coords[1] - self.coords[1] * other.coords[0]]
        }
    }

    pub fn unit_vector(&self) -> Vector3 {
        let new = Vector3::copy(self);
        let len = self.len();
        new / len
    }

    pub fn copy(&self) -> Vector3 {
        Vector3::new(self.coords[0], self.coords[1], self.coords[2])
    }
}


//Overloading the [] Operator
impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

//Overloading the Negation (-<Object>) Operator
impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3{ coords: [-self.coords[0], -self.coords[1], -self.coords[2]]}
    }
}

//Overloading the Addition + Operator
impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {coords: [self.coords[0] + other.coords[0], self.coords[1] + other.coords[1], self.coords[2] + other.coords[2]]}
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {coords: [self.coords[0] - other.coords[0], self.coords[1] - other.coords[1], self.coords[2] - other.coords[2]]}
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {coords: [self.coords[0] * other.coords[0], self.coords[1] * other.coords[1], self.coords[2] * other.coords[2]] }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 { coords: [self * other.coords[0], self * other.coords[1], self * other.coords[2]] }
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
        (1.0 / other) * self
    }
}

//Assign operators

impl AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, value: f64) {
        self.coords[0] += value;
        self.coords[1] += value;
        self.coords[2] += value;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, value: f64) {
        self.coords[0] *= value;
        self.coords[1] *= value;
        self.coords[2] *= value;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, value: f64) {
        *self *= 1. / value
    }
}