use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use crate::tags::{Location, Tag};
use crate::query::Query;

pub struct Matcher {
    by_tag: HashMap<Tag, HashSet<Location>>,
}


impl Matcher {
    pub fn new(tagged_locations: Vec<(Tag, Location)>) -> Matcher {
        let mut by_tag = HashMap::new();

        for (tag, location) in tagged_locations {
            if !by_tag.contains_key(&tag) {
                by_tag.insert(tag.clone(), HashSet::new());
            }

            let locs = by_tag.get_mut(&tag).unwrap();
            locs.insert(location);
        }

        Matcher { by_tag }
    }

    pub fn get_matches(&self, query: Query) -> Vec<Location> {
        match query {
            Query::Present(tag) => {
                self.by_tag.get(&tag)
                    .map(|locs| locs
                         .iter()
                         .map(|x| x.clone())
                         .collect()
                    )
                    .unwrap_or_else(|| Vec::new())
            },
            _ => panic!("Woops, that query isn't supported yet :-("),
        }
    }
}

