use std::collections::{HashMap, HashSet};
use std::vec::Vec;

use crate::location::Location;
use crate::tags::Tag;
use crate::query::{PartialQuery, PartialQueryOperator, Query};

pub struct Matcher {
    locations_by_tag: HashMap<Tag, HashSet<Location>>,
    tags_by_location: HashMap<Location, HashSet<Tag>>,
}


impl Matcher {
    pub fn new(tagged_locations: Vec<(Tag, Location)>) -> Matcher {
        let mut locations_by_tag = HashMap::new();
        let mut tags_by_location = HashMap::new();

        // Build two lookup tables:
        // * all locations that each tag is found at
        // * all tags at each location
        for (tag, location) in tagged_locations {
            locations_by_tag
                .entry(tag.clone())
                .or_insert(HashSet::new())
                .insert(location.clone());

            tags_by_location
                .entry(location)
                .or_insert(HashSet::new())
                .insert(tag);
        }

        Matcher {
            locations_by_tag,
            tags_by_location,
        }
    }

    /// Execute a `Query` returning a sorted list of matched file locations.
    pub fn execute(&self, query: Query) -> Vec<&Location> {
        let mut matches: Vec<&Location> = self
            .do_execute(query)
            .iter()
            .cloned()
            .collect();
        matches.sort();
        matches
    }

    /// Return a list of potential tags based on an initial query and operation.
    pub fn suggest(&self, partial_query: PartialQuery) -> Vec<&Tag> {
        // From the matching locations, we can determine which tags can be used.
        //
        // * AND: this will be any tag used at a location in `matching_locations`
        // * OR: this will be any tag since it's independant
        //
        // NOTE: OR is currently very simple because we have no parentheses
        // (or precedence) but we'll need to give more thought later.
        use PartialQueryOperator::*;
        let matching_tags: HashSet<&Tag> = match partial_query.operator {
            AND => {
                self.do_execute(partial_query.query)
                    .iter()
                    .filter_map(|loc| self.tags_by_location.get(&loc))
                    .flatten()
                    .collect()
            },
            OR  => {
                self.all_tags()
            },
        };

        // Now account for the text fragment if it is present
        let mut matching_tags: Vec<&Tag> = match partial_query.fragment {
            Some(text) =>
                matching_tags
                .iter()
                .filter(|t| t.starts_with(&text))
                .cloned()
                .collect(),
            None => matching_tags
                .iter()
                .cloned()
                .collect(),
        };

        // Sort the tags and return
        matching_tags.sort();
        matching_tags
    }

    // Execute the given query.
    //
    // Note: we call `clone()` a lot in the following code, but we are cloning
    // references, not the `Location` object.
    fn do_execute(&self, query: Query) -> HashSet<&Location> {
        match query {
            Query::Present(tag) => {
                match self.locations_by_tag.get(&tag) {
                    Some(locations) => locations
                        .iter() 
                        .collect(),
                    None => HashSet::new(),
                }
            },
            Query::Absent(tag) => {
                let where_present = self.do_execute(Query::Present(tag));

                self.all_locations()
                    .difference(&where_present)
                    .cloned()
                    .collect()
            },
            Query::And(q1, q2) => {
                let a = self.do_execute(*q1);
                let b = self.do_execute(*q2);

                a.intersection(&b)
                    .cloned()
                    .collect()
            },
            Query::Or(q1, q2) => {
                let a = self.do_execute(*q1);
                let b = self.do_execute(*q2);

                a.union(&b)
                    .cloned()
                    .collect()
            },
        }
    }

    fn all_locations(&self) -> HashSet<&Location> {
        self.tags_by_location
            .keys()
            .collect()
    }

    fn all_tags(&self) -> HashSet<&Tag> {
        self.locations_by_tag
            .keys()
            .collect()
    }
}
