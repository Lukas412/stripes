pub trait Serializable {
    fn serialize_into(&self, output: &mut String);
}

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

impl Serializable for Animation {
    fn serialize_into(&self, output: &mut String) {}
}

pub struct Frame {
    parts: Vec<Part>,
}

impl Frame {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }
}

enum Part {
    SingleColor(Color),
    Linear { start: Color, end: Color, steps: u8 },
}

pub struct Changes {
    changes: Vec<Change>,
}

impl Changes {
    pub fn new() -> Self {
        Self { changes: vec![] }
    }
}

enum Change {
    Part(Part),
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl Serializable for Color {
    fn serialize_into(&self, output: &mut String) {
        output.push(self.red.into());
        output.push(self.green.into());
        output.push(self.blue.into());
    }
}
