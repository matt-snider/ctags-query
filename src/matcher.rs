use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use crate::tags::{Location, Tag};
use crate::query::Query;

pub struct Matcher {
    locations: HashSet<Location>,
    by_tag: HashMap<Tag, HashSet<Location>>,
}


impl Matcher {
    pub fn new(tagged_locations: Vec<(Tag, Location)>) -> Matcher {
        let mut by_tag = HashMap::new();
        let mut locations = HashSet::new();

        // Maintain a map of locations by tag
        // and a set of all locations
        for (tag, location) in tagged_locations {
            if !by_tag.contains_key(&tag) {
                by_tag.insert(tag.clone(), HashSet::new());
            }

            let locs = by_tag.get_mut(&tag).unwrap();
            locs.insert(location.clone());
            locations.insert(location);
        }

        Matcher { 
            by_tag,
            locations,
        }
    }

    pub fn get_matches(&self, query: Query) -> Vec<Location> {
        self.do_match(query).iter().map(|x| x.clone()).collect()
    }

    fn do_match(&self, query: Query) -> HashSet<Location> {
        match query {
            Query::Present(tag) => {
                self.present(tag)
            },
            Query::Absent(tag) => {
                self.absent(tag)
            },
            Query::And(q1, q2) => {
                let a = self.do_match(*q1);
                let b = self.do_match(*q2);
                self.and(a, b)
            },
            Query::Or(q1, q2) => {
                let a = self.do_match(*q1);
                let b = self.do_match(*q2);
                self.or(a, b)
            },
        }
    }

    fn and(&self, a: HashSet<Location>, b: HashSet<Location>) -> HashSet<Location> {
        a.intersection(&b).map(|x| x.clone()).collect()
    }

    fn or(&self, a: HashSet<Location>, b: HashSet<Location>) -> HashSet<Location> {
        a.union(&b).map(|x| x.clone()).collect()
    }

    fn present(&self, tag: Tag) -> HashSet<Location> {
        match self.by_tag.get(&tag) {
            Some(locations) => locations
                .iter() 
                .map(|x| x.clone())
                .collect(),
            None => HashSet::new(),
        }
    }

    fn absent(&self, tag: Tag) -> HashSet<Location> {
        let where_present = self.present(tag);
        self.locations.difference(&where_present)
            .map(|x| x.clone())
            .collect()
    }

}

