use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

pub enum Hue {
    BLUE,
    GREEN,
    RED,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Colour {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, rhs: Colour) -> Colour {
        return Colour {
            blue: self.blue.overflowing_sub(rhs.blue).0,
            green: self.green.overflowing_sub(rhs.green).0,
            red: self.red.overflowing_sub(rhs.red).0,
        };
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Colour {
        return Colour {
            blue: self.blue.overflowing_add(rhs.blue).0,
            green: self.green.overflowing_add(rhs.green).0,
            red: self.red.overflowing_add(rhs.red).0,
        };
    }
}

impl Div<u8> for Colour {
    type Output = Colour;

    fn div(self, rhs: u8) -> Colour {
        return Colour {
            blue: self.blue / rhs,
            green: self.green / rhs,
            red: self.red / rhs,
        };
    }
}

pub fn colour_to_bytes(v: &Vec<Colour>) -> Vec<u8> {
    return v
        .iter()
        .fold(Vec::new(), |mut v, col| {
            v.push(col.blue);
            v.push(col.green);
            v.push(col.red);
            return v;
        });
}

fn extract_blue(v: &Vec<Colour>) -> Vec<u8> {
    return v
        .iter()
        .fold(Vec::new(), |mut v, col| {
            v.push(col.blue);
            return v;
        });
}

fn extract_green(v: &Vec<Colour>) -> Vec<u8> {
    return v
        .iter()
        .fold(Vec::new(), |mut v, col| {
            v.push(col.green);
            return v;
        });
}

fn extract_red(v: &Vec<Colour>) -> Vec<u8> {
    return v
        .iter()
        .fold(Vec::new(), |mut v, col| {
            v.push(col.red);
            return v;
        });
}

pub fn extract_colour(v: &Vec<Colour>, hue: &Hue) -> Vec<u8> {
    return match hue {
        Hue::BLUE => extract_blue(v),
        Hue::GREEN => extract_green(v),
        Hue::RED => extract_red(v),
    };
}
