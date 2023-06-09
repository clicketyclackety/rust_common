use crate::geometry::basics::Point3d::Point3d;
use crate::geometry::basics::Plane::Plane;

// A Box 
pub struct Box {
    plane:Plane,
    min:Point3d,
    max:Point3d
}


impl PartialEq for Box {
    fn eq(&self, other: &Self) -> bool {
        self.min.eq(&other.min) &&
        self.max.eq(&other.max) &&
        self.plane.eq(&other.plane)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
