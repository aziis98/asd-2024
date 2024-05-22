#[derive(Debug)]
pub enum Orientation {
    Forward,
    Reverse,
}

#[derive(Debug)]
pub enum Entry {
    Header {
        version: String,
    },
    Segment {
        id: String,
        sequence: String,
    },
    Link {
        from: String,
        from_orient: Orientation,
        to: String,
        to_orient: Orientation,
    },
    Path {
        name: String,
        segments: Vec<(String, Orientation)>,
    },
    Walk {
        sample: String,

        haplotype_index: usize,
        seq_id: String,
        seq_start: usize,
        seq_end: usize,

        segments: Vec<(String, Orientation)>,
    },
}
