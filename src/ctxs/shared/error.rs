use std::fmt::Display;

#[derive(Debug)]
pub struct Generic {
    pub name: String,
    // pub description: Option<String>,
}

impl Generic {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Display for Generic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl std::error::Error for Generic {}

impl From<std::io::Error> for Generic {
    fn from(error: std::io::Error) -> Self {
        Self {
            name: error.to_string(),
        }
    }
}
