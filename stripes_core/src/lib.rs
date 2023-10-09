pub struct Animation {
    statt: Frame,
    changes: [Changes],
}

struct Frame {
    parts: Vec<Part>,
}

enum Part {}

struct Changes {
    changes: Vec<Change>,
}

enum Change {
    Part(Part),
}
