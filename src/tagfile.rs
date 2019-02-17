use std::io::{self, BufRead, BufReader};
use std::path::{PathBuf};
use std::fs::File;

pub fn load(filepath: &str) -> io::Result<Vec<(Location, Tag)>> {
    let file = File::open(filepath)?;
    let buf = BufReader::new(file);

    let mut tagged_locations = Vec::new();
    for line in buf.lines().skip(1) {
        let line = line?;
        let tagged_location = match read_line(&line) {
            Some(tl) => tl,
            None => panic!(""),
        };

        tagged_locations.push(tagged_location);
    }

    Ok(tagged_locations)
}


fn read_line(line: &str) -> Option<(Location, Tag)> {
    let parts: Vec<&str> = line.split('\t').collect();
    let tag = parts[0];
    let filename = parts[1];
    let lineno = parts[2].replace(";\"","");
    let header = parts[3];

    Some((
        Location {
            file: PathBuf::from(filename.clone()),
            lineno: lineno.parse().unwrap(),
            header: String::from(header),
        },
        Tag { name: String::from(tag) }
    ))
}


#[derive(Debug, PartialEq)]
pub struct Tag {
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct Location {
    file: PathBuf,
    lineno: usize,
    header: String,
}

