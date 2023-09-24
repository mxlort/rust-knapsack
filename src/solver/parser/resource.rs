use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Resource {
    pub id: String,
    pub title: String,
    pub amount: i64,
}

impl Resource {
    pub fn new(input: &[&str]) -> Self {
        Self {
            id: input[0].trim().to_string(),
            title: String::from(input[1].trim()),
            amount: input[2].trim().parse::<i64>().expect("Not a valid data"),
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "resource '[{}]{}' (qty: {})",
            self.id, self.title, self.amount
        )
    }
}
