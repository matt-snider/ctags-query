use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use crate::location::Location;

pub type Tag = String;

/// Read the tag information from a CTags-style file
pub fn from_file<P>(path: P) -> io::Result<Vec<(Tag, Location)>>
    where P: Into<PathBuf>
{
    let file = File::open(path.into())?;
    let buf = BufReader::new(file);

    from_buf(buf)
}

fn from_buf<B>(buf: B) -> io::Result<Vec<(Tag, Location)>>
    where B: BufRead
{
    let mut tagged_locations = Vec::new();
    for line in buf.lines().skip(1) {
        let line = line?;
        if is_header(&line) {
            continue;
        }

        let loc = read_line(&line).ok_or(
            io::Error::new(io::ErrorKind::Other, format!("Failed to parse line: '{}'", line))
        )?;
        tagged_locations.push(loc);
    }

    Ok(tagged_locations)
}

// Read a line from a tags file producing a tuple (Tag, Location)
//
// A tags file has the following format based on CTags:
//
//    {tagname} {TAB} {tagfile} {TAB} {tagaddress} {COMMENT} [{extrafield}...]
//
// {tagname}: is the identifier/tag
// {tagfile}: the file containing {tagname} (absolute or relative)
// {tagaddress}: an ex command that will position the user at the tagged location
// {extrafield} (optional): each field consists of <TAB>{fieldname}:{value} and
// allows extra application-specified tag information to be specified
// {COMMENT}: `;"` which indicates the end of the standard fields
// {TAB}: a tab character
//
// See `:help tags-file-format` for more info.
fn read_line(line: &str) -> Option<(Tag, Location)> {
    let parts: Vec<&str> = line.split('\t').collect();

    if let Some(&[tag, file, address, extra]) = parts.get(0..4) {
        let tag = String::from(tag);
        let file = String::from(file);
        let address = address.replace(";\"","");
        let extra = String::from(extra);
        Some((tag, Location {
            file,
            address,
            extra,
        }))
    } else {
        None
    }
}

fn is_header(line: &str) -> bool {
    line.len() >= 6 && &line[0..6] == "!_TAG_"
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_read_line() {
        assert_eq!(
            read_line("tag1\tpath/to/file.md\t42;\"\tvimwiki:path/to/file.md\\tpath/to/file.md#header").unwrap(),
            ("tag1".into(), Location {
                file: "path/to/file.md".into(),
                address: "42".into(),
                extra: "vimwiki:path/to/file.md\tpath/to/file.md#header".into(),
            })
        );
    }

    #[test]
    fn test_is_header_vimwiki_headers() {
        assert!(is_header("!_TAG_FILE_FORMAT\t2"));
        assert!(is_header("!_TAG_FILE_SORTED\t1"));
        assert!(is_header("!_TAG_OUTPUT_MODE\tvimwiki-tags"));
        assert!(is_header("!_TAG_PROGRAM_AUTHOR\tVimwiki"));
        assert!(is_header("!_TAG_PROGRAM_NAME\tVimwiki Tags"));
        assert!(is_header("!_TAG_PROGRAM_URL\thttps://github.com/vimwiki/vimwiki"));
        assert!(is_header("!_TAG_PROGRAM_VERSION\t2.5"));
    }

    #[test]
    fn test_is_header_bad_headers() {
        assert!(!is_header("!_T"));
        assert!(!is_header("!_TA"));
        assert!(!is_header("tag1\tpath/to/file.md	42;\"\tvimwiki:path/to/file.md\\tpath/to/file.md#header"));
    }

    #[test]
    fn test_is_header_empty_header() {
        assert!(!is_header(""));
    }

    #[test]
    fn test_from_buf() {
        let buf = b"
!_TAG_FILE_FORMAT\t2
!_TAG_FILE_SORTED\t1
!_TAG_OUTPUT_MODE\tvimwiki-tags
tag1\tpath/to/file1.md\t21;\"\tvimwiki:path/to/file1.md\\tpath/to/file1.md#header1
tag1\tpath/to/file2.md\t11;\"\tvimwiki:path/to/file2.md\\tpath/to/file2.md#header1
tag2\tpath/to/file1.md\t42;\"\tvimwiki:path/to/file1.md\\tpath/to/file1.md#header2
tag2\tpath/to/file2.md\t22;\"\tvimwiki:path/to/file2.md\\tpath/to/file2.md#header2
" as &[u8];
        let locations = from_buf(buf);
        assert_eq!(locations.unwrap(), vec![
            ("tag1".into(), Location {
                file: "path/to/file1.md".into(),
                address: "21".into(),
                extra: "vimwiki:path/to/file1.md\\tpath/to/file1.md#header1".into(),
            }),
            ("tag1".into(), Location {
                file: "path/to/file2.md".into(),
                address: "11".into(),
                extra: "vimwiki:path/to/file2.md\\tpath/to/file2.md#header1".into(),
            }),
            ("tag2".into(), Location {
                file: "path/to/file1.md".into(),
                address: "42".into(),
                extra: "vimwiki:path/to/file1.md\\tpath/to/file1.md#header2".into(),
            }),
            ("tag2".into(), Location {
                file: "path/to/file2.md".into(),
                address: "22".into(),
                extra: "vimwiki:path/to/file2.md\\tpath/to/file2.md#header2".into(),
            })
        ]);
    }

    #[test]
    fn test_from_buf_bad_format_line_3() {
        let buf = b"
!_TAG_FILE_FORMAT\t2
tag1\tpath/to/file1.md\t21;\"\tvimwiki:path/to/file1.md\\tpath/to/file1.md#header1
tag1\tpath/to/file2.md\t11;\"\tvimwiki:path/to/file2.md\\tpath/to/file2.md#header1
this is a bad line
" as &[u8];
        match from_buf(buf) {
            Err(e) if e.kind() == io::ErrorKind::Other => {
                assert_eq!(e.to_string(), "Failed to parse line: 'this is a bad line'");
            },
            Err(e) => assert!(false, format!("Expected a different io::Error, got {}", e)),
            Ok(_) => assert!(false, "Expected line 3 to cause error"),
        }
    }
}

