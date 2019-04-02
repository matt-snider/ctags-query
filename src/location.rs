use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Location {
    pub file: String,
    pub address: String,
    pub extra: String,
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

