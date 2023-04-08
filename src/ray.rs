mod vector3;
mod color;
use crate::vector3::Point3;
use crate::color::write_color;

#[derive(Debug)]
pub struct Ray {
    orig : Point3,
    dir : Vector3
}

impl Ray {
    pub fn new(origin : &Point3, direction : &Vector3) {
        orig = origin;
        dir = direction;
    }

    pub fn new_empty() {
        orig = Point3::new(0,0,0);
        dir = Vector3::new(0,0,0);
    }
}