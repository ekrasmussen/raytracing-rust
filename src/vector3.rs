use std::ops::Index;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug)]
struct Vector3 {
    //Made as an array to maximize speed
    coords: [f64; 3]
}

impl Vector3 {
    //Methods to get coords from the array
    fn x(&self) -> f64 {self.coords[0]}
    fn y(&self) -> f64 {self.coords[1]}
    fn z(&self) -> f64 {self.coords[2]}

    //Constructors
    fn new_empty() -> Self {
        Self{coords: [0, 3]}
    }
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self{coords: [x, y, z]}
    }
    //End of Constructors
}


//Overloading the [] Operator
impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index];
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

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {coords: [self.coords[0] - other.coords[0], self.coords[1] - other.coords[1], self.coords[2] - other.coords[2]]}
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {coords: [self.coords[0] * other.coords[0], self.coords[1] * other.coords[1], self.coords[2] * other.coords[2]]}
    }
}