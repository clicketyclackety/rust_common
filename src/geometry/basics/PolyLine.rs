use crate::IsValid::IsValid;

use crate::geometry::basics::Point3d::Point3d;

pub struct PolyLine {
    points:Vec<Point3d>,
}

impl PolyLine {

    pub const UNSET:PolyLine = PolyLine { points:Vec::new() };

    pub fn new(points:Vec<Point3d>) -> PolyLine {
        PolyLine { points }
    }

    pub fn length(&self) -> f32 {
        if self.points.len() == 0
        {
            return 0f32
        }

        let mut last_point:Point3d = self.points[0];
        let mut counter = 0;
        let mut length = 0f32;
        loop {
            if counter == self.points.len() {
                break;
            }

            let dist = last_point.distance_to(&self.points[counter]);
            length += dist;

            last_point = self.points[counter]; 

            counter += 1;
        }
        
        length
    }

}


impl IsValid for PolyLine {
    fn is_valid(&self) -> bool {
        if self.points.len() == 0 {
            return false;
        }
        
        let mut counter = 0;
        loop {
            if counter == self.points.len()
            {
                return true;
            }

            if !self.points[counter].is_valid()
            {
                return false;
            }
            
            counter += 1;
        }

    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_distance_invalid() {
        let poly = PolyLine::UNSET;
        assert_eq!(0f32, poly.length());
    }

    #[test]
    fn test_distance_valid() {
        let points = vec!(Point3d::ORIGIN,
                          Point3d::new(100f32, 0f32, 0f32),
                          Point3d::new(100f32, 500f32, 0f32),
                          Point3d::new(0f32, 500f32, 0f32),
                          Point3d::ORIGIN);
        
        let poly = PolyLine::new(points);
        assert_eq!(1200f32, poly.length());
    }

}