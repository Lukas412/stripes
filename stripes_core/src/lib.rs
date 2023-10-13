use std::fmt::{Display, Formatter};

mod changes;
mod color;
mod frame;
mod part;

pub use {
    changes::{Change, Changes},
    color::Color,
    frame::Frame,
    part::{LinearGradient, Part, SolidColor},
};

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

    pub fn with(mut self, changes: Changes) -> Self {
        self.add(changes);
        self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_display_animation() {
        let start = Frame::new()
            .with(SolidColor::new(Color::red(), 50))
            .with(LinearGradient::new(Color::red(), Color::blue(), 20))
            .with(SolidColor::new(Color::blue(), 10));
        let changes1 = Changes::new()
            .with_part(15, SolidColor::new(Color::green(), 5))
            .with_part(0, LinearGradient::new(Color::red(), Color::green(), 15))
            .with_part(20, LinearGradient::new(Color::green(), Color::red(), 5));
        let changes2 = Changes::new()
            .with_part(40, SolidColor::new(Color::green(), 2))
            .with_part(42, LinearGradient::new(Color::green(), Color::blue(), 28));
        let animation = Animation::new(start).with(changes1).with(changes2);

        let actual = format!("{}", animation);

        assert_eq!(actual, "cFF0000l50,sFF0000e0000FFl20,c0000FFl10;p15:c00FF00l5,p0:sFF0000e00FF00l15,p20:s00FF00eFF0000l5;p40:c00FF00l2,p42:s00FF00e0000FFl28")
    }
}
