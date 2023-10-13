use std::fmt::{Display, Formatter};
use std::ptr::write;

pub struct Animation {
    start: Frame,
    changes: Vec<Changes>,
}

impl Animation {
    pub fn new(start: Frame) -> Self {
        Self {
            start,
            changes: vec![],
        }
    }

    pub fn add(&mut self, changes: Changes) {
        self.changes.push(changes)
    }
}

impl Display for Animation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.start)?;
        for changes in self.changes.iter() {
            write!(f, "{}", changes)?;
        }
        Ok(())
    }
}

pub struct Frame {
    parts: Vec<Part>,
}

impl Frame {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    pub fn add_solid_color(&mut self, value: SolidColor) {
        self.add(Part::Solid(value))
    }

    fn add(&mut self, part: Part) {
        self.parts.push(part)
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

enum Part {
    Solid(SolidColor),
    Linear { start: Color, end: Color, steps: u8 },
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Solid(color) => {
                write!(f, "{}", color)
            }
            Part::Linear { start, end, steps } => {}
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

pub struct Changes {
    changes: Vec<Change>,
}

impl Changes {
    pub fn new() -> Self {
        Self { changes: vec![] }
    }
}

impl Display for Changes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ";")?;
        let mut changes = self.changes.iter();
        if let Some(first) = changes.next() {
            write!(f, "{}", first)?;
        }
        for change in changes {
            write!(f, ",{}", change)?;
        }
        Ok(())
    }
}

enum Change {
    Part(Part),
}

impl Display for Change {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Change::Part(part) => {
                write!(f, "{}", part);
            }
        }
        Ok(())
    }
}

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
