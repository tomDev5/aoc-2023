#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Node {
    name: String,
}

impl From<&str> for Node {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
impl Node {
    #[allow(dead_code)]
    pub fn is_full_start(&self) -> bool {
        self.name == "AAA"
    }

    #[allow(dead_code)]
    pub fn is_full_end(&self) -> bool {
        self.name == "ZZZ"
    }

    #[allow(dead_code)]
    pub fn is_part_start(&self) -> bool {
        self.name.ends_with("A")
    }

    #[allow(dead_code)]
    pub fn is_part_end(&self) -> bool {
        self.name.ends_with("Z")
    }
}
