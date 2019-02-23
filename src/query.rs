use crate::tags::Tag;


#[derive(Debug, PartialEq)]
pub enum Query {
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
    Present(Tag),
    Absent(Tag),
}

