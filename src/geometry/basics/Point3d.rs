use core::panic;
use std::ops;

use crate::IsValid::IsValid;

/// A Point in three-dimensional space
#[derive(Copy, Clone)]
pub struct Point3d
{
    /// The X coordinate
    pub x: f32,
    /// The Y coordinate
    pub y: f32,
    /// The Z coordinate
    pub z: f32,
}

impl Point3d {
    
    // Constants
    
    /// The Origin, (0,0,0)
    pub const ORIGIN:Point3d = Point3d {x:0f32, y:0f32, z:0f32 };

    /// An Unset point. A completely invalid point that does not exist
    pub const UNSET:Point3d = Point3d { x:f32::NAN, y:f32::NAN, z:f32::NAN };

    /// A Point at the edge of positive Infinity
    pub const INFINITY:Point3d = Point3d { x:f32::INFINITY, y:f32::INFINITY, z:f32::INFINITY };

    /// A point at the edge of negative infinity
    pub const NEGATIVE_INFINITY:Point3d = Point3d { x:f32::NEG_INFINITY, y:f32::NEG_INFINITY, z:f32::NEG_INFINITY };

    /// The Maximum possible Point
    pub const MAX:Point3d = Point3d { x:f32::MAX, y:f32::MAX, z:f32::MAX };

    /// The Minimum possible Point
    pub const MIN:Point3d = Point3d { x:f32::MIN, y:f32::MIN, z:f32::MIN };

    // Constructors

    /// Constructs a new Point3d
    pub fn new(x:f32, y:f32, z:f32) -> Point3d {
        Point3d { x, y, z }
    }

    // Methods
    
    /// Adds two points together and returns the result
    pub fn add(p1: &Point3d, p2: &Point3d) -> Point3d {
        Point3d::new(p1.x + p2.x,
                    p1.y + p2.y,
                    p1.z + p2.z)
    }
    
    /// Subtracts the left hand side point from the right hand size point and returns the result
    pub fn subtract(lhs: &Point3d, rhs: &Point3d) -> Point3d {
        Point3d::new(lhs.x - rhs.x,
                    lhs.y - rhs.y,
                    lhs.z - rhs.z)
    }
    
    /// Multiplies two points together and returns the result
    pub fn multiply(p1: &Point3d, p2: &Point3d) -> Point3d {
        Point3d::new(p1.x * p2.x,
                    p1.y * p2.y,
                    p1.z * p2.z)
    }

    /// Multiplies a point by a factor
    pub fn multiply_by_factor(p1: &Point3d, factor:f32) -> Point3d {
        Point3d::new(p1.x * factor,
            p1.y * factor,
            p1.z * factor)
    }
    
    /// Divides the left hand side point from the right hand size point and returns the result
    pub fn divide(lhs: &Point3d, rhs: &Point3d) -> Point3d {
        Point3d::new(lhs.x / rhs.x,
                    lhs.y / rhs.y,
                    lhs.z / rhs.z)
    }

    /// Divides a point by a factor
    pub fn divide_by_factor(p1: &Point3d, factor:f32) -> Point3d {
        Point3d::new(p1.x / factor,
            p1.y / factor,
            p1.z / factor)
    }

    pub fn distance_to(&self, rhs:&Point3d) -> f32 {
        let x_val = (rhs.x - self.x).powf(2f32);
        let y_val = (rhs.y - self.y).powf(2f32);
        let z_val = (rhs.z - self.z).powf(2f32);
        
        let val = x_val + y_val + z_val;
        
        val.sqrt()
    }

    pub fn interpolate(&self, rhs:&Point3d, parameter:f32) -> Point3d {
        let diff_x = (self.x - rhs.x) / 2f32;
        let diff_y = (self.y - rhs.y) / 2f32;
        let diff_z = (self.z - rhs.z) / 2f32;
        
        let new_x = self.x + (diff_x * parameter);
        let new_y = self.y + (diff_y * parameter);
        let new_z = self.z + (diff_z * parameter);

        Point3d::new(new_x, new_y, new_z)
    }

}


impl ops::Add<Point3d> for Point3d {
    type Output = Point3d;
    fn add(self, rhs: Point3d) -> Self::Output {
        Point3d::add(&self, &rhs)
    }
}

impl ops::Sub<Point3d> for Point3d {
    type Output = Point3d;
    fn sub(self, rhs: Point3d) -> Self::Output {
        Point3d::subtract(&self, &rhs)
    }
}

impl ops::Mul<Point3d> for Point3d {
    type Output = Point3d;
    fn mul(self, rhs: Point3d) -> Self::Output {
        Point3d::multiply(&self, &rhs)
    }
}

impl ops::Div<Point3d> for Point3d {
    type Output = Point3d;
    fn div(self, rhs: Point3d) -> Self::Output {
        Point3d::divide(&self, &rhs)
    }
}

/*
impl Clone for Point3d {
    fn clone(&self) -> Point3d {
        Point3d::new(self.x, self.y, self.z)
    }
}
*/

impl IsValid for Point3d {
    fn is_valid(&self) -> bool {
        !self.x.is_nan() && self.x.is_finite() &&
        !self.y.is_nan() && self.y.is_finite() &&
        !self.z.is_nan() && self.z.is_finite()
    }
}

impl PartialEq for Point3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Point3d {
    fn gt(&self, other: &Self) -> bool {
        self.x > other.x &&
        self.y > other.y &&
        self.z > other.z
    }

    fn lt(&self, other: &Self) -> bool {
        self.x < other.x &&
        self.y < other.y &&
        self.z < other.z
    }

    /// So. How does this work?
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        panic!("not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_valid() {
        // Not Valid
        assert!(!Point3d::UNSET.is_valid());
        assert!(!Point3d::INFINITY.is_valid());
        assert!(!Point3d::NEGATIVE_INFINITY.is_valid());
        
        // Valid
        assert!(Point3d::ORIGIN.is_valid());
        assert!(Point3d::MAX.is_valid());
        assert!(Point3d::MIN.is_valid());
    }

    #[test]
    pub fn point_origin() {
        let point = Point3d::ORIGIN;
        assert_eq!(0f32, point.x);
        assert_eq!(0f32, point.y);
        assert_eq!(0f32, point.z);
    }

    #[test]
    pub fn add_two_points() {
        let point: Point3d = Point3d::new(100f32, 100f32, 100f32);
        let point2: Point3d = Point3d::new(200f32, 200f32, 200f32);

        let new_point = point + point2;
        assert_eq!(300f32, new_point.x);
        assert_eq!(300f32, new_point.y);
        assert_eq!(300f32, new_point.z);
    }

    #[test]
    pub fn subtract_two_points() {
        let point: Point3d = Point3d::new(100f32, 100f32, 100f32);
        let point2: Point3d = Point3d::new(200f32, 200f32, 200f32);

        let new_point = point2 - point;
        assert_eq!(100f32, new_point.x);
        assert_eq!(100f32, new_point.y);
        assert_eq!(100f32, new_point.z);
    }

    #[test]
    pub fn divide_two_points() {
        let point: Point3d = Point3d::new(100f32, 100f32, 100f32);
        let point2: Point3d = Point3d::new(200f32, 200f32, 200f32);

        let new_point = point2 / point;
        assert_eq!(2f32, new_point.x);
        assert_eq!(2f32, new_point.y);
        assert_eq!(2f32, new_point.z);
    }

    #[test]
    pub fn divide_by_factor() {
        let point = Point3d::new(100f32, 200f32, 300f32);
        let new_point = Point3d::divide_by_factor(&point, 2f32);
        
        assert_eq!(50f32, new_point.x);
        assert_eq!(100f32, new_point.y);
        assert_eq!(150f32, new_point.z);
    }

    #[test]
    pub fn multiply_two_points() {
        let point: Point3d = Point3d::new(4f32, 4f32, 4f32);
        let point2: Point3d = Point3d::new(5f32, 5f32, 5f32);

        let new_point = point * point2;
        assert_eq!(20f32, new_point.x);
        assert_eq!(20f32, new_point.y);
        assert_eq!(20f32, new_point.z);
    }

    #[test]
    pub fn multiply_by_factor() {
        let point: Point3d = Point3d::new(100f32, 200f32, 300f32);
        let bigger_point = Point3d::multiply_by_factor(&point, 2f32);
        
        assert_eq!(200f32, bigger_point.x);
        assert_eq!(400f32, bigger_point.y);
        assert_eq!(600f32, bigger_point.z);
    }

}
