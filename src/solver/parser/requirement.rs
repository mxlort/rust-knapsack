use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Requirement {
    pub id: String,
    pub amount: u32,
}

impl Requirement {
    pub fn new(input: &str) -> Self {
        let req = input.split('=').collect::<Vec<&str>>();

        Self {
            id: req[0].to_string(),
            amount: req[1].parse::<u32>().expect("Unable to parse requirement amount"),
        }
    }
}

impl fmt::Display for Requirement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.amount, self.id)
    }
}
