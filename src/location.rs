use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Location {
    pub file: String,
    pub address: String,
    pub extra: String,
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        self.file.cmp(&other.file)
            .then(self.address.cmp(&other.address))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.file == other.file
            && self.address == other.address
    }
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.address.hash(state);
    }
}

impl Eq for Location {}

