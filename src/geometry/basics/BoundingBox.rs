use crate::IsValid::IsValid;
use crate::geometry::basics::Point3d::Point3d;

// A Box orientated to the WorldXY 
pub struct BoundingBox {
    /// 
    min:Point3d,
    /// 
    max:Point3d,
}

impl BoundingBox {

    /// A completely empty box with 0 volume
    pub const EMPTY:BoundingBox = BoundingBox { min:Point3d::ORIGIN, max:Point3d::ORIGIN };

    // An unset box with an unset min/max
    pub const UNSET:BoundingBox = BoundingBox { min:Point3d::UNSET, max:Point3d::UNSET };

    /// Creates a new boundingbox between the two points
    pub fn new(min:Point3d, max:Point3d) -> BoundingBox {
        BoundingBox { min, max }
    }

    /// Returns the volume of the bounding box
    pub fn volume(&self) -> f32 {
        let x_dist = self.min.x - self.min.x;
        let y_dist = self.min.z - self.min.y;
        let z_dist = self.max.z - self.min.z;
        
        x_dist * y_dist * z_dist
    }

    /// The center of the Box
    pub fn center(&self) -> Point3d {
        let x_half = self.min.x + ((self.max.x - self.min.x) / 2f32);
        let y_half = self.min.y + ((self.max.y - self.min.y) / 2f32);
        let z_half = self.min.z + ((self.max.z - self.min.z) / 2f32);

        Point3d::new(x_half, y_half, z_half)
    }

    pub fn contains_point(&self, point:Point3d) -> bool {
        if point.x < self.min.x || point.x > self.max.x {
            return false;
        }
        if point.y < self.min.y || point.y > self.max.y {
            return false;
        }
        if point.z < self.min.z || point.z > self.max.z {
            return false;
        }

        return true
    }

    pub fn contains_boundingbox(&self, bounds:BoundingBox) -> bool {
        self.contains_point(bounds.min) && self.contains_point(bounds.max)
    }

    pub fn inflate_uniform(&self, val:f32) {
        self.inflate(val, val, val)
    }

    pub fn inflate(&self, x:f32, y:f32, z:f32) {
        panic!("Not implemented!")
    }

    pub fn point_at(&self, x:f32, y:f32, z:f32) -> Point3d {
        let x_diff = self.max.x - self.min.x;
        let rel_x = x_diff * x;
        let new_x = self.min.x + rel_x;

        let y_diff = self.max.y - self.min.y;
        let rel_y = y_diff * y;
        let new_y = self.min.y + rel_y;

        let z_diff = self.max.z - self.min.z;
        let rel_z = z_diff * z;
        let new_z = self.min.z + rel_z;

        Point3d::new(new_x, new_y, new_z)
    }

    /// Returns all the corners of the Box
    pub fn get_corners(&self) -> Vec<Point3d> {
        vec![
            self.point_at(0f32, 0f32, 0f32),
            self.point_at(0f32, 1f32, 0f32),
            self.point_at(0f32, 0f32, 1f32),
            self.point_at(0f32, 1f32, 1f32),

            self.point_at(1f32, 0f32, 0f32),
            self.point_at(1f32, 1f32, 0f32),
            self.point_at(1f32, 0f32, 1f32),
            self.point_at(1f32, 1f32, 1f32),
        ]
        
    }

}

impl IsValid for BoundingBox {
    fn is_valid(&self) -> bool {
        self.min.is_valid() && self.max.is_valid() &&
        self.max > self.min
    }
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.min.eq(&other.min) &&
        self.max.eq(&other.max)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}