use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader};
use std::path::{PathBuf};

pub type Tag = String;

#[derive(Clone, Debug)]
pub struct Location {
    pub file: String,
    pub address: String,
    pub extra: String,
}

/// Read the tag information from a CTags-style file
pub fn from_file<P>(path: P) -> io::Result<Vec<(Tag, Location)>> 
where P: Into<PathBuf>
{
    let file = File::open(path.into())?;
    let buf = BufReader::new(file);

    let mut tagged_locations = Vec::new();
    for line in buf.lines().skip(1) {
        let line = line?;
        tagged_locations.push(read_line(&line));
    }

    Ok(tagged_locations)
}

/// Read a line from a tags file producing a tuple (Tag, Location)
///
/// A tags file has the following format based on CTags:
///
///    {tagname} {TAB} {tagfile} {TAB} {tagaddress} {COMMENT} [{extrafield}...]
///
/// {tagname}: is the identifier/tag
/// {tagfile}: the file containing {tagname} (absolute or relative)
/// {tagaddress}: an ex command that will position the user at the tagged location
/// {extrafield} (optional): each field consists of <TAB>{fieldname}:{value} and
/// allows extra application-specified tag information to be specified
/// {COMMENT}: `;"` which indicates the end of the standard fields
/// {TAB}: a tab character
///
/// See `:help tags-file-format` for more info.
fn read_line(line: &str) -> (Tag, Location) {
    let parts: Vec<&str> = line.split('\t').collect();
    let tag = String::from(parts[0]);
    let file = String::from(parts[1]);
    let address = parts[2].replace(";\"","");
    let extra = String::from(parts[3]);

    (tag, Location { 
        file,
        address,
        extra,
    })
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.address.hash(state);
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.file == other.file && self.address == other.address
    }
}

impl Eq for Location {}

