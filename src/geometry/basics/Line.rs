use crate::IsValid::IsValid;
use crate::geometry::basics::Point3d::Point3d;
use crate::geometry::basics::Vector3d::Vector3d;

/// A Line segment constrained between two points
pub struct Line {
    /// The start of the line
    pub start: Point3d,
    // The end of the line
    pub end: Point3d,
}

impl Line {

    pub const UNSET:Line = Line { start:Point3d::UNSET, end:Point3d::UNSET };

    pub fn new(start: Point3d, end: Point3d) -> Line {
        Line { 
            start, 
            end 
        }
    }

    pub fn new_given_direction(origin: Point3d, direction: Vector3d, distance:f32)
    {
        let unit_direction = Vector3d::unitize(&direction);
        let adjusted_amplitude = Vector3d::multiply_by_factor(&unit_direction, distance);

        let end = Point3d::new(0f32, 0f32, 0f32);

        Line::new(origin, end);
    }

    /// Returns the Direction of the Line
    pub fn get_start_tangeant(&self) -> Vector3d {
        Vector3d::new(self.start.x - self.end.x,
                    self.start.y - self.end.y,
                    self.start.z - self.end.z)
    }

}

impl IsValid for Line {
    fn is_valid(&self) -> bool {
        self.start.is_valid() && self.end.is_valid()
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.start.eq(&other.start) &&
        self.end.eq(&other.end)
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
        assert!(!Line::UNSET.is_valid());
    }

}