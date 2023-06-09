use crate::IsValid::IsValid;
use crate::geometry::basics::Point3d::Point3d;

/// A Circle
pub struct Circle
{
    /// The center of the circle
    pub center: Point3d,

    /// The radius of the circle
    pub radius: f32,

}

impl Circle {
    
    pub const UNSET:Circle = Circle { center:Point3d::UNSET, radius:f32::NAN };
    
    pub fn new(center:Point3d, radius: f32) -> Circle {
        if radius.is_sign_negative() {
            panic!("Input radius cannot be negative")
        }

        Circle { center, radius }
    }

}

impl IsValid for Circle {
    fn is_valid(&self) -> bool {
        self.center.is_valid() &&
        self.radius.is_finite() &&
        self.radius.is_sign_positive()
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center.eq(&other.center) &&
        self.radius.eq(&other.radius)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    pub fn is_valid_failure_zero()
    {
        let zero = Circle::new(Point3d::ORIGIN, 0f32);
        assert!(!zero.is_valid());      
    }

    #[test]
    pub fn is_valid_failure_unset()
    {
        let unset = Circle::UNSET;
        assert!(!unset.is_valid()); 
    }

    #[test]
    #[should_panic]
    pub fn is_valid_failure_negative()
    {
        let negative = Circle::new(Point3d::ORIGIN, -100f32);
        assert!(!negative.is_valid());      
    }

    #[test]
    pub fn is_valid()
    {
        let valid_1 = Circle::new(Point3d::ORIGIN, 100f32);
        assert!(valid_1.is_valid());
        
        let tiny = Circle::new(Point3d::ORIGIN, 0.000001f32);
        assert!(tiny.is_valid());
    }
    
}
