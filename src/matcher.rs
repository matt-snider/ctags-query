use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use crate::location::Location;
use crate::tags::Tag;
use crate::query::Query;

pub struct Matcher {
    by_tag: HashMap<Tag, HashSet<Location>>,
}


impl Matcher {
    pub fn new(tagged_locations: Vec<(Tag, Location)>) -> Matcher {
        let mut by_tag = HashMap::new();

        for (tag, location) in tagged_locations {
            by_tag.entry(tag)
                .or_insert(HashSet::new())
                .insert(location);
        }

        Matcher { by_tag }
    }

    pub fn get_matches(&self, query: Query) -> Vec<&Location> {
        let mut matches: Vec<&Location> = self
            .execute(query)
            .iter()
            .cloned()
            .collect();
        matches.sort();

        matches
    }

    // Execute the given query.
    //
    // Note: we call `clone()` a lot in the following code, but we are cloning
    // references, not the `Location` object (it doesn't even implement `Clone`)
    fn execute(&self, query: Query) -> HashSet<&Location> {
        match query {
            Query::Present(tag) => {
                match self.by_tag.get(&tag) {
                    Some(locations) => locations
                        .iter() 
                        .collect(),
                    None => HashSet::new(),
                }
            },
            Query::Absent(tag) => {
                let where_present = self.execute(Query::Present(tag));

                self.all_locations()
                    .difference(&where_present)
                    .cloned()
                    .collect()
            },
            Query::And(q1, q2) => {
                let a = self.execute(*q1);
                let b = self.execute(*q2);

                a.intersection(&b)
                    .cloned()
                    .collect()
            },
            Query::Or(q1, q2) => {
                let a = self.execute(*q1);
                let b = self.execute(*q2);

                a.union(&b)
                    .cloned()
                    .collect()
            },
        }
    }

    fn all_locations(&self) -> HashSet<&Location> {
        self.by_tag.values()
            .flatten()
            .collect()
    }
}
