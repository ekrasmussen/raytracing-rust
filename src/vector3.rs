use std::ops::Index;

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