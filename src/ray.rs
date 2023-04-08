use crate::vector3::Point3;
use crate::vector3::Vector3;
use crate::color::write_color;

#[derive(Debug)]
pub struct Ray {
    orig : Point3,
    dir : Vector3
}

impl Ray {
    pub fn new(origin : Point3, direction : Vector3) -> Self {
        Self{orig: origin, dir: direction}
    }

    pub fn new_empty() -> Self{
        Self{orig: Point3::new_empty(), dir: Vector3::new_empty()}
    }

    pub fn origin(&self) -> &Point3 {&self.orig}
    pub fn dir(&self) -> &Vector3 {&self.dir}

    pub fn at(&self, t : f64) -> Point3 {
        self.orig.copy() + t*self.dir.copy()
    }
}