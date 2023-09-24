use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Product {
    pub id: String,
    pub value: u32,
    pub max: u32,
    pub solution: u32,
    pub requirements: Vec<super::Requirement>,
}

impl Product {
    pub fn new(input: &[&str]) -> Self {
        Self {
            id: input[0].trim().to_string(),
            value: input[1].trim().parse::<u32>().expect("Not a valid data"),
            max: 0,
            solution: 0,
            requirements: input[2..]
                .iter()
                .map(|input| super::Requirement::new(input.trim()))
                .collect::<Vec<super::Requirement>>(),
        }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "product {} @ {}$, req({})",
            self.id,
            self.value,
            self.requirements
                .iter()
                .rev()
                .map(|r| r.to_string())
                .reduce(|acc, r| r + ", " + &acc)
                .unwrap()
        )
    }
}
