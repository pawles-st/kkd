use pic_entropy::colour::Colour;
use std::ops::{Add, Sub, Div};
use serde::{Serialize, Deserialize};

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ColourDiff {
    pub blue: f64,
    pub green: f64,
    pub red: f64,
}

impl ColourDiff {
    pub fn from_colour(colour: Colour) -> Self {
        return ColourDiff{
            blue: colour.blue as f64,
            green: colour.green as f64, 
            red: colour.red as f64,
        };
    }

    pub fn from_colour_ref(colour: &Colour) -> Self {
        return ColourDiff{
            blue: colour.blue as f64,
            green: colour.green as f64, 
            red: colour.red as f64,
        };
    }

    pub fn zero() -> Self {
        return ColourDiff{
            blue: 0.0,
            green: 0.0,
            red: 0.0,
        }
    }
}

impl ColourDiff {
    pub fn abs_diff(self, rhs: ColourDiff) -> Self {
        return ColourDiff{
            blue: (self.blue - rhs.blue),
            green: (self.green - rhs.green).abs(),
            red: (self.red - rhs.red).abs(),
        }
    }
}

impl Add<ColourDiff> for ColourDiff {
    type Output = ColourDiff;

    fn add(self, rhs: ColourDiff) -> ColourDiff {
        return ColourDiff {
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
            red: self.red + rhs.red,
        };
    }
}

impl Sub<ColourDiff> for ColourDiff {
    type Output = ColourDiff;

    fn sub(self, rhs: ColourDiff) -> ColourDiff {
        return ColourDiff {
            blue: self.blue - rhs.blue,
            green: self.green - rhs.green,
            red: self.red - rhs.red,
        };
    }
}

impl Div<f64> for ColourDiff {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        return ColourDiff{
            blue: self.blue / rhs,
            green: self.green / rhs,
            red: self.red / rhs,
        };
    }
}
