#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub color: String,
    pub value: String,
    pub is_face: bool,
}

impl Card {
    pub fn new(color: &str, value: &str, is_face: bool) -> Self {
        return Self {
            color: color.to_string(),
            value: value.to_string(),
            is_face,
        }
    }
}
