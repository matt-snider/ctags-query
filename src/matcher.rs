use std::vec::Vec;
use std::collections::HashMap;
use crate::tags::{Location, TaggedLocation};
use crate::query::Query;

pub struct Matcher {
    by_tag: HashMap<String, Vec<TaggedLocation>>,
}


impl Matcher {
    pub fn new(tagged_locations: Vec<TaggedLocation>) -> Matcher {
        let mut by_tag = HashMap::new();

        for tagged_location in tagged_locations {
            if !by_tag.contains_key(&tagged_location.tag) {
                by_tag.insert(tagged_location.tag.clone(), Vec::new());
            }

            let locs = by_tag.get_mut(&tagged_location.tag).unwrap();
            locs.push(tagged_location);
        }

        Matcher { by_tag }
    }

    pub fn get_matches(&self, query: Query) -> Vec<Location> {
        match query {
            Query::Present(tag) => {
                self.by_tag.get(&tag)
                    .map(|locs| locs
                         .iter()
                         .map(|x| x.location.clone())
                         .collect()
                    )
                    .unwrap_or_else(|| Vec::new())
            },
            _ => panic!("Woops, that query isn't supported yet :-("),
        }
    }
}

