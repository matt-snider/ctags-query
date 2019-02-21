use std::io::{self, BufRead, BufReader};
use std::path::{PathBuf};
use std::fs::File;


#[derive(Debug, PartialEq)]
pub struct TaggedLocation {
    tag: String, 
    file: PathBuf,
    lineno: usize,
    header: String,
}


pub fn load(filepath: &str) -> io::Result<Vec<TaggedLocation>> {
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


fn read_line(line: &str) -> Option<TaggedLocation> {
    let parts: Vec<&str> = line.split('\t').collect();
    let tag = String::from(parts[0]);
    let file = PathBuf::from(parts[1]);
    let lineno: usize = parts[2].replace(";\"","").parse().unwrap();
    let header = String::from(parts[3]);

    Some(TaggedLocation {
        tag,
        file,
        lineno,
        header,
    })
}
