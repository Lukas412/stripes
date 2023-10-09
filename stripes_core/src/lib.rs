pub trait Serializable {
    fn serialize_into(&self, output: &mut String);
}

pub struct Animation {
    statt: Frame,
    changes: Vec<Changes>,
}

struct Frame {
    parts: Vec<Part>,
}

enum Part {
    SingleColor(Color),
}

struct Changes {
    changes: Vec<Change>,
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
