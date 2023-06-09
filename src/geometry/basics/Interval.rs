use std::ops::{self, Add};

use crate::IsValid::IsValid;

// Should Intervals be able to have a reverse direction?

/// An interval between two numbers
pub struct Interval {
    /// The minimum value
    min:f32,
    /// The maximum value
    max:f32
}

impl Interval {

    /// An Unset Interval
    pub const UNSET:Interval = Interval { min:f32::NAN, max:f32::NAN };

    /// A Zero Interval
    pub const ZERO:Interval = Interval { min:0f32, max:0f32 };

    pub fn new(min:f32, max:f32) -> Interval {
        Interval { min, max }
    }

    pub fn mid(&self) -> f32 {
        self.min + ((self.max - self.min) / 2f32)
    }

    pub fn is_increasing(&self) -> bool {
        self.min > self.max
    }

    pub fn is_decreasing(&self) -> bool {
        self.min < self.max
    }

    pub fn from_intersection(i1:Interval, i2:Interval) -> Interval {
        let mut lower = i1.min;
        let mut higher = i1.max;



        panic!("Not implemented yet!")
    }

    pub fn from_union(i1:Interval, i2:Interval) -> Interval {
        let mut lower = i1.min;
        let mut higher = i1.max;

        if i1.min > i2.min {
            lower = i2.min;
        }

        if i1.max > i2.max {
            higher = i2.max
        }

        Interval::new(lower, higher)
    }

    pub fn includes(&self, i1:Interval) -> bool {
        i1.min > self.min &&
        i1.max < self.max
    }

    pub fn includes_parameter(&self, p:f32) -> bool {
        p > self.min &&
        p < self.max
    }

    pub fn swap(self) -> Interval {
        Interval::new(self.max, self.min)
    }


}


impl ops::Add<f32> for Interval {
    type Output = Interval;
    fn add(self, shift: f32) -> Self::Output {
        Interval::new(self.min + shift,
                        self.max + shift)
    }
}

impl ops::Sub<f32> for Interval {
    type Output = Interval;
    fn sub(self, shift: f32) -> Self::Output {
        Interval::new(self.min - shift,
                        self.max - shift)
    }
}



impl IsValid for Interval {
    fn is_valid(&self) -> bool {
        self.max.is_finite() &&
        self.min.is_finite()
    }
}