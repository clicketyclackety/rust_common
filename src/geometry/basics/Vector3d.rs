use std::ops;

use crate::IsValid::IsValid;

/// A Point in three-dimensional space
#[derive(Copy, Clone, Debug)]
pub struct Vector3d
{
    /// The X coordinate
    pub x: f32,
    /// The Y coordinate
    pub y: f32,
    /// The Z coordinate
    pub z: f32,
}

impl Vector3d {
    
    // Constants
    
    /// The Origin, (0,0,0)
    pub const ORIGIN:Vector3d = Vector3d {x:0f32, y:0f32, z:0f32 };

    /// An Unset point. A completely invalid point that does not exist
    pub const UNSET:Vector3d = Vector3d { x:f32::NAN, y:f32::NAN, z:f32::NAN };

    /// A Point at the edge of positive Infinity
    pub const INFINITY:Vector3d = Vector3d { x:f32::INFINITY, y:f32::INFINITY, z:f32::INFINITY };

    /// A point at the edge of negative infinity
    pub const NEGATIVE_INFINITY:Vector3d = Vector3d { x:f32::NEG_INFINITY, y:f32::NEG_INFINITY, z:f32::NEG_INFINITY };

    /// The Maximum possible Point
    pub const MAX:Vector3d = Vector3d { x:f32::MAX, y:f32::MAX, z:f32::MAX };

    /// The Minimum possible Point
    pub const MIN:Vector3d = Vector3d { x:f32::MIN, y:f32::MIN, z:f32::MIN };

    /// A Vector along the X Axis (1,0,0)
    pub const XAXIS:Vector3d = Vector3d { x:1f32, y:0f32, z:0f32 };
    
    /// A Vector along the Y Axis (1,0,0)
    pub const YAXIS:Vector3d = Vector3d { x:0f32, y:1f32, z:0f32 };
    
    /// A Vector along the Z Axis (1,0,0)
    pub const ZAXIS:Vector3d = Vector3d { x:0f32, y:0f32, z:1f32 };

    // Constructors

    /// Constructs a new Vector3d
    pub fn new(x:f32, y:f32, z:f32) -> Vector3d {
        Vector3d { x, y, z }
    }

    /// Returns the length of the Vector3d
    pub fn length(&self) -> f32 {
        let pows = self.x.powf(2f32) + self.y.powf(2f32) + self.z.powf(2f32);
        pows.sqrt()
    }

    /// Returns a normalized Vector3d
    pub fn unitize(vec:&Vector3d) -> Vector3d {
        Vector3d::divide_by_factor(vec, vec.length())
    }

    // Methods
    
    /// Adds two points together and returns the result
    pub fn add(p1: &Vector3d, p2: &Vector3d) -> Vector3d {
        Vector3d::new(p1.x + p2.x,
                    p1.y + p2.y,
                    p1.z + p2.z)
    }
    
    /// Subtracts the left hand side point from the right hand size point and returns the result
    pub fn subtract(lhs: &Vector3d, rhs: &Vector3d) -> Vector3d {
        Vector3d::new(lhs.x - rhs.x,
                    lhs.y - rhs.y,
                    lhs.z - rhs.z)
    }
    
    /// Multiplies two points together and returns the result
    pub fn multiply(p1: &Vector3d, p2: &Vector3d) -> Vector3d {
        Vector3d::new(p1.x * p2.x,
                    p1.y * p2.y,
                    p1.z * p2.z)
    }

    /// Multiplies a point by a factor
    pub fn multiply_by_factor(p1: &Vector3d, factor:f32) -> Vector3d {
        Vector3d::new(p1.x * factor,
            p1.y * factor,
            p1.z * factor)
    }
    
    /// Divides the left hand side point from the right hand size point and returns the result
    pub fn divide(lhs: &Vector3d, rhs: &Vector3d) -> Vector3d {
        Vector3d::new(lhs.x / rhs.x,
                    lhs.y / rhs.y,
                    lhs.z / rhs.z)
    }

    /// Divides a point by a factor
    pub fn divide_by_factor(p1: &Vector3d, factor:f32) -> Vector3d {
        Vector3d::new(p1.x / factor,
            p1.y / factor,
            p1.z / factor)
    }

}


impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn add(self, rhs: Vector3d) -> Self::Output {
        Vector3d::add(&self, &rhs)
    }
}

impl ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn sub(self, rhs: Vector3d) -> Self::Output {
        Vector3d::subtract(&self, &rhs)
    }
}

impl ops::Mul<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn mul(self, rhs: Vector3d) -> Self::Output {
        Vector3d::multiply(&self, &rhs)
    }
}

impl ops::Div<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn div(self, rhs: Vector3d) -> Self::Output {
        Vector3d::divide(&self, &rhs)
    }
}

impl  IsValid for Vector3d {
    fn is_valid(&self) -> bool {
        !self.x.is_nan() && self.x.is_finite() &&
        !self.y.is_nan() && self.y.is_finite() &&
        !self.z.is_nan() && self.z.is_finite()
    }
}


impl PartialEq for Vector3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_valid() {
        // Not Valid
        assert!(!Vector3d::UNSET.is_valid());
        assert!(!Vector3d::INFINITY.is_valid());
        assert!(!Vector3d::NEGATIVE_INFINITY.is_valid());
        
        // Valid
        assert!(Vector3d::ORIGIN.is_valid());
        assert!(Vector3d::MAX.is_valid());
        assert!(Vector3d::MIN.is_valid());
    }

    #[test]
    pub fn vector_origin() {
        let vector = Vector3d::ORIGIN;
        assert_eq!(0f32, vector.x);
        assert_eq!(0f32, vector.y);
        assert_eq!(0f32, vector.z);
    }

    #[test]
    pub fn add_two_vectors() {
        let vector: Vector3d = Vector3d::new(100f32, 100f32, 100f32);
        let vector2: Vector3d = Vector3d::new(200f32, 200f32, 200f32);

        let new_vector = vector + vector2;
        assert_eq!(300f32, new_vector.x);
        assert_eq!(300f32, new_vector.y);
        assert_eq!(300f32, new_vector.z);
    }

    #[test]
    pub fn subtract_two_vectors() {
        let vector: Vector3d = Vector3d::new(100f32, 100f32, 100f32);
        let vector2: Vector3d = Vector3d::new(200f32, 200f32, 200f32);

        let new_vector = vector2 - vector;
        assert_eq!(100f32, new_vector.x);
        assert_eq!(100f32, new_vector.y);
        assert_eq!(100f32, new_vector.z);
    }

    #[test]
    pub fn divide_two_vectors() {
        let vector: Vector3d = Vector3d::new(100f32, 100f32, 100f32);
        let vector2: Vector3d = Vector3d::new(200f32, 200f32, 200f32);

        let new_vector = vector2 / vector;
        assert_eq!(2f32, new_vector.x);
        assert_eq!(2f32, new_vector.y);
        assert_eq!(2f32, new_vector.z);
    }

    #[test]
    pub fn divide_by_factor() {
        let vector = Vector3d::new(100f32, 200f32, 300f32);
        let new_vector = Vector3d::divide_by_factor(&vector, 2f32);
        
        assert_eq!(50f32, new_vector.x);
        assert_eq!(100f32, new_vector.y);
        assert_eq!(150f32, new_vector.z);
    }

    #[test]
    pub fn multiply_two_vectors() {
        let vector: Vector3d = Vector3d::new(4f32, 4f32, 4f32);
        let vector2: Vector3d = Vector3d::new(5f32, 5f32, 5f32);

        let new_vector = vector * vector2;
        assert_eq!(20f32, new_vector.x);
        assert_eq!(20f32, new_vector.y);
        assert_eq!(20f32, new_vector.z);
    }

    #[test]
    pub fn multiply_by_factor() {
        let vector: Vector3d = Vector3d::new(100f32, 200f32, 300f32);
        let bigger_vector = Vector3d::multiply_by_factor(&vector, 2f32);
        
        assert_eq!(200f32, bigger_vector.x);
        assert_eq!(400f32, bigger_vector.y);
        assert_eq!(600f32, bigger_vector.z);
    }

}
