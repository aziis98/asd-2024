use std::str::FromStr;

#[derive(Debug)]
pub enum Orientation {
    Forward,
    Reverse,
}

impl From<&str> for Orientation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Orientation::Forward,
            ">" => Orientation::Forward,
            "-" => Orientation::Reverse,
            "<" => Orientation::Reverse,
            _ => panic!("Invalid orientation: {}", s),
        }
    }
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

/// Parse a line of the source file into a Header struct
///
/// ```txt
/// H  VN:Z:1.0
/// ```
///
fn parse_header(line: &str) -> Entry {
    let columns: Vec<&str> = line.split(':').collect();

    Entry::Header {
        version: columns[2].to_string(),
    }
}

/// Parse a line of the source file into a Segment struct
///
/// ```txt
/// S  1  ACGT
/// ```
fn parse_segment(line: &str) -> Entry {
    let columns: Vec<&str> = line.split('\t').collect();

    Entry::Segment {
        id: columns[1].to_string(),
        sequence: columns[2].to_string(),
    }
}

/// Parse a line of the source file into a Link struct
///
/// ```txt
/// L  1  +  2  -  3M
/// ```
fn parse_link(line: &str) -> Entry {
    let columns: Vec<&str> = line.split('\t').collect();

    Entry::Link {
        from: columns[1].to_string(),
        from_orient: columns[2].into(),
        to: columns[3].to_string(),
        to_orient: columns[4].into(),
    }
}

/// Parse a line of the source file into a Path struct
///
/// ```txt
/// P	A	11+,12+,14+,15-,17+	*,*,*,*
/// ```
fn parse_path(line: &str) -> Entry {
    let columns: Vec<&str> = line.split('\t').collect();

    Entry::Path {
        name: columns[1].to_string(),
        segments: columns[2]
            .split(',')
            .map(|s| {
                let (name, orient) = s.split_at(s.len() - 1);
                (name.to_string(), orient.into())
            })
            .collect(),
    }
}

fn parse_path_segments(s: &str) -> Vec<(String, Orientation)> {
    let mut result = Vec::new();
    let mut rest = s;

    loop {
        println!("Rest: {}", rest);

        let r = rest;

        let (orient, r) = r.split_at(1);
        let (name, r) = r.split_at(r.find(['<', '>']).unwrap_or(r.len()));

        rest = r;
        result.push((name.to_string(), orient.into()));

        if rest.is_empty() {
            break;
        }
    }

    result
}

/// Parse a line of the source file into a Walk struct
///
/// ```txt
/// W	sample	1	A	0	5	>11>12>14>15>17
/// ```
fn parse_walk(line: &str) -> Entry {
    let columns: Vec<&str> = line.split('\t').collect();

    Entry::Walk {
        sample: columns[1].to_string(),

        haplotype_index: usize::from_str(columns[2]).unwrap(),
        seq_id: columns[3].to_string(),
        seq_start: usize::from_str(columns[4]).unwrap(),
        seq_end: usize::from_str(columns[5]).unwrap(),

        segments: parse_path_segments(columns[6]),
    }
}

pub fn parse_source(source: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        println!("Parsing: {}", line);

        let first_char = line.chars().next().unwrap();
        let entry = match first_char {
            'H' => parse_header(line),
            'S' => parse_segment(line),
            'L' => parse_link(line),
            'P' => parse_path(line),
            'W' => parse_walk(line),
            _ => {
                eprintln!("Unknown line type: {}", line);
                continue;
            }
        };
        entries.push(entry);
    }
    entries
}
