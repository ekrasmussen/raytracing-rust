use std::ops::Index;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

type Point3 = Vector3;
type Color = Vector3;

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

    fn len() -> f64 {
        sqrt(len_sqrt())
    }

    fn len_sqrt() {
        coords[0]*coords[0] + coords[1]*coords[1] + coords[2]*coords[2];
    }
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

//Assign operators

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, value: f64) {
        self.coords[0] += value;
        self.coords[1] += value;
        self.coords[2] += value;
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, value: f64) {
        self.coords[0] *= value;
        self.coords[1] *= value;
        self.coords[2] *= value;
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, value: f64) {
        *self *= 1/value
    }
}