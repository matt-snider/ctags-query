use crate::tags::Tag;


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OR,
    NOT,
    AND,
    END,
    TAG(Tag),
}

