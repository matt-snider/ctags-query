use crate::tags::Tag;


/// A query to be executed on the tags file.
#[derive(Debug, PartialEq)]
pub enum Query {
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
    Present(Tag),
    Absent(Tag),
}


/// A incomplete query that can be used for completion.
///
/// Internally just wraps a `Query` object with an additional operator.
#[derive(Debug, PartialEq)]
pub struct PartialQuery {
    pub query: Query,
    pub operator: PartialQueryOperator,
    pub fragment: Option<String>,
}


#[derive(Debug, PartialEq)]
pub enum PartialQueryOperator {
    AND,
    OR,
}
