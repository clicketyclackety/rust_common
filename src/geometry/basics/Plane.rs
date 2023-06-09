use crate::IsValid::IsValid;
use crate::geometry::basics::Point3d::Point3d;
use crate::geometry::basics::Vector3d::Vector3d;

/// Defines the infinite
pub struct Plane {
    /// The origin, this can be unset as its technically unecessary
    pub origin: Point3d,
    /// The X Vector
    pub x: Vector3d,
    /// The Y Vector
    pub y: Vector3d,
    /// The Z Vector
    pub z: Vector3d,
}

impl Plane {

    /// An Unset Plane, with an unset origin and unset vectors
    pub const UNSET:Plane = Plane { origin:Point3d::UNSET, x:Vector3d::UNSET, y:Vector3d::UNSET, z:Vector3d::UNSET };

    /// The World XY Plane
    pub const WORLDXY:Plane = Plane { origin:Point3d::ORIGIN, x:Vector3d::XAXIS, y:Vector3d::YAXIS, z:Vector3d::ZAXIS };

    /// The World YZ Plane
    pub const WORLDYZ:Plane = Plane { origin:Point3d::ORIGIN, x:Vector3d::ZAXIS, y:Vector3d::XAXIS, z:Vector3d::YAXIS };

    /// The World ZX Plane
    pub const WORLDZX:Plane = Plane { origin:Point3d::ORIGIN, x:Vector3d::YAXIS, y:Vector3d::ZAXIS, z:Vector3d::XAXIS };

    pub fn new(origin:Point3d, x:Vector3d, y:Vector3d) -> Plane {
        let z = Plane::cross_product(x.clone(), y.clone());
        Plane { origin, x, y, z }
    }

    /// Returns the Z Vector
    /// https://en.wikipedia.org/wiki/Cross_product
    fn cross_product(x:Vector3d, y:Vector3d) -> Vector3d {
        x * y
    }

}

impl IsValid for Plane {
    fn is_valid(&self) -> bool {
        self.origin.is_valid() &&
        self.x.is_valid() &&
        self.y.is_valid() &&
        self.z.is_valid()
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.origin.eq(&other.origin) &&
        self.x.eq(&other.x) &&
        self.y.eq(&other.y) &&
        self.z.eq(&other.z)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid()
    {
        // Invalids
        assert!(!Plane::UNSET.is_valid());
        assert!(!Plane::new(Point3d::INFINITY, Vector3d::INFINITY, Vector3d::INFINITY).is_valid());

        // Valids
        assert!(Plane::WORLDXY.is_valid());
        assert!(Plane::WORLDYZ.is_valid());
        assert!(Plane::WORLDZX.is_valid());
    }

    #[test]
    fn cross_product()
    {
        let z = Plane::cross_product(Vector3d::XAXIS, Vector3d::YAXIS);
        assert_eq!(z, Vector3d::ZAXIS);
    }

}