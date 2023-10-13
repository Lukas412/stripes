use crate::Color;
use std::fmt::{Display, Formatter};

pub enum Part {
    Solid(SolidColor),
    Linear(LinearGradient),
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Solid(color) => {
                write!(f, "{}", color)
            }
            Part::Linear(gradient) => {
                write!(f, "{}", gradient)
            }
        }
    }
}

pub struct SolidColor {
    color: Color,
    length: u8,
}

impl SolidColor {
    pub fn new(color: Color, mut length: u8) -> Self {
        if length == 0 {
            length = 1;
        }
        Self { color, length }
    }

    pub fn single(color: Color) -> Self {
        Self { color, length: 1 }
    }
}

impl Display for SolidColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "c{}l{}", self.color, self.length)
    }
}

impl From<SolidColor> for Part {
    fn from(value: SolidColor) -> Self {
        Self::Solid(value)
    }
}

pub struct LinearGradient {
    start: Color,
    end: Color,
    length: u8,
}

impl LinearGradient {
    pub fn new(start: Color, end: Color, mut length: u8) -> Self {
        if length <= 1 {
            length = 2;
        }
        Self { start, end, length }
    }
}

impl Display for LinearGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "s{}e{}l{}", self.start, self.end, self.length)
    }
}

impl From<LinearGradient> for Part {
    fn from(value: LinearGradient) -> Self {
        Self::Linear(value)
    }
}
