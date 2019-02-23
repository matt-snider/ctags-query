use std::io::{self, BufRead, BufReader};
use std::path::{PathBuf};
use std::fs::File;


pub type Tag = String;


#[derive(Clone, Debug, PartialEq)]
pub struct TaggedLocation {
    pub tag: Tag, 
    pub location: Location,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    pub file: PathBuf,
    pub lineno: usize,
    pub header: String,
}


pub fn from_file<P>(path: P) -> io::Result<Vec<TaggedLocation>> 
where P: Into<PathBuf>
{
    let file = File::open(path.into())?;
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


fn read_line(line: &str) -> Option<TaggedLocation> {
    let parts: Vec<&str> = line.split('\t').collect();
    let tag = String::from(parts[0]);
    let file = PathBuf::from(parts[1]);
    let lineno: usize = parts[2].replace(";\"","").parse().unwrap();
    let header = String::from(parts[3]);

    Some(TaggedLocation {
        tag,
        location: Location { 
            file,
            lineno,
            header,
        },
    })
}
