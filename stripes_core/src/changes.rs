use crate::Part;
use std::fmt::{Display, Formatter};

pub struct Changes {
    changes: Vec<Change>,
}

impl Changes {
    pub fn new() -> Self {
        Self { changes: vec![] }
    }

    pub fn add(&mut self, change: Change) {
        self.changes.push(change)
    }

    pub fn with(mut self, change: Change) -> Self {
        self.add(change);
        self
    }

    pub fn add_part(&mut self, position: u16, part: impl Into<Part>) {
        self.add(Change::new_part(position, part))
    }

    pub fn with_part(mut self, position: u16, part: impl Into<Part>) -> Self {
        self.add_part(position, part);
        self
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

pub struct Change {
    position: u16,
    content: ChangeContent,
}

impl Change {
    pub fn new_part(position: u16, part: impl Into<Part>) -> Self {
        Self {
            position,
            content: ChangeContent::Part(part.into()),
        }
    }
}

impl Display for Change {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "p{}:{}", self.position, self.content)
    }
}

enum ChangeContent {
    Part(Part),
}

impl Display for ChangeContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeContent::Part(part) => {
                write!(f, "{}", part)
            }
        }
    }
}
